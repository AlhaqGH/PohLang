/// File upload handling for PohLang web framework
/// Supports multipart/form-data parsing and file storage

use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

/// Represents an uploaded file
#[derive(Debug, Clone)]
pub struct UploadedFile {
    pub filename: String,
    pub content_type: String,
    pub size: usize,
    pub data: Vec<u8>,
    pub temp_path: Option<PathBuf>,
}

impl UploadedFile {
    /// Saves the uploaded file to a specified path
    pub fn save_to(&self, path: &Path) -> Result<()> {
        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let mut file = File::create(path)?;
        file.write_all(&self.data)?;
        Ok(())
    }
    
    /// Gets file extension
    pub fn extension(&self) -> Option<&str> {
        Path::new(&self.filename)
            .extension()
            .and_then(|ext| ext.to_str())
    }
    
    /// Validates file size (in bytes)
    pub fn validate_size(&self, max_size: usize) -> Result<()> {
        if self.size > max_size {
            return Err(anyhow!(
                "File size {} exceeds maximum allowed size {}",
                self.size,
                max_size
            ));
        }
        Ok(())
    }
    
    /// Validates file type
    pub fn validate_type(&self, allowed_types: &[&str]) -> Result<()> {
        if !allowed_types.contains(&self.content_type.as_str()) {
            return Err(anyhow!(
                "File type '{}' not allowed. Allowed types: {:?}",
                self.content_type,
                allowed_types
            ));
        }
        Ok(())
    }
    
    /// Validates file extension
    pub fn validate_extension(&self, allowed_extensions: &[&str]) -> Result<()> {
        let ext = self.extension().ok_or_else(|| anyhow!("File has no extension"))?;
        
        if !allowed_extensions.contains(&ext) {
            return Err(anyhow!(
                "File extension '{}' not allowed. Allowed extensions: {:?}",
                ext,
                allowed_extensions
            ));
        }
        Ok(())
    }
}

/// File upload configuration
#[derive(Debug, Clone)]
pub struct UploadConfig {
    pub max_file_size: usize,           // Maximum file size in bytes
    pub max_total_size: usize,          // Maximum total upload size
    pub allowed_extensions: Vec<String>, // Allowed file extensions
    pub allowed_types: Vec<String>,     // Allowed MIME types
    pub upload_dir: PathBuf,            // Directory to store uploads
    pub temp_dir: PathBuf,              // Temporary directory for uploads
}

impl Default for UploadConfig {
    fn default() -> Self {
        Self {
            max_file_size: 10 * 1024 * 1024,        // 10MB
            max_total_size: 100 * 1024 * 1024,      // 100MB
            allowed_extensions: vec![
                "jpg".to_string(),
                "jpeg".to_string(),
                "png".to_string(),
                "gif".to_string(),
                "pdf".to_string(),
                "txt".to_string(),
                "doc".to_string(),
                "docx".to_string(),
            ],
            allowed_types: vec![
                "image/jpeg".to_string(),
                "image/png".to_string(),
                "image/gif".to_string(),
                "application/pdf".to_string(),
                "text/plain".to_string(),
            ],
            upload_dir: PathBuf::from("./uploads"),
            temp_dir: PathBuf::from("./temp"),
        }
    }
}

impl UploadConfig {
    /// Validates an uploaded file against config
    pub fn validate(&self, file: &UploadedFile) -> Result<()> {
        file.validate_size(self.max_file_size)?;
        
        if !self.allowed_extensions.is_empty() {
            if let Some(ext) = file.extension() {
                if !self.allowed_extensions.contains(&ext.to_lowercase()) {
                    return Err(anyhow!(
                        "File extension '{}' not allowed",
                        ext
                    ));
                }
            } else {
                return Err(anyhow!("File has no extension"));
            }
        }
        
        if !self.allowed_types.is_empty() {
            if !self.allowed_types.contains(&file.content_type) {
                return Err(anyhow!(
                    "File type '{}' not allowed",
                    file.content_type
                ));
            }
        }
        
        Ok(())
    }
}

