/// Advanced routing system for PohLang web framework
/// Supports path parameters, query strings, route groups, and middleware

use anyhow::{anyhow, Result};
use regex::Regex;
use std::collections::HashMap;
use std::sync::Arc;

use super::http::{HttpRequest, HttpResponse, RouteHandler};

/// Represents a route pattern that can include parameters
#[derive(Debug, Clone)]
pub struct RoutePattern {
    pub pattern: String,
    pub param_names: Vec<String>,
    pub regex: Regex,
}

impl RoutePattern {
    /// Creates a new route pattern from a path string
    /// Converts "/users/:id" to regex and extracts parameter names
    pub fn new(pattern: &str) -> Result<Self> {
        let mut param_names = Vec::new();
        let mut regex_pattern = String::from("^");
        
        let parts: Vec<&str> = pattern.split('/').collect();
        
        for (i, part) in parts.iter().enumerate() {
            if i > 0 {
                regex_pattern.push('/');
            }
            
            if part.starts_with(':') {
                // Path parameter like :id
                let param_name = part[1..].to_string();
                param_names.push(param_name);
                regex_pattern.push_str(r"([^/]+)");
            } else if *part == "*" {
                // Wildcard match
                param_names.push("*".to_string());
                regex_pattern.push_str(r"(.*)");
            } else {
                // Literal path segment
                regex_pattern.push_str(&regex::escape(part));
            }
        }
        
        regex_pattern.push('$');
        
        let regex = Regex::new(&regex_pattern)
            .map_err(|e| anyhow!("Invalid route pattern '{}': {}", pattern, e))?;
        
        Ok(Self {
            pattern: pattern.to_string(),
            param_names,
            regex,
        })
    }
    
    /// Attempts to match a path against this pattern
    /// Returns Some(params) if match, None otherwise
    pub fn matches(&self, path: &str) -> Option<HashMap<String, String>> {
        let captures = self.regex.captures(path)?;
        
        let mut params = HashMap::new();
        for (i, name) in self.param_names.iter().enumerate() {
            if let Some(value) = captures.get(i + 1) {
                params.insert(name.clone(), value.as_str().to_string());
            }
        }
        
        Some(params)
    }
}

/// Enhanced route with pattern matching
#[derive(Clone)]
pub struct EnhancedRoute {
    pub pattern: RoutePattern,
    pub method: String,
    pub handler: RouteHandler,
    pub middleware: Vec<String>, // Middleware IDs to apply
}

impl std::fmt::Debug for EnhancedRoute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnhancedRoute")
            .field("pattern", &self.pattern.pattern)
            .field("method", &self.method)
            .field("middleware", &self.middleware)
            .field("handler", &"<function>")
            .finish()
    }
}

/// Router with advanced features
pub struct Router {
    routes: Vec<EnhancedRoute>,
    middleware: HashMap<String, RouteHandler>,
    not_found_handler: Option<RouteHandler>,
    error_handler: Option<Arc<dyn Fn(anyhow::Error) -> HttpResponse + Send + Sync>>,
}

