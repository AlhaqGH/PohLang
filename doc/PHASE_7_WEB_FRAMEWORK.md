# PohLang Web Framework - Phase 7 Design Document

## Mission: Make Task Master a Real Web App! 🌐

**Goal**: Transform Task Master from a CLI app to a fully functional web application accessible in any browser.

---

## Current State Analysis

### ✅ What PohLang Has (v0.5.4)
- Variables and data structures (Dictionary, List)
- File I/O operations
- Control flow (If, While, loops)
- Functions
- Error handling
- String operations
- Basic collections

### ❌ What PohLang Needs for Web Apps

#### Critical Missing Features:
1. **HTTP Server** - Ability to listen on a port and serve requests
2. **Routing** - Map URLs to handlers
3. **HTTP Request/Response** - Parse requests, build responses
4. **HTML Templating** - Generate dynamic HTML
5. **Static File Serving** - Serve CSS, JS, images
6. **JSON API** - Parse and generate JSON
7. **Sessions/Cookies** - User state management
8. **WebSocket Support** - Real-time updates (optional)

---

## Proposed PohLang Web Syntax

### 1. HTTP Server Creation

```pohlang
Start Program

# Create a web server
Make server to Create web server on port 3000

# Server is now ready to accept routes
Write "Server created on port 3000"

End Program
```

### 2. Route Definitions

```pohlang
# Define a GET route
Make route "/" with method "GET" and handler home_handler

# Route with parameters
Make route "/task/:id" with method "GET" and handler get_task_handler

# POST route for creating tasks
Make route "/api/tasks" with method "POST" and handler create_task_handler
```

### 3. Request/Response Handling

```pohlang
Make home_handler with request:
    # Access request properties
    Set path to request["path"]
    Set method to request["method"]
    Set query to request["query"]
    Set headers to request["headers"]
    Set body to request["body"]
    
    # Build response
    Make response to Dictionary contains
        "status" set to 200,
        "headers" set to Dictionary contains "Content-Type" set to "text/html",
        "body" set to "<h1>Welcome to Task Master</h1>"
    
    Return response
End
```

### 4. HTML Templating

```pohlang
# Template with variables
Make template to """
<!DOCTYPE html>
<html>
<head>
    <title>{{title}}</title>
</head>
<body>
    <h1>{{heading}}</h1>
    <p>{{content}}</p>
</body>
</html>
"""

# Render template with data
Make data to Dictionary contains
    "title" set to "Task Master",
    "heading" set to "My Tasks",
    "content" set to "You have 5 tasks"

Set html to render template with data

# Or use phrasal expression:
Set html to template rendered with data
```

### 5. JSON Operations

```pohlang
# Parse JSON from string
Set json_str to '{"name":"Buy milk","priority":"high"}'
Set task_data to parse json json_str

# Generate JSON from dictionary
Make task to Dictionary contains
    "id" set to 1,
    "title" set to "Buy milk",
    "status" set to "pending"

Set json_response to task as json
# Or: Set json_response to convert task to json
```

### 6. Static File Serving

```pohlang
# Serve static files from directory
Use serve static files from "public"

# Or as route:
Make route "/static/*" with handler static_file_handler
```

### 7. Complete Web Server Example

```pohlang
Start Program

# Create server
Make server to Create web server on port 3000

# Home page route
Make server route "/" with method "GET":
    Make response to Dictionary contains
        "status" set to 200,
        "headers" set to Dictionary contains "Content-Type" set to "text/html",
        "body" set to "<h1>Task Master</h1>"
    Return response
End

# API route - Get all tasks
Make server route "/api/tasks" with method "GET":
    # Get tasks from storage
    Set tasks to load tasks from file
    
    # Return JSON
    Make response to Dictionary contains
        "status" set to 200,
        "headers" set to Dictionary contains "Content-Type" set to "application/json",
        "body" set to tasks as json
    Return response
End

# API route - Create task
Make server route "/api/tasks" with method "POST":
    # Parse request body
    Set task_data to parse json request["body"]
    
    # Create task
    Make new_task to Dictionary contains
        "id" set to next_id,
        "title" set to task_data["title"],
        "status" set to "pending"
    
    # Save task
    Use add task new_task
    
    # Return created task
    Make response to Dictionary contains
        "status" set to 201,
        "headers" set to Dictionary contains "Content-Type" set to "application/json",
        "body" set to new_task as json
    Return response
End

# Start server
Write "Starting Task Master web server..."
Use start server on port 3000
Write "Server running at http://localhost:3000"

End Program
```

---

## Implementation Plan

### Phase 7.1: Runtime HTTP Server (Rust)

Add to `runtime/src/stdlib/`:

1. **`http.rs`** - HTTP server implementation
   - Using `warp` or `actix-web` crate
   - Request/Response types
   - Route registration
   - Handler execution

2. **`json.rs`** - JSON operations
   - Parse JSON strings
   - Generate JSON from values
   - Using `serde_json` crate

