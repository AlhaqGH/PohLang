/// Middleware system for PohLang web framework
/// Provides request/response pipeline with before/after hooks

use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use super::http::{HttpRequest, HttpResponse};

/// Middleware function signature
/// Takes request, may modify it, returns whether to continue
pub type MiddlewareFunc = Arc<dyn Fn(&mut HttpRequest, &mut MiddlewareContext) -> Result<bool> + Send + Sync>;

/// Response middleware function signature
/// Called after route handler, can modify response
pub type ResponseMiddlewareFunc = Arc<dyn Fn(&HttpRequest, &mut HttpResponse, &MiddlewareContext) -> Result<()> + Send + Sync>;

/// Context passed through middleware chain
#[derive(Debug, Clone)]
pub struct MiddlewareContext {
    pub data: HashMap<String, String>,
    pub start_time: Instant,
}

impl MiddlewareContext {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            start_time: Instant::now(),
        }
    }
    
    pub fn set(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }
    
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
    
    pub fn elapsed_ms(&self) -> u128 {
        self.start_time.elapsed().as_millis()
    }
}

impl Default for MiddlewareContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Middleware chain manager
pub struct MiddlewareChain {
    request_middleware: Vec<MiddlewareFunc>,
    response_middleware: Vec<ResponseMiddlewareFunc>,
}

impl MiddlewareChain {
    pub fn new() -> Self {
        Self {
            request_middleware: Vec::new(),
            response_middleware: Vec::new(),
        }
    }
    
    /// Adds request middleware (runs before route handler)
    pub fn add_request_middleware(&mut self, middleware: MiddlewareFunc) {
        self.request_middleware.push(middleware);
    }
    
    /// Adds response middleware (runs after route handler)
    pub fn add_response_middleware(&mut self, middleware: ResponseMiddlewareFunc) {
        self.response_middleware.push(middleware);
    }
    
    /// Runs all request middleware
    /// Returns false if any middleware stops the chain
    pub fn run_request(&self, request: &mut HttpRequest, context: &mut MiddlewareContext) -> Result<bool> {
        for middleware in &self.request_middleware {
            if !middleware(request, context)? {
                return Ok(false);
            }
        }
        Ok(true)
    }
    
    /// Runs all response middleware
    pub fn run_response(&self, request: &HttpRequest, response: &mut HttpResponse, context: &MiddlewareContext) -> Result<()> {
        for middleware in &self.response_middleware {
            middleware(request, response, context)?;
        }
        Ok(())
    }
}

impl Default for MiddlewareChain {
    fn default() -> Self {
        Self::new()
    }
}

/// Built-in middleware functions

/// CORS middleware - adds CORS headers
pub fn cors_middleware(
    allowed_origins: Vec<String>,
    allowed_methods: Vec<String>,
    allowed_headers: Vec<String>,
) -> ResponseMiddlewareFunc {
    Arc::new(move |_req, response, _ctx| {
        response.headers.insert(
            "Access-Control-Allow-Origin".to_string(),
            allowed_origins.join(", "),
        );
        response.headers.insert(
            "Access-Control-Allow-Methods".to_string(),
            allowed_methods.join(", "),
        );
        response.headers.insert(
            "Access-Control-Allow-Headers".to_string(),
            allowed_headers.join(", "),
        );
        Ok(())
    })
}

/// Logging middleware - logs requests
pub fn logging_middleware() -> MiddlewareFunc {
    Arc::new(|req, ctx| {
        println!("[REQUEST] {} {}", req.method, req.path);
        ctx.set("logged", "true");
        Ok(true)
    })
}

/// Response time middleware - adds X-Response-Time header
pub fn response_time_middleware() -> ResponseMiddlewareFunc {
    Arc::new(|_req, response, ctx| {
        let elapsed = ctx.elapsed_ms();
        response.headers.insert(
            "X-Response-Time".to_string(),
            format!("{}ms", elapsed),
        );
        Ok(())
    })
}

/// Authentication middleware - checks for auth token
pub fn auth_middleware(token_name: String, required_token: String) -> MiddlewareFunc {
    Arc::new(move |req, _ctx| {
        if let Some(token) = req.headers.get(&token_name) {
            if token == &required_token {
                return Ok(true);
            }
        }
        Ok(false) // Stop chain - unauthorized
    })
}