/// Parses multipart/form-data from request body
pub fn parse_multipart(
    body: &[u8],
    boundary: &str,
    config: &UploadConfig,
) -> Result<(HashMap<String, String>, HashMap<String, UploadedFile>)> {
    let mut form_data = HashMap::new();
    let mut files = HashMap::new();
    
    let boundary_str = format!("--{}", boundary);
    let body_str = String::from_utf8_lossy(body);
    
    let parts: Vec<&str> = body_str.split(&boundary_str).collect();
    
    for part in parts {
        if part.trim().is_empty() || part.starts_with("--") {
            continue;
        }
        
        // Parse part headers and body
        let mut lines = part.lines();
        let mut headers = HashMap::new();
        
        // Read headers
        for line in lines.by_ref() {
            let line = line.trim();
            if line.is_empty() {
                break;
            }
            
            if let Some((key, value)) = line.split_once(':') {
                headers.insert(key.trim().to_lowercase(), value.trim().to_string());
            }
        }
        
        // Get Content-Disposition header
        if let Some(disposition) = headers.get("content-disposition") {
            let name = extract_param(disposition, "name");
            
            if let Some(filename) = extract_param(disposition, "filename") {
                // This is a file upload
                let content_type = headers
                    .get("content-type")
                    .cloned()
                    .unwrap_or_else(|| "application/octet-stream".to_string());
                
                // Collect remaining lines as file data
                let data: Vec<u8> = lines
                    .collect::<Vec<&str>>()
                    .join("\n")
                    .trim()
                    .as_bytes()
                    .to_vec();
                
                let file = UploadedFile {
                    filename: filename.clone(),
                    content_type,
                    size: data.len(),
                    data,
                    temp_path: None,
                };
                
                // Validate file
                config.validate(&file)?;
                
                if let Some(name) = name {
                    files.insert(name, file);
                }
            } else if let Some(name) = name {
                // This is a regular form field
                let value = lines.collect::<Vec<&str>>().join("\n").trim().to_string();
                form_data.insert(name, value);
            }
        }
    }
    
    Ok((form_data, files))
}

/// Extracts a parameter value from Content-Disposition header
fn extract_param(header: &str, param: &str) -> Option<String> {
    let search = format!("{}=", param);
    if let Some(start) = header.find(&search) {
        let value_start = start + search.len();
        let rest = &header[value_start..];
        
        // Handle quoted values
        if rest.starts_with('"') {
            if let Some(end) = rest[1..].find('"') {
                return Some(rest[1..=end].to_string());
            }
        } else {
            // Handle unquoted values (until semicolon or end)
            let end = rest.find(';').unwrap_or(rest.len());
            return Some(rest[..end].trim().to_string());
        }
    }
    None
}

/// Parses application/x-www-form-urlencoded data
pub fn parse_form_urlencoded(body: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    
    for pair in body.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            let key = urlencoding::decode(key).unwrap_or_default().to_string();
            let value = urlencoding::decode(value).unwrap_or_default().to_string();
            result.insert(key, value);
        }
    }
    
    result
}

/// Generates a unique filename to avoid collisions
pub fn generate_unique_filename(original: &str) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    
    let path = Path::new(original);
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    
    if ext.is_empty() {
        format!("{}_{}", stem, timestamp)
    } else {
        format!("{}_{}.{}", stem, timestamp, ext)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_unique_filename() {
        let filename = generate_unique_filename("test.jpg");
        assert!(filename.starts_with("test_"));
        assert!(filename.ends_with(".jpg"));
    }
    
    #[test]
    fn test_parse_form_urlencoded() {
        let data = "name=John+Doe&age=30&city=New%20York";
        let result = parse_form_urlencoded(data);
        
        assert_eq!(result.get("name"), Some(&"John Doe".to_string()));
        assert_eq!(result.get("age"), Some(&"30".to_string()));
        assert_eq!(result.get("city"), Some(&"New York".to_string()));
    }
    
    #[test]
    fn test_file_validation() {
        let file = UploadedFile {
            filename: "test.jpg".to_string(),
            content_type: "image/jpeg".to_string(),
            size: 1024,
            data: vec![0; 1024],
            temp_path: None,
        };
        
        // Should pass
        assert!(file.validate_size(2048).is_ok());
        assert!(file.validate_type(&["image/jpeg", "image/png"]).is_ok());
        assert!(file.validate_extension(&["jpg", "png"]).is_ok());
        
        // Should fail
        assert!(file.validate_size(512).is_err());
        assert!(file.validate_type(&["application/pdf"]).is_err());
        assert!(file.validate_extension(&["png", "gif"]).is_err());
    }
}
