# PohLang Web Framework Guide

## Overview

PohLang includes a comprehensive web framework that supports modern web development features including:
- Advanced routing with path parameters
- Middleware system (CORS, auth, logging, rate limiting)
- File uploads with validation
- Static file serving with MIME types
- Session management
- Template rendering
- And more...

## Quick Start

### Basic Web Server

```pohlang
Create web server on port 8080

Add route "/" with:
    Write html response "<h1>Welcome to PohLang!</h1>"

Start server
```

### Advanced Routing with Path Parameters

```pohlang
# Route with path parameters
Add route "/users/:id" with:
    Get the request
    Set user_id to (Get path parameter "id")
    Write json response with "user_id" as user_id

# Multiple parameters
Add route "/posts/:year/:month/:slug" with:
    Get the request
    Set year to (Get path parameter "year")
    Set month to (Get path parameter "month")
    Set slug to (Get path parameter "slug")
    Write json response with "year" as year and "month" as month and "slug" as slug

# Wildcard routes
Add route "/files/*" with:
    Get the request
    Set path to (Get path parameter "*")
    Serve static file from path
```

## Routing System

### Simple Routes

```pohlang
Add route "/" with:
    Write html response "<h1>Home</h1>"

Add route "/about" with:
    Write html response "<h1>About</h1>"

Add route "/api/data" with:
    Write json response with "status" as "ok" and "data" as Make a list of 1, 2, 3
```

### Path Parameters

Path parameters let you extract values from URLs:

```pohlang
# Single parameter
Add route "/users/:id" with:
    Set id to (Get path parameter "id")
    Write json response with "user" as id

# Multiple parameters
Add route "/blog/:year/:month/:day" with:
    Set year to (Get path parameter "year")
    Set month to (Get path parameter "month")
    Set day to (Get path parameter "day")
    Write html response with "Date: " + year + "-" + month + "-" + day

# Optional parameters with defaults
Add route "/search" with:
    Set query to (Get query parameter "q" or "")
    Set page to (Get query parameter "page" or "1")
    Write json response with "query" as query and "page" as page
```

### Route Groups

Organize routes with common prefixes:

```pohlang
Create route group "/api/v1"

Add route "/users" to group with:
    Write json response with "users" as Make a list of "user1", "user2"

Add route "/posts" to group with:
    Write json response with "posts" as Make a list of "post1", "post2"

# Routes become: /api/v1/users and /api/v1/posts
```

## Middleware System

Middleware processes requests before they reach your route handlers and responses before they're sent.

### CORS Middleware

```pohlang
Add CORS middleware with:
    Allow origins "*"
    Allow methods "GET", "POST", "PUT", "DELETE"
    Allow headers "Content-Type", "Authorization"
```

### Logging Middleware

```pohlang
Add logging middleware

# Logs every request:
# [REQUEST] GET /api/users
# [REQUEST] POST /api/posts
```

### Authentication Middleware

```pohlang
Add auth middleware with:
    Require header "Authorization"
    Match token "secret-token-123"

# Or custom auth
Add middleware "custom-auth" with:
    Get the request
    Set token to (Get header "Authorization")
    If token equals "Bearer valid-token" then:
        Continue with request
    Else:
        Write response 401 with "Unauthorized"
        Stop middleware chain
```

### Rate Limiting

```pohlang
Add rate limiter with:
    Max requests 100
    Time window 60 seconds
    
# Limits clients to 100 requests per minute
```

### Security Headers

```pohlang
Add security headers middleware

# Adds headers:
# X-Content-Type-Options: nosniff
# X-Frame-Options: DENY
# X-XSS-Protection: 1; mode=block
# Strict-Transport-Security: max-age=31536000
```

### Custom Middleware

```pohlang
Create middleware "timing" with:
    Get the request
    Set start_time to (Current time)
    Continue with request
    Get the response
    Set elapsed to (Current time) minus start_time
    Add header "X-Response-Time" with elapsed + "ms"
```

## File Uploads

Handle file uploads with validation:

```pohlang
Add route "/upload" with method "POST" with:
    Get the request
    Parse multipart form data
    
    Get uploaded file "avatar"
    
    # Validate file
    If file size greater than 10485760 then:  # 10MB
        Write response 413 with "File too large"
        Return
    
    If file type not in Make a list of "image/jpeg", "image/png" then:
        Write response 400 with "Invalid file type"
        Return
    
    # Save file
    Save file to "uploads/" + (Generate unique filename)
    
    Write json response with "message" as "File uploaded" and "filename" as filename
```

### Upload Configuration