3. **`template.rs`** - HTML templating
   - Simple variable substitution `{{var}}`
   - Loop support `{{#each items}}`
   - Conditionals `{{#if condition}}`
   - Using `handlebars` or custom implementation

### Phase 7.2: Parser Extensions

Add to `runtime/src/parser/`:

1. **Server creation syntax**
   ```rust
   "Create web server on port <port>"
   ```

2. **Route definition syntax**
   ```rust
   "Make route <path> with method <method> and handler <function>"
   ```

3. **JSON operations**
   ```rust
   "parse json <string>"
   "convert <value> to json"
   "<value> as json"
   ```

4. **Template rendering**
   ```rust
   "render <template> with <data>"
   "<template> rendered with <data>"
   ```

### Phase 7.3: VM Integration

Add to `runtime/src/vm/vm.rs`:

1. **New expression types**:
   - `CreateServer(port)`
   - `AddRoute(path, method, handler)`
   - `ParseJson(string)`
   - `ToJson(value)`
   - `RenderTemplate(template, data)`
   - `StartServer(port)`

2. **HTTP request/response handling**
3. **Async support** for server operations

---

## Task Master Web App Structure

```
Task_Master/
├── src/
│   ├── main.poh              # Web server entry point
│   ├── routes.poh            # Route handlers
│   ├── api.poh               # REST API endpoints
│   └── storage.poh           # Data persistence
├── public/
│   ├── index.html            # Main page
│   ├── css/
│   │   └── style.css         # Styling
│   ├── js/
│   │   └── app.js            # Frontend JavaScript
│   └── images/
│       └── logo.png
├── templates/
│   ├── layout.html           # Base template
│   ├── tasks.html            # Task list view
│   └── task-form.html        # Add/edit task form
└── data/
    └── tasks.json            # Task storage
```

---

## Minimal Viable Web App

### Backend (PohLang)

```pohlang
Start Program

Make server to Create web server on port 3000

# Serve homepage
Make server route "/":
    Set html to read file at "public/index.html"
    Return response with status 200 and body html
End

# API: Get all tasks
Make server route "/api/tasks" with method "GET":
    Set tasks_json to read file at "data/tasks.json"
    Return json response with body tasks_json
End

# API: Create task
Make server route "/api/tasks" with method "POST":
    Set task_data to parse json from request body
    # Add to tasks...
    Return json response with status 201
End

Use start server

End Program
```

### Frontend (HTML + JavaScript)

```html
<!DOCTYPE html>
<html>
<head>
    <title>Task Master</title>
    <link rel="stylesheet" href="/css/style.css">
</head>
<body>
    <h1>Task Master</h1>
    <div id="task-list"></div>
    <form id="add-task-form">
        <input type="text" id="title" placeholder="Task title">
        <button>Add Task</button>
    </form>
    <script src="/js/app.js"></script>
</body>
</html>
```

```javascript
// app.js
async function loadTasks() {
    const response = await fetch('/api/tasks');
    const tasks = await response.json();
    displayTasks(tasks);
}

async function addTask(title) {
    await fetch('/api/tasks', {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify({title, status: 'pending'})
    });
    loadTasks();
}

loadTasks();
```

---

## Alternative: APK (Android App)

For APK creation, we would need:

1. **Mobile UI Framework** - React Native, Flutter, or native Android
2. **PohLang Mobile Runtime** - Compile to native mobile
3. **Local Storage** - SQLite or similar
4. **UI Components** - Lists, forms, buttons

**Recommendation**: Start with **Web App** first, as it's:
- Easier to implement
- Works on all devices (desktop + mobile)
- No app store required
- Easier to update

---

## Next Steps

### Immediate Actions:

1. **Implement HTTP Server in Rust**
   - Add dependencies: `warp`, `tokio`, `serde_json`
   - Create `runtime/src/stdlib/http.rs`
   - Implement basic server functionality

2. **Add Parser Support**
   - Add "Create web server" syntax
   - Add "Make route" syntax
   - Add JSON operations

3. **Create Simple Web Demo**
   - Single endpoint serving HTML
   - Prove concept works

4. **Build Full Task Master Web App**
   - All CRUD operations via API
   - Beautiful responsive UI
   - Real-time updates

---

## Timeline Estimate

- **Week 1**: HTTP server runtime implementation
- **Week 2**: Parser integration + basic demo
- **Week 3**: Full Task Master web app
- **Week 4**: Polish, testing, deployment

---

## Questions to Address

1. **Async/Await**: Does PohLang need async syntax?
2. **Middleware**: Support for logging, authentication?
3. **Database**: Keep JSON files or add SQLite?
4. **Security**: CORS, CSRF protection?
5. **Deployment**: How to package and deploy?

---

**Status**: Design phase complete ✅  
**Next**: Begin implementation  
**Priority**: High - This unlocks real-world applications!

---

*This will make PohLang a true full-stack language! 🚀*