/// Rate limiting middleware (simple in-memory)
pub struct RateLimiter {
    requests: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
    max_requests: usize,
    window_secs: u64,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_secs: u64) -> Self {
        Self {
            requests: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window_secs,
        }
    }
    
    pub fn middleware(&self) -> MiddlewareFunc {
        let requests = Arc::clone(&self.requests);
        let max_requests = self.max_requests;
        let window_secs = self.window_secs;
        
        Arc::new(move |req, _ctx| {
            let client_ip = req.headers
                .get("X-Forwarded-For")
                .or_else(|| req.headers.get("X-Real-IP"))
                .cloned()
                .unwrap_or_else(|| "unknown".to_string());
            
            let mut requests_map = requests.lock().unwrap();
            let now = Instant::now();
            
            // Get or create request history
            let history = requests_map.entry(client_ip.clone()).or_insert_with(Vec::new);
            
            // Remove old requests outside window
            history.retain(|&time| now.duration_since(time).as_secs() < window_secs);
            
            // Check if limit exceeded
            if history.len() >= max_requests {
                return Ok(false); // Rate limit exceeded
            }
            
            // Add current request
            history.push(now);
            
            Ok(true)
        })
    }
}

/// Body size limit middleware
pub fn body_size_limit_middleware(max_size_bytes: usize) -> MiddlewareFunc {
    Arc::new(move |req, _ctx| {
        if req.body.len() > max_size_bytes {
            return Ok(false);
        }
        Ok(true)
    })
}

/// Security headers middleware
pub fn security_headers_middleware() -> ResponseMiddlewareFunc {
    Arc::new(|_req, response, _ctx| {
        response.headers.insert(
            "X-Content-Type-Options".to_string(),
            "nosniff".to_string(),
        );
        response.headers.insert(
            "X-Frame-Options".to_string(),
            "DENY".to_string(),
        );
        response.headers.insert(
            "X-XSS-Protection".to_string(),
            "1; mode=block".to_string(),
        );
        response.headers.insert(
            "Strict-Transport-Security".to_string(),
            "max-age=31536000; includeSubDomains".to_string(),
        );
        Ok(())
    })
}

/// Compression middleware (placeholder - would need actual compression)
pub fn compression_middleware() -> ResponseMiddlewareFunc {
    Arc::new(|_req, response, _ctx| {
        // Would implement gzip/deflate compression here
        response.headers.insert(
            "Content-Encoding".to_string(),
            "identity".to_string(), // No compression yet
        );
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_middleware_chain() {
        let mut chain = MiddlewareChain::new();
        
        // Add logging middleware
        chain.add_request_middleware(Arc::new(|_req, ctx| {
            ctx.set("step1", "done");
            Ok(true)
        }));
        
        chain.add_request_middleware(Arc::new(|_req, ctx| {
            ctx.set("step2", "done");
            Ok(true)
        }));
        
        let mut req = HttpRequest {
            method: "GET".to_string(),
            path: "/test".to_string(),
            headers: HashMap::new(),
            body: String::new(),
        };
        
        let mut ctx = MiddlewareContext::new();
        let result = chain.run_request(&mut req, &mut ctx).unwrap();
        
        assert!(result);
        assert_eq!(ctx.get("step1"), Some(&"done".to_string()));
        assert_eq!(ctx.get("step2"), Some(&"done".to_string()));
    }
    
    #[test]
    fn test_middleware_stops_chain() {
        let mut chain = MiddlewareChain::new();
        
        chain.add_request_middleware(Arc::new(|_req, _ctx| {
            Ok(true) // Continue
        }));
        
        chain.add_request_middleware(Arc::new(|_req, _ctx| {
            Ok(false) // Stop chain
        }));
        
        chain.add_request_middleware(Arc::new(|_req, ctx| {
            ctx.set("should_not_run", "true");
            Ok(true)
        }));
        
        let mut req = HttpRequest {
            method: "GET".to_string(),
            path: "/test".to_string(),
            headers: HashMap::new(),
            body: String::new(),
        };
        
        let mut ctx = MiddlewareContext::new();
        let result = chain.run_request(&mut req, &mut ctx).unwrap();
        
        assert!(!result);
        assert_eq!(ctx.get("should_not_run"), None);
    }
    
    #[test]
    fn test_rate_limiter() {
        let limiter = RateLimiter::new(3, 1); // 3 requests per second
        let middleware = limiter.middleware();
        
        let mut req = HttpRequest {
            method: "GET".to_string(),
            path: "/test".to_string(),
            headers: HashMap::new(),
            body: String::new(),
        };
        
        let mut ctx = MiddlewareContext::new();
        
        // First 3 requests should succeed
        assert!(middleware(&mut req, &mut ctx).unwrap());
        assert!(middleware(&mut req, &mut ctx).unwrap());
        assert!(middleware(&mut req, &mut ctx).unwrap());
        
        // 4th request should fail
        assert!(!middleware(&mut req, &mut ctx).unwrap());
    }
}