```pohlang
Configure upload with:
    Max file size 10485760        # 10MB
    Max total size 104857600      # 100MB
    Allowed extensions ".jpg", ".png", ".pdf"
    Allowed types "image/jpeg", "image/png", "application/pdf"
    Upload directory "./uploads"
    Temp directory "./temp"
```

### Form Data Parsing

```pohlang
# Multipart form data
Add route "/form" with method "POST" with:
    Get the request
    Parse form data
    
    Set name to (Get form field "name")
    Set email to (Get form field "email")
    Get uploaded file "photo"
    
    Write json response with "name" as name and "email" as email

# URL-encoded form data
Add route "/simple-form" with method "POST" with:
    Get the request
    Parse urlencoded data
    
    Set username to (Get form field "username")
    Set password to (Get form field "password")
    
    # Process login...
```

## Static Files

Serve static files (HTML, CSS, JS, images) from a directory:

```pohlang
# Serve from public directory
Serve static files from "./public"

# Or specific route
Add route "/static/*" with:
    Set path to (Get path parameter "*")
    Serve file from "./public/" + path
```

### Static File Configuration

```pohlang
Configure static files with:
    Root directory "./public"
    Index files "index.html", "index.htm"
    Enable directory listing false
    Enable cache true
    Cache max age 3600  # 1 hour
    Security check true  # Prevent directory traversal
```

### MIME Types

Automatic MIME type detection for:
- **Text**: .html, .css, .js, .json, .xml, .txt
- **Images**: .jpg, .png, .gif, .svg, .ico, .webp
- **Fonts**: .woff, .woff2, .ttf, .otf
- **Documents**: .pdf, .doc, .docx, .xls, .xlsx
- **Audio/Video**: .mp3, .ogg, .wav, .mp4, .webm
- **Archives**: .zip, .tar, .gz

## Sessions

Manage user sessions with cookies:

```pohlang
# Enable sessions
Enable sessions with:
    Secret key "my-secret-key-change-this"
    Session timeout 3600  # 1 hour
    Cookie name "session_id"
    Secure cookies true
    Same site "Strict"

# Set session data
Add route "/login" with method "POST" with:
    Get the request
    Parse form data
    Set username to (Get form field "username")
    
    # Validate credentials...
    
    Create session
    Set session value "user" to username
    Set session value "login_time" to (Current time)
    
    Write json response with "message" as "Logged in"

# Get session data
Add route "/profile" with:
    Get the session
    Set user to (Get session value "user")
    
    If user is empty then:
        Write response 401 with "Not logged in"
        Return
    
    Write html response with "Welcome " + user

# Destroy session
Add route "/logout" with:
    Destroy session
    Write json response with "message" as "Logged out"
```

## WebSockets

Real-time bidirectional communication:

```pohlang
# WebSocket endpoint
Add websocket route "/ws/chat" with:
    On connect:
        Write "Client connected"
        Send message "Welcome to chat!"
    
    On message:
        Set msg to (Get message)
        Write "Received: " + msg
        Broadcast message msg to all clients
    
    On disconnect:
        Write "Client disconnected"

# Client-side JavaScript
# const ws = new WebSocket('ws://localhost:8080/ws/chat');
# ws.onmessage = (event) => console.log(event.data);
# ws.send('Hello server!');
```

## Templates

Render dynamic HTML with templates:

```pohlang
# Load template
Load template "user.html" from "./templates"

# Render with data
Add route "/user/:id" with:
    Set id to (Get path parameter "id")
    Set user to (Fetch user by id)
    
    Render template "user.html" with:
        "username" as user username
        "email" as user email
        "joined" as user created_at
    
    Write html response with rendered content
```

**user.html template:**
```html
<!DOCTYPE html>
<html>
<head>
    <title>{{username}}'s Profile</title>
</head>
<body>
    <h1>{{username}}</h1>
    <p>Email: {{email}}</p>
    <p>Joined: {{joined}}</p>
</body>
</html>
```

## Complete Example: REST API

