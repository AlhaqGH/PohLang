use anyhow::{anyhow, Result};
use serde_json::{json, Value as JsonValue};
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use tiny_http::{Header, Request, Response, Server, StatusCode};

use super::router::{Router, RoutePattern};

/// Represents an HTTP request for PohLang
#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub query: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub path_params: HashMap<String, String>, // Added for path parameters
}

/// Represents an HTTP response for PohLang
#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Default for HttpResponse {
    fn default() -> Self {
        Self {
            status: 200,
            headers: HashMap::new(),
            body: String::new(),
        }
    }
}

/// Route handler type
pub type RouteHandler = Arc<dyn Fn(HttpRequest) -> Result<HttpResponse> + Send + Sync>;

/// Represents a route in the web server
#[derive(Clone)]
pub struct Route {
    pub path: String,
    pub method: String,
    pub handler: RouteHandler,
}

impl std::fmt::Debug for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Route")
            .field("path", &self.path)
            .field("method", &self.method)
            .field("handler", &"<function>")
            .finish()
    }
}

/// Web server instance
#[derive(Debug)]
pub struct WebServer {
    port: u16,
    routes: Arc<Mutex<Vec<Route>>>,
    router: Arc<Mutex<Router>>, // Added for advanced routing
}

impl WebServer {
    /// Creates a new web server
    pub fn new(port: u16) -> Self {
        Self {
            port,
            routes: Arc::new(Mutex::new(Vec::new())),
            router: Arc::new(Mutex::new(Router::new())),
        }
    }

    /// Adds a route to the server
    pub fn add_route(&mut self, path: String, method: String, handler: RouteHandler) {
        let route = Route {
            path,
            method: method.to_uppercase(),
            handler,
        };

        if let Ok(mut routes) = self.routes.lock() {
            routes.push(route);
        }
    }

    /// Add a route directly (for internal use, can work with Arc)
    pub fn add_route_direct(&self, route: Route) {
        if let Ok(mut routes) = self.routes.lock() {
            routes.push(route);
        }
    }

    /// Starts the web server (blocking)
    pub fn start(&self) -> Result<()> {
        let addr = format!("127.0.0.1:{}", self.port);
        let server = Server::http(&addr).map_err(|e| anyhow!("Failed to start server: {}", e))?;

        println!("🚀 Server listening on http://{}", addr);
        eprintln!("[DEBUG] Entering request loop...");

        let routes = self.routes.clone();
        let mut request_count = 0;

        loop {
            eprintln!("[DEBUG] Waiting for request #{}...", request_count + 1);
            let request = match server.recv() {
                Ok(req) => {
                    request_count += 1;
                    eprintln!("[DEBUG] Received request #{}", request_count);
                    req
                }
                Err(e) => {
                    eprintln!("[ERROR] Error receiving request: {}", e);
                    continue;
                }
            };

            let routes = routes.clone();
            thread::spawn(move || {
                if let Err(e) = handle_request(request, &routes) {
                    eprintln!("Error handling request: {}", e);
                }
            });
        }
    }
}

/// Start a server from an Arc without holding the mutex lock
/// This is the proper way to start a server that's shared via Arc<Mutex<WebServer>>
pub fn start_server_from_arc(server_arc: Arc<Mutex<WebServer>>) -> Result<()> {
    // Extract what we need without holding the lock
    let (addr, routes) = {
        let server = server_arc.lock().unwrap();
        let addr = format!("0.0.0.0:{}", server.port);
        let routes = server.routes.clone();
        (addr, routes)
    }; // Lock released here

    // Now start the server without holding any locks
    let http_server = Server::http(&addr).map_err(|e| anyhow!("Failed to start server: {}", e))?;

    eprintln!("[DEBUG] Server::http() succeeded, server object created");
    eprintln!("[DEBUG] Server address: {}", addr);
    eprintln!(
        "[DEBUG] Server type: {:?}",
        std::any::type_name_of_val(&http_server)
    );

    println!("🚀 Server listening on http://{}", addr);
    eprintln!("[DEBUG] Entering request loop...");

    let mut request_count = 0;

    loop {
        eprintln!(
            "[DEBUG] Loop iteration {}, about to call server.recv()...",
            request_count + 1
        );
        eprintln!("[DEBUG] Right before recv() call...");
        let request = match http_server.recv() {
            Ok(req) => {
                request_count += 1;
                eprintln!("[DEBUG] Received request #{}", request_count);
                req
            }
            Err(e) => {
                eprintln!("[ERROR] Error receiving request: {}", e);
                eprintln!("[ERROR] Error details: {:?}", e);
                return Err(anyhow!("Server recv() failed: {}", e));
            }
        };

        let routes_clone = routes.clone();
        thread::spawn(move || {
            if let Err(e) = handle_request(request, &routes_clone) {
                eprintln!("Error handling request: {}", e);
            }
        });
    }
}

