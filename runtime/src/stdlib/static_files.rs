/// Static file serving for PohLang web framework
/// Handles serving files with proper MIME types, caching, and security

use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use super::http::HttpResponse;

/// MIME type mapping
pub struct MimeTypes {
    mappings: HashMap<String, String>,
}

impl MimeTypes {
    pub fn new() -> Self {
        let mut mappings = HashMap::new();
        
        // Text
        mappings.insert("txt".to_string(), "text/plain".to_string());
        mappings.insert("html".to_string(), "text/html".to_string());
        mappings.insert("htm".to_string(), "text/html".to_string());
        mappings.insert("css".to_string(), "text/css".to_string());
        mappings.insert("js".to_string(), "application/javascript".to_string());
        mappings.insert("json".to_string(), "application/json".to_string());
        mappings.insert("xml".to_string(), "application/xml".to_string());
        
        // Images
        mappings.insert("jpg".to_string(), "image/jpeg".to_string());
        mappings.insert("jpeg".to_string(), "image/jpeg".to_string());
        mappings.insert("png".to_string(), "image/png".to_string());
        mappings.insert("gif".to_string(), "image/gif".to_string());
        mappings.insert("svg".to_string(), "image/svg+xml".to_string());
        mappings.insert("ico".to_string(), "image/x-icon".to_string());
        mappings.insert("webp".to_string(), "image/webp".to_string());
        
        // Fonts
        mappings.insert("woff".to_string(), "font/woff".to_string());
        mappings.insert("woff2".to_string(), "font/woff2".to_string());
        mappings.insert("ttf".to_string(), "font/ttf".to_string());
        mappings.insert("otf".to_string(), "font/otf".to_string());
        
        // Documents
        mappings.insert("pdf".to_string(), "application/pdf".to_string());
        mappings.insert("doc".to_string(), "application/msword".to_string());
        mappings.insert("docx".to_string(), "application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_string());
        mappings.insert("xls".to_string(), "application/vnd.ms-excel".to_string());
        mappings.insert("xlsx".to_string(), "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string());
        
        // Audio/Video
        mappings.insert("mp3".to_string(), "audio/mpeg".to_string());
        mappings.insert("ogg".to_string(), "audio/ogg".to_string());
        mappings.insert("wav".to_string(), "audio/wav".to_string());
        mappings.insert("mp4".to_string(), "video/mp4".to_string());
        mappings.insert("webm".to_string(), "video/webm".to_string());
        
        // Archives
        mappings.insert("zip".to_string(), "application/zip".to_string());
        mappings.insert("tar".to_string(), "application/x-tar".to_string());
        mappings.insert("gz".to_string(), "application/gzip".to_string());
        
        Self { mappings }
    }
    
    pub fn get_mime_type(&self, path: &Path) -> String {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            self.mappings
                .get(&ext.to_lowercase())
                .cloned()
                .unwrap_or_else(|| "application/octet-stream".to_string())
        } else {
            "application/octet-stream".to_string()
        }
    }
}

impl Default for MimeTypes {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for static file serving
#[derive(Debug, Clone)]
pub struct StaticFileConfig {
    pub root_dir: PathBuf,
    pub index_files: Vec<String>,
    pub enable_directory_listing: bool,
    pub enable_cache: bool,
    pub cache_max_age: u32, // in seconds
    pub enable_gzip: bool,
    pub security_check: bool, // Prevent directory traversal
}

impl Default for StaticFileConfig {
    fn default() -> Self {
        Self {
            root_dir: PathBuf::from("./public"),
            index_files: vec!["index.html".to_string(), "index.htm".to_string()],
            enable_directory_listing: false,
            enable_cache: true,
            cache_max_age: 3600, // 1 hour
            enable_gzip: false,
            security_check: true,
        }
    }
}

/// Static file server
pub struct StaticFileServer {
    config: StaticFileConfig,
    mime_types: MimeTypes,
}

impl StaticFileServer {
    pub fn new(config: StaticFileConfig) -> Self {
        Self {
            config,
            mime_types: MimeTypes::new(),
        }
    }
    
    /// Serves a file from the configured root directory
    pub fn serve(&self, path: &str) -> Result<HttpResponse> {
        // Security: prevent directory traversal
        if self.config.security_check && path.contains("..") {
            return Ok(HttpResponse {
                status: 403,
                headers: HashMap::new(),
                body: "403 Forbidden: Directory traversal not allowed".to_string(),
            });
        }
        
        // Construct full path
        let file_path = self.config.root_dir.join(path.trim_start_matches('/'));
        
        // Check if path is a directory
        if file_path.is_dir() {
            return self.serve_directory(&file_path);
        }
        
        // Serve the file
        self.serve_file(&file_path)
    }
    
