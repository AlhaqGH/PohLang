/// Live Reload Module for PohLang Web Framework
/// 
/// Provides Flutter-style hot reload capabilities using simple HTTP polling:
/// - Client polls /__reload_check endpoint every 500ms
/// - Server tracks file modification times
/// - Sub-1s reload times
/// - No native dependencies (works on all platforms)

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use std::fs;

/// Client-side JavaScript for hot reload using polling
/// Injected into all HTML responses automatically
pub const LIVERELOAD_SCRIPT: &str = r#"
<script>
(function() {
    console.log('[LiveReload] Starting hot reload client...');
    let lastCheck = Date.now();
    let checkInterval = 500; // Check every 500ms
    let failCount = 0;
    
    function checkForUpdates() {
        fetch('/__reload_check?t=' + lastCheck)
            .then(res => res.json())
            .then(data => {
                failCount = 0;
                if (data.changed) {
                    console.log('[LiveReload] Change detected, reloading now...');
                    window.location.reload();
                } else {
                    // Schedule next check
                    setTimeout(checkForUpdates, checkInterval);
                }
            })
            .catch(err => {
                failCount++;
                console.log('[LiveReload] Check failed (attempt ' + failCount + '):', err.message);
                // Retry with exponential backoff (max 5 seconds)
                const delay = Math.min(checkInterval * Math.pow(2, failCount - 1), 5000);
                setTimeout(checkForUpdates, delay);
            });
    }
    
    // Start checking after page load
    if (document.readyState === 'complete') {
        setTimeout(checkForUpdates, checkInterval);
    } else {
        window.addEventListener('load', function() {
            setTimeout(checkForUpdates, checkInterval);
        });
    }
    
    console.log('[LiveReload] Watching for changes (polling every ' + checkInterval + 'ms)...');
})();
</script>
"#;

/// Tracks file modification times for hot reload
#[derive(Clone, Debug)]
pub struct LiveReloadTracker {
    watch_paths: Vec<PathBuf>,
    file_times: Arc<Mutex<HashMap<PathBuf, SystemTime>>>,
}

impl LiveReloadTracker {
    /// Create a new live reload tracker
    /// 
    /// # Arguments
    /// * `watch_paths` - Directories to watch for changes
    pub fn new(watch_paths: Vec<PathBuf>) -> Self {
        let tracker = Self {
            watch_paths,
            file_times: Arc::new(Mutex::new(HashMap::new())),
        };
        
        // Initialize file times
        tracker.scan_files();
        
        println!("[LiveReload] Tracker initialized");
        println!("[LiveReload] Watching: {:?}", tracker.watch_paths);
        println!("[LiveReload] Polling endpoint: GET /__reload_check");
        
        tracker
    }
    
    /// Scan all watched directories and update file times
    fn scan_files(&self) {
        let mut times = self.file_times.lock().unwrap();
        times.clear();
        
        for watch_path in &self.watch_paths {
            if let Ok(entries) = Self::walk_dir(watch_path) {
                for entry in entries {
                    if Self::is_relevant_file(&entry) {
                        if let Ok(metadata) = fs::metadata(&entry) {
                            if let Ok(modified) = metadata.modified() {
                                times.insert(entry, modified);
                            }
                        }
                    }
                }
            }
        }
        
        println!("[LiveReload] Tracking {} files", times.len());
    }
    