/// Handles an incoming HTTP request
pub fn handle_request_external(request: Request, routes: &Arc<Mutex<Vec<Route>>>) -> Result<()> {
    handle_request(request, routes)
}

/// Handles an incoming HTTP request (internal)
fn handle_request(mut request: Request, routes: &Arc<Mutex<Vec<Route>>>) -> Result<()> {
    // Extract request information
    let method = request.method().to_string();
    let path = request.url().to_string();

    // Parse query string
    let query = parse_query_string(&path);

    // Extract headers
    let mut headers = HashMap::new();
    for header in request.headers() {
        headers.insert(
            header.field.as_str().to_string(),
            header.value.as_str().to_string(),
        );
    }

    // Read body
    let mut body_string = String::new();
    if let Err(e) = request.as_reader().read_to_string(&mut body_string) {
        eprintln!("Error reading body: {}", e);
    }

    // Create PohLang request object
    let poh_request = HttpRequest {
        method: method.clone(),
        path: path.split('?').next().unwrap_or(&path).to_string(),
        query,
        headers,
        body: body_string,
        path_params: HashMap::new(), // Will be filled by router if matched
    };

    // Find matching route - try exact match first, then pattern matching
    let routes_guard = routes.lock().unwrap();
    
    // Try exact match first (for backwards compatibility)
    let exact_match = routes_guard
        .iter()
        .find(|r| r.path == poh_request.path && r.method == method);

    let response = if let Some(route) = exact_match {
        // Exact route match
        match (route.handler)(poh_request.clone()) {
            Ok(resp) => resp,
            Err(e) => error_response(500, format!("Handler error: {}", e)),
        }
    } else {
        // Try pattern matching for path parameters
        let mut matched = false;
        let mut final_response = error_response(404, "Not Found".to_string());
        
        for route in routes_guard.iter() {
            // Check if this route has path parameters (contains ':')
            if route.path.contains(':') && route.method == method {
                if let Ok(pattern) = RoutePattern::new(&route.path) {
                    if let Some(params) = pattern.matches(&poh_request.path) {
                        // Found a match! Create request with path params
                        let mut req_with_params = poh_request.clone();
                        req_with_params.path_params = params;
                        
                        final_response = match (route.handler)(req_with_params) {
                            Ok(resp) => resp,
                            Err(e) => error_response(500, format!("Handler error: {}", e)),
                        };
                        matched = true;
                        break;
                    }
                }
            }
        }
        
        final_response
    };

    // Build tiny_http response
    let status_code = StatusCode::from(response.status);
    let mut tiny_response = Response::from_string(response.body);

    // Add headers
    for (key, value) in response.headers {
        if let Ok(header) = Header::from_bytes(key.as_bytes(), value.as_bytes()) {
            tiny_response = tiny_response.with_header(header);
        }
    }

    tiny_response = tiny_response.with_status_code(status_code);

    // Send response
    request
        .respond(tiny_response)
        .map_err(|e| anyhow!("Failed to send response: {}", e))?;

    Ok(())
}

/// Parses query string from URL
fn parse_query_string(url: &str) -> HashMap<String, String> {
    let mut query = HashMap::new();

    if let Some(query_str) = url.split('?').nth(1) {
        for pair in query_str.split('&') {
            if let Some((key, value)) = pair.split_once('=') {
                query.insert(key.to_string(), value.to_string());
            }
        }
    }

    query
}

/// Helper function to create an HTML response
pub fn html_response(html: String) -> HttpResponse {
    let mut headers = HashMap::new();
    headers.insert(
        "Content-Type".to_string(),
        "text/html; charset=utf-8".to_string(),
    );

    // Auto-inject livereload script if HTML contains </body>
    let html_with_reload = if html.contains("</body>") {
        html.replace("</body>", &format!("{}</body>", LIVERELOAD_SCRIPT))
    } else {
        html
    };

    HttpResponse {
        status: 200,
        headers,
        body: html_with_reload,
    }
}

const LIVERELOAD_SCRIPT: &str = r#"
<script>
(function() {
    console.log('[LiveReload] Monitoring for changes...');
    let lastCheck = Date.now();
    
    function checkForUpdates() {
        fetch('/__reload_check')
            .then(res => res.json())
            .then(data => {
                if (data.changed) {
                    console.log('[LiveReload] Changes detected! Reloading...');
                    window.location.reload();
                }
            })
            .catch(err => {
                console.error('[LiveReload] Error checking for updates:', err);
            });
    }
    
    // Check every 500ms (2x per second)
    setInterval(checkForUpdates, 500);
})();
</script>
"#;