    /// Serves a specific file
    fn serve_file(&self, path: &Path) -> Result<HttpResponse> {
        if !path.exists() {
            return Ok(HttpResponse {
                status: 404,
                headers: HashMap::new(),
                body: "404 Not Found".to_string(),
            });
        }
        
        // Read file
        let mut file = File::open(path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        
        // Get MIME type
        let content_type = self.mime_types.get_mime_type(path);
        
        // Check if text file before moving content_type
        let is_text = self.is_text_file(&content_type);
        
        // Build headers
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), content_type);
        
        // Add caching headers
        if self.config.enable_cache {
            headers.insert(
                "Cache-Control".to_string(),
                format!("public, max-age={}", self.config.cache_max_age),
            );
            
            // Add ETag based on file metadata
            if let Ok(metadata) = fs::metadata(path) {
                if let Ok(modified) = metadata.modified() {
                    if let Ok(duration) = modified.duration_since(SystemTime::UNIX_EPOCH) {
                        let etag = format!("\"{}\"", duration.as_secs());
                        headers.insert("ETag".to_string(), etag);
                    }
                }
            }
        }
        
        // Convert to string (or base64 for binary)
        let body = if is_text {
            String::from_utf8_lossy(&contents).to_string()
        } else {
            // For binary files, we'd ideally return raw bytes
            // For now, return a message
            format!("Binary file: {} bytes", contents.len())
        };
        
        Ok(HttpResponse {
            status: 200,
            headers,
            body,
        })
    }
    
    /// Serves a directory (index file or listing)
    fn serve_directory(&self, path: &Path) -> Result<HttpResponse> {
        // Try to serve index files
        for index in &self.config.index_files {
            let index_path = path.join(index);
            if index_path.exists() {
                return self.serve_file(&index_path);
            }
        }
        
        // Directory listing if enabled
        if self.config.enable_directory_listing {
            return self.generate_directory_listing(path);
        }
        
        Ok(HttpResponse {
            status: 403,
            headers: HashMap::new(),
            body: "403 Forbidden: Directory listing not allowed".to_string(),
        })
    }
    
    /// Generates HTML directory listing
    fn generate_directory_listing(&self, path: &Path) -> Result<HttpResponse> {
        let entries = fs::read_dir(path)?;
        
        let mut html = String::from("<html><head><title>Directory Listing</title>");
        html.push_str("<style>body{font-family:monospace;padding:20px;}a{display:block;padding:5px;}</style>");
        html.push_str("</head><body><h1>Directory Listing</h1><ul>");
        
        for entry in entries {
            if let Ok(entry) = entry {
                let name = entry.file_name().to_string_lossy().to_string();
                let is_dir = entry.path().is_dir();
                let display_name = if is_dir {
                    format!("{}/", name)
                } else {
                    name.clone()
                };
                html.push_str(&format!(
                    "<li><a href=\"{}\">{}</a></li>",
                    name, display_name
                ));
            }
        }
        
        html.push_str("</ul></body></html>");
        
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/html".to_string());
        
        Ok(HttpResponse {
            status: 200,
            headers,
            body: html,
        })
    }
    
    /// Checks if content type is text
    fn is_text_file(&self, content_type: &str) -> bool {
        content_type.starts_with("text/")
            || content_type.contains("json")
            || content_type.contains("xml")
            || content_type.contains("javascript")
    }
}

/// Helper function to serve static files from a directory
pub fn serve_static(root_dir: &str, path: &str) -> Result<HttpResponse> {
    let config = StaticFileConfig {
        root_dir: PathBuf::from(root_dir),
        ..Default::default()
    };
    
    let server = StaticFileServer::new(config);
    server.serve(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mime_types() {
        let mime = MimeTypes::new();
        
        assert_eq!(mime.get_mime_type(Path::new("test.html")), "text/html");
        assert_eq!(mime.get_mime_type(Path::new("test.css")), "text/css");
        assert_eq!(mime.get_mime_type(Path::new("test.js")), "application/javascript");
        assert_eq!(mime.get_mime_type(Path::new("test.jpg")), "image/jpeg");
        assert_eq!(mime.get_mime_type(Path::new("test.png")), "image/png");
        assert_eq!(mime.get_mime_type(Path::new("test.pdf")), "application/pdf");
    }
    
    #[test]
    fn test_security_check() {
        let config = StaticFileConfig::default();
        let server = StaticFileServer::new(config);
        
        // Should block directory traversal
        let response = server.serve("../../../etc/passwd").unwrap();
        assert_eq!(response.status, 403);
    }
}