    /// Check if any watched files have changed
    /// Returns true if changes detected
    pub fn check_for_changes(&self) -> bool {
        let old_times = self.file_times.lock().unwrap().clone();
        let mut new_times = HashMap::new();
        let mut changed = false;
        
        // Scan all files
        for watch_path in &self.watch_paths {
            if let Ok(entries) = Self::walk_dir(watch_path) {
                for entry in entries {
                    if Self::is_relevant_file(&entry) {
                        if let Ok(metadata) = fs::metadata(&entry) {
                            if let Ok(modified) = metadata.modified() {
                                new_times.insert(entry.clone(), modified);
                                
                                // Check if file is new or modified
                                if let Some(old_time) = old_times.get(&entry) {
                                    if modified > *old_time {
                                        println!("[LiveReload] Modified: {:?}", entry);
                                        changed = true;
                                    }
                                } else {
                                    println!("[LiveReload] New file: {:?}", entry);
                                    changed = true;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Check for deleted files
        for old_path in old_times.keys() {
            if !new_times.contains_key(old_path) {
                println!("[LiveReload] Deleted: {:?}", old_path);
                changed = true;
            }
        }
        
        // Update stored times
        if changed {
            *self.file_times.lock().unwrap() = new_times;
        }
        
        changed
    }
    
    /// Recursively walk directory and collect file paths
    fn walk_dir(path: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
        let mut files = Vec::new();
        
        if path.is_file() {
            files.push(path.to_path_buf());
            return Ok(files);
        }
        
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                
                // Skip hidden files and directories
                if let Some(name) = path.file_name() {
                    let name_str = name.to_string_lossy();
                    if name_str.starts_with('.') || name_str == "target" || name_str == "node_modules" {
                        continue;
                    }
                }
                
                if path.is_file() {
                    files.push(path);
                } else if path.is_dir() {
                    if let Ok(sub_files) = Self::walk_dir(&path) {
                        files.extend(sub_files);
                    }
                }
            }
        }
        
        Ok(files)
    }
    
    /// Check if a file is relevant for hot reload
    /// Returns true for .poh files and common web assets
    fn is_relevant_file(path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            matches!(
                ext_str.as_str(),
                "poh" | "html" | "css" | "js" | "json" | "svg" | "png" | "jpg" | "jpeg" | "gif" | "ico" | "woff" | "woff2" | "ttf"
            )
        } else {
            false
        }
    }
}

/// Generate JSON response for reload check endpoint
/// Returns {"changed": true/false}
pub fn check_reload_response(tracker: &LiveReloadTracker) -> String {
    let changed = tracker.check_for_changes();
    serde_json::json!({"changed": changed}).to_string()
}

/// Inject live reload script into HTML response
/// Adds the polling client before </body> tag
pub fn inject_livereload_script(html: String) -> String {
    // Don't inject if already present
    if html.contains("__reload_check") {
        return html;
    }
    
    if html.contains("</body>") {
        html.replace("</body>", &format!("{}</body>", LIVERELOAD_SCRIPT))
    } else if html.contains("</html>") {
        html.replace("</html>", &format!("{}</html>", LIVERELOAD_SCRIPT))
    } else {
        // If no body or html tag, append at end
        format!("{}{}", html, LIVERELOAD_SCRIPT)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_inject_script() {
        let html = "<html><body><h1>Test</h1></body></html>".to_string();
        let result = inject_livereload_script(html);
        assert!(result.contains("<script>"));
        assert!(result.contains("__reload_check"));
        assert!(result.contains("</body>"));
    }
    
    #[test]
    fn test_no_double_inject() {
        let html = "<html><body><script>__reload_check</script></body></html>".to_string();
        let result = inject_livereload_script(html);
        // Should not add another script
        assert_eq!(result.matches("__reload_check").count(), 1);
    }
    
    #[test]
    fn test_is_relevant_file() {
        assert!(LiveReloadTracker::is_relevant_file(Path::new("test.poh")));
        assert!(LiveReloadTracker::is_relevant_file(Path::new("style.css")));
        assert!(LiveReloadTracker::is_relevant_file(Path::new("script.js")));
        assert!(LiveReloadTracker::is_relevant_file(Path::new("index.html")));
        assert!(!LiveReloadTracker::is_relevant_file(Path::new("test.txt")));
        assert!(!LiveReloadTracker::is_relevant_file(Path::new("readme.md")));
    }
}
