# Path Parameters in PohLang Web Server

## Overview
PohLang now supports dynamic path parameters in web routes, allowing you to extract values from URLs like `/users/:id` or `/posts/:year/:month/:slug`.

## Syntax

### Defining Routes with Parameters
```pohlang
Add route "/users/:id" with method "GET" to server:
    # Handler code here
```

### Accessing Path Parameters
```pohlang
Set param_value to get path parameter "param_name"
```

## Complete Example

```pohlang
Start Program

# Create web server
Set server to create web server on port 8080

# Single path parameter
Add route "/users/:id" with method "GET" to server:
    Set user_id to get path parameter "id"
    Set response to make a dictionary with "message" set to "User endpoint", "id" set to user_id
    Write json response with response

# Multiple path parameters
Add route "/posts/:year/:month/:slug" with method "GET" to server:
    Set year_value to get path parameter "year"
    Set month_value to get path parameter "month"
    Set slug_value to get path parameter "slug"
    Set response to make a dictionary with "year" set to year_value, "month" set to month_value, "slug" set to slug_value
    Write json response with response

Start server

End Program
```

## Testing

Given the route `/users/:id`, a request to `/users/123` will extract:
- `id` = "123"

Given the route `/posts/:year/:month/:slug`, a request to `/posts/2024/01/hello-world` will extract:
- `year` = "2024"
- `month` = "01"
- `slug` = "hello-world"

## Implementation Details

### Route Pattern Matching
1. Routes are first tried for exact matches (backwards compatibility)
2. If no exact match, pattern matching with `:param` syntax is attempted
3. Parameters are extracted using regex patterns
4. Extracted values are stored in a HashMap

### Technical Flow
1. **Route Definition**: When adding a route with `:param` syntax, the pattern is parsed and stored
2. **Request Matching**: Incoming requests are matched against both exact routes and pattern routes
3. **Parameter Extraction**: If a pattern route matches, parameters are extracted from the URL
4. **Handler Access**: Parameters are available in the handler via `Get path parameter "name"`

### Code Changes
- **AST**: Added `GetPathParam(Box<Expr>)` expression type
- **Parser**: Added `P_GET_PATH_PARAM` phrase recognition
- **VM**: HttpRequest now stored in handler context as `__request`
- **HTTP Server**: Integrated RoutePattern matching with parameter extraction

## Supported Features
- ✅ Single parameters: `/users/:id`
- ✅ Multiple parameters: `/posts/:year/:month/:slug`
- ✅ Mix of static and dynamic segments
- ✅ Type-safe parameter access
- ✅ Error handling for missing parameters
- ✅ Backwards compatible with exact-match routes

## Error Handling
If you try to access a parameter that doesn't exist:
```pohlang
Set value to get path parameter "nonexistent"
# Error: "Path parameter 'nonexistent' not found"
```

If you try to access parameters outside a route handler:
```pohlang
Set value to get path parameter "id"
# Error: "get path parameter: no request context available"
```

## Next Steps
- Add query parameter access: `Get query parameter "key"`
- Add request header access: `Get request header "Content-Type"`
- Add request body parsing for POST/PUT requests