/// Helper function to create a JSON response
pub fn json_response(json: JsonValue) -> HttpResponse {
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    HttpResponse {
        status: 200,
        headers,
        body: serde_json::to_string(&json).unwrap_or_else(|_| "{}".to_string()),
    }
}

/// Helper function to create a JSON response with custom status
pub fn json_response_with_status(json: JsonValue, status: u16) -> HttpResponse {
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    HttpResponse {
        status,
        headers,
        body: serde_json::to_string(&json).unwrap_or_else(|_| "{}".to_string()),
    }
}

/// Helper function to create an error response
pub fn error_response(status: u16, message: String) -> HttpResponse {
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    let error_json = json!({
        "error": message,
        "status": status
    });

    HttpResponse {
        status,
        headers,
        body: serde_json::to_string(&error_json).unwrap(),
    }
}

/// Converts HttpRequest to JSON for PohLang
pub fn request_to_json(request: &HttpRequest) -> JsonValue {
    json!({
        "method": request.method,
        "path": request.path,
        "query": request.query,
        "headers": request.headers,
        "body": request.body
    })
}

/// Converts JSON to HttpResponse for PohLang
pub fn json_to_response(json: &JsonValue) -> Result<HttpResponse> {
    let status = json["status"].as_u64().unwrap_or(200) as u16;

    let headers: HashMap<String, String> = json["headers"]
        .as_object()
        .map(|obj| {
            obj.iter()
                .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
                .collect()
        })
        .unwrap_or_default();

    let body = json["body"].as_str().unwrap_or("").to_string();

    Ok(HttpResponse {
        status,
        headers,
        body,
    })
}

/// Serve static files from a directory
/// Returns None if file not found, otherwise returns HttpResponse
pub fn serve_static_file(root_dir: &Path, request_path: &str) -> Option<HttpResponse> {
    // Security: prevent directory traversal
    let safe_path = request_path.trim_start_matches('/');
    if safe_path.contains("..") {
        return Some(error_response(403, "Forbidden".to_string()));
    }

    let file_path = root_dir.join(safe_path);

    // If path is a directory, try index.html
    let file_path = if file_path.is_dir() {
        file_path.join("index.html")
    } else {
        file_path
    };

    // Check if file exists
    if !file_path.exists() || !file_path.is_file() {
        return None;
    }

    // Read file
    match fs::read(&file_path) {
        Ok(content) => {
            let mime_type = guess_mime_type(&file_path);
            let mut headers = HashMap::new();
            headers.insert("Content-Type".to_string(), mime_type);

            // Add cache control for static assets
            if is_cacheable_asset(&file_path) {
                headers.insert(
                    "Cache-Control".to_string(),
                    "public, max-age=3600".to_string(),
                );
            }

            Some(HttpResponse {
                status: 200,
                headers,
                body: String::from_utf8_lossy(&content).to_string(),
            })
        }
        Err(_) => Some(error_response(500, "Failed to read file".to_string())),
    }
}

/// Guess MIME type from file extension
fn guess_mime_type(path: &Path) -> String {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") | Some("htm") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "application/javascript; charset=utf-8",
        Some("json") => "application/json",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",
        Some("woff") => "font/woff",
        Some("woff2") => "font/woff2",
        Some("ttf") => "font/ttf",
        Some("txt") => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
    .to_string()
}

/// Check if file should be cached
fn is_cacheable_asset(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|e| e.to_str()),
        Some("css")
            | Some("js")
            | Some("png")
            | Some("jpg")
            | Some("jpeg")
            | Some("gif")
            | Some("svg")
            | Some("woff")
            | Some("woff2")
            | Some("ttf")
            | Some("ico")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_server() {
        let server = WebServer::new(3000);
        assert_eq!(server.port, 3000);
    }

    #[test]
    fn test_html_response() {
        let response = html_response("<h1>Hello</h1>".to_string());
        assert_eq!(response.status, 200);
        assert!(response
            .headers
            .get("Content-Type")
            .unwrap()
            .contains("text/html"));
        assert_eq!(response.body, "<h1>Hello</h1>");
    }

    #[test]
    fn test_json_response() {
        let json = json!({"message": "success"});
        let response = json_response(json);
        assert_eq!(response.status, 200);
        assert_eq!(
            response.headers.get("Content-Type").unwrap(),
            "application/json"
        );
        assert!(response.body.contains("success"));
    }
}