```pohlang
# Create a full REST API with all features

Create web server on port 8080

# Configure
Configure upload with max file size 10485760
Enable sessions with secret "change-this-secret"
Serve static files from "./public"

# Middleware
Add CORS middleware with allow origins "*"
Add logging middleware
Add security headers middleware
Add rate limiter with max requests 100 and time window 60

# API Routes
Create route group "/api/v1"

# GET /api/v1/users
Add route "/users" to group with:
    Set users to Make a list of:
        Make a dictionary with "id" as 1 and "name" as "Alice"
        Make a dictionary with "id" as 2 and "name" as "Bob"
    Write json response with "users" as users

# GET /api/v1/users/:id
Add route "/users/:id" to group with:
    Set id to (Get path parameter "id")
    # Fetch user from database...
    Write json response with "id" as id and "name" as "User " + id

# POST /api/v1/users
Add route "/users" to group with method "POST" with:
    Get the session
    Set user to (Get session value "user")
    
    If user is empty then:
        Write response 401 with "Unauthorized"
        Return
    
    Get the request
    Parse json body
    Set name to (Get json field "name")
    Set email to (Get json field "email")
    
    # Save to database...
    
    Write response 201 with json:
        "id" as 123
        "name" as name
        "email" as email

# POST /api/v1/upload
Add route "/upload" to group with method "POST" with:
    Get the request
    Parse multipart form data
    Get uploaded file "file"
    
    If file size greater than 10485760 then:
        Write response 413 with "File too large"
        Return
    
    Save file to "uploads/" + (Generate unique filename)
    Write json response with "filename" as filename

# Start server
Start server
Write "Server running on http://localhost:8080"
Write "API available at http://localhost:8080/api/v1"
```

## Database Integration

Connect to databases for persistence:

```pohlang
# SQLite
Connect to sqlite database "./data.db"

Create table "users" with columns:
    "id" as integer primary key
    "username" as text unique
    "email" as text
    "created_at" as datetime default current_timestamp

# Insert data
Add route "/register" with method "POST" with:
    Get the request
    Parse json body
    Set username to (Get json field "username")
    Set email to (Get json field "email")
    
    Insert into "users" values username and email
    
    Write response 201 with "User created"

# Query data
Add route "/users" with:
    Select all from "users"
    Write json response with "users" as results

# Update data
Add route "/users/:id" with method "PUT" with:
    Set id to (Get path parameter "id")
    Get the request
    Parse json body
    Set email to (Get json field "email")
    
    Update "users" set "email" to email where "id" equals id
    
    Write json response with "message" as "Updated"

# Delete data
Add route "/users/:id" with method "DELETE" with:
    Set id to (Get path parameter "id")
    Delete from "users" where "id" equals id
    Write response 204
```

## Error Handling

Handle errors gracefully:

```pohlang
# Global error handler
Set error handler with:
    Get the error
    Write "Error occurred: " + error message
    Write response 500 with json:
        "error" as error message
        "timestamp" as (Current time)

# Route-specific error handling
Add route "/api/data" with:
    Try:
        # Risky operation
        Set data to (Fetch data from external API)
        Write json response with "data" as data
    Catch error:
        Write "Failed to fetch data: " + error
        Write response 503 with "Service temporarily unavailable"

# 404 handler
Set not found handler with:
    Write response 404 with html:
        "<h1>404 Not Found</h1><p>The page you're looking for doesn't exist.</p>"
```

## Performance & Production

### Caching

```pohlang
# Enable response caching
Enable cache with:
    Max age 3600
    Cache headers "Cache-Control: public, max-age=3600"

# Add ETag support
Add etag middleware
```

### Compression

```pohlang
# Enable gzip compression
Enable compression with:
    Min size 1024  # Compress responses > 1KB
    Types "text/html", "text/css", "application/json"
```

### HTTPS/TLS

```pohlang
Create secure web server on port 443 with:
    Certificate "./certs/cert.pem"
    Private key "./certs/key.pem"
```

### Environment Configuration

```pohlang
# Load from .env file
Load environment from ".env"

Set port to (Get env "PORT" or "8080")
Set db_path to (Get env "DATABASE_URL" or "./data.db")
Set secret to (Get env "SECRET_KEY" or "change-this")

Create web server on port port
Enable sessions with secret secret
Connect to database db_path
```

## Best Practices

1. **Security**
   - Always validate user input
   - Use HTTPS in production
   - Set strong session secrets
   - Enable CORS carefully
   - Validate file uploads

2. **Performance**
   - Enable caching for static files
   - Use compression for text responses
   - Set appropriate cache headers
   - Use rate limiting to prevent abuse

3. **Error Handling**
   - Set global error handlers
   - Return appropriate HTTP status codes
   - Log errors for debugging
   - Don't expose sensitive information

4. **Code Organization**
   - Use route groups for API versioning
   - Separate middleware by concern
   - Keep route handlers focused
   - Use templates for HTML

5. **Testing**
   - Test all routes
   - Test middleware chains
   - Test error conditions
   - Test file uploads with various sizes/types

## See Also

- [Complete Guide](./COMPLETE_GUIDE.md) - Full language reference
- [Cheat Sheet](./CHEAT_SHEET.md) - Quick syntax reference
- [Web Framework Plan](./WEB_FRAMEWORK_PLAN.md) - Implementation roadmap
- [Examples](../examples/) - Example applications