impl std::fmt::Debug for Router {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Router")
            .field("routes", &self.routes)
            .field("middleware_count", &self.middleware.len())
            .field("has_not_found_handler", &self.not_found_handler.is_some())
            .field("has_error_handler", &self.error_handler.is_some())
            .finish()
    }
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            middleware: HashMap::new(),
            not_found_handler: None,
            error_handler: None,
        }
    }
    
    /// Adds a route with path parameter support
    pub fn add_route(
        &mut self,
        pattern: &str,
        method: &str,
        handler: RouteHandler,
    ) -> Result<()> {
        let route_pattern = RoutePattern::new(pattern)?;
        self.routes.push(EnhancedRoute {
            pattern: route_pattern,
            method: method.to_uppercase(),
            handler,
            middleware: Vec::new(),
        });
        Ok(())
    }
    
    /// Adds a route with specific middleware
    pub fn add_route_with_middleware(
        &mut self,
        pattern: &str,
        method: &str,
        handler: RouteHandler,
        middleware: Vec<String>,
    ) -> Result<()> {
        let route_pattern = RoutePattern::new(pattern)?;
        self.routes.push(EnhancedRoute {
            pattern: route_pattern,
            method: method.to_uppercase(),
            handler,
            middleware,
        });
        Ok(())
    }
    
    /// Registers middleware
    pub fn add_middleware(&mut self, name: &str, handler: RouteHandler) {
        self.middleware.insert(name.to_string(), handler);
    }
    
    /// Sets custom 404 handler
    pub fn set_not_found_handler(&mut self, handler: RouteHandler) {
        self.not_found_handler = Some(handler);
    }
    
    /// Sets custom error handler
    pub fn set_error_handler(
        &mut self,
        handler: Arc<dyn Fn(anyhow::Error) -> HttpResponse + Send + Sync>,
    ) {
        self.error_handler = Some(handler);
    }
    
    /// Finds a matching route and extracts path parameters
    pub fn find_route(&self, path: &str, method: &str) -> Option<(&EnhancedRoute, HashMap<String, String>)> {
        for route in &self.routes {
            if route.method == method.to_uppercase() {
                if let Some(params) = route.pattern.matches(path) {
                    return Some((route, params));
                }
            }
        }
        None
    }
    
    /// Handles a request through the router
    pub fn handle(&self, mut request: HttpRequest) -> Result<HttpResponse> {
        // Find matching route
        if let Some((route, params)) = self.find_route(&request.path, &request.method) {
            // Add path parameters to request
            request.query.extend(params.clone());
            
            // Apply middleware in order
            for middleware_name in &route.middleware {
                if let Some(middleware) = self.middleware.get(middleware_name) {
                    let response = middleware(request.clone())?;
                    // If middleware returns non-200, stop processing
                    if response.status != 200 {
                        return Ok(response);
                    }
                }
            }
            
            // Call route handler
            match (route.handler)(request) {
                Ok(response) => Ok(response),
                Err(e) => {
                    if let Some(error_handler) = &self.error_handler {
                        Ok(error_handler(e))
                    } else {
                        Err(e)
                    }
                }
            }
        } else {
            // No route found - return 404
            if let Some(not_found) = &self.not_found_handler {
                Ok(not_found(request)?)
            } else {
                Ok(HttpResponse {
                    status: 404,
                    headers: HashMap::new(),
                    body: format!("404 Not Found: {}", request.path),
                })
            }
        }
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

/// Route group for organizing routes
pub struct RouteGroup {
    prefix: String,
    router: Router,
}

impl RouteGroup {
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
            router: Router::new(),
        }
    }
    
    pub fn add_route(
        &mut self,
        pattern: &str,
        method: &str,
        handler: RouteHandler,
    ) -> Result<()> {
        let full_pattern = format!("{}{}", self.prefix, pattern);
        self.router.add_route(&full_pattern, method, handler)
    }
    
    pub fn merge_into(self, router: &mut Router) {
        router.routes.extend(self.router.routes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_route_pattern_simple() {
        let pattern = RoutePattern::new("/users").unwrap();
        assert!(pattern.matches("/users").is_some());
        assert!(pattern.matches("/posts").is_none());
    }
    
    #[test]
    fn test_route_pattern_with_param() {
        let pattern = RoutePattern::new("/users/:id").unwrap();
        let params = pattern.matches("/users/123").unwrap();
        assert_eq!(params.get("id"), Some(&"123".to_string()));
    }
    
    #[test]
    fn test_route_pattern_multiple_params() {
        let pattern = RoutePattern::new("/users/:user_id/posts/:post_id").unwrap();
        let params = pattern.matches("/users/42/posts/99").unwrap();
        assert_eq!(params.get("user_id"), Some(&"42".to_string()));
        assert_eq!(params.get("post_id"), Some(&"99".to_string()));
    }
    
    #[test]
    fn test_route_pattern_wildcard() {
        let pattern = RoutePattern::new("/static/*").unwrap();
        let params = pattern.matches("/static/css/style.css").unwrap();
        assert_eq!(params.get("*"), Some(&"css/style.css".to_string()));
    }
}
