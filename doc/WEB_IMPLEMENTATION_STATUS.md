# PohLang Web Framework Implementation Guide

## ✅ Phase 1: Runtime Implementation - COMPLETE!

### What We've Built

1. **HTTP Server Module** (`runtime/src/stdlib/http.rs`)
   - ✅ `WebServer` struct for creating servers
   - ✅ Route registration system
   - ✅ Request/Response handling
   - ✅ JSON support (already had it in network.rs)
   - ✅ Helper functions: `html_response()`, `json_response()`, `error_response()`
   - ✅ Built on `tiny_http` (pure Rust, no native dependencies)

2. **Template Engine** (`runtime/src/stdlib/template.rs`)
   - ✅ Variable substitution: `{{variable}}`
   - ✅ Nested properties: `{{user.name}}`
   - ✅ Loops: `{{#each items}}...{{/each}}`
   - ✅ Conditionals: `{{#if condition}}...{{/if}}`
   - ✅ Full rendering: `render_full(template, data)`

3. **Dependencies Added**
   - `tiny_http = "0.12"` - Lightweight HTTP server
   - `handlebars = "5.0"` - Template engine (for future)
   - `once_cell = "1.19"` - Static initialization

---

## 🚧 Phase 2: Parser Integration - IN PROGRESS

We need to add PohLang syntax for web features. Here's the plan:

### New Expressions Needed

#### 1. Create Web Server
```pohlang
Make server to Create web server on port 3000
```
**Parser**: Recognize "Create web server on port <number>"  
**Returns**: WebServer object

#### 2. Add Route
```pohlang
Make server route "/":
    # Handler code here
    Return response
End
```
**Parser**: Handle route definitions with inline handlers  
**Alternative syntax**:
```pohlang
Add route "/" with method "GET" to server with handler home_handler
```

#### 3. Start Server
```pohlang
Use start server
# Or:
Start server on port 3000
```
**Parser**: Recognize server start command (blocking)

#### 4. Create Response
```pohlang
Make response to Dictionary contains
    "status" set to 200,
    "headers" set to Dictionary contains "Content-Type" set to "text/html",
    "body" set to "<h1>Hello</h1>"

# Or using helper:
Make response to html response with "<h1>Hello</h1>"
Make response to json response with task_data
```

#### 5. Template Rendering
```pohlang
Make template to """
<html>
  <h1>{{title}}</h1>
  <p>{{content}}</p>
</html>
"""

Make data to Dictionary contains
    "title" set to "Task Master",
    "content" set to "Welcome!"

Set html to render template with data
```

---

## 🎯 Phase 3: Minimal Web App Example

### File: `examples/web_hello.poh`
```pohlang
Start Program

Write "Creating web server..."

# Create server
Make server to Create web server on port 3000

# Add home route
Add route "/" with method "GET" to server:
    Make html to "<h1>Hello from PohLang!</h1><p>Welcome to the web!</p>"
    Return html response with html
End

# Add API route
Add route "/api/hello" with method "GET" to server:
    Make data to Dictionary contains
        "message" set to "Hello from PohLang API!",
        "version" set to "0.5.4"
    Return json response with data
End

Write "Starting server on http://localhost:3000"
Write "Press Ctrl+C to stop"

# Start server (blocking)
Start server

End Program
```

### Expected Output
```
Creating web server...
Starting server on http://localhost:3000
Press Ctrl+C to stop
🚀 Server listening on http://127.0.0.1:3000
```

Visit:
- `http://localhost:3000/` → See HTML page
- `http://localhost:3000/api/hello` → Get JSON response

---

## 🔧 Implementation Steps

### Step 1: Parser Changes

**File**: `runtime/src/parser/parser.rs`

Add new expression types:
```rust
pub enum Expression {
    // ... existing expressions ...
    
    // Web server expressions
    CreateWebServer(u16),                                    // port
    AddRoute(String, String, Box<Expression>),              // path, method, handler
    StartServer,
    HtmlResponse(Box<Expression>),                          // html content
    JsonResponse(Box<Expression>),                          // json data
    RenderTemplate(Box<Expression>, Box<Expression>),       // template, data
}
```

**File**: `runtime/src/parser/mod.rs`

Add parsing rules:
```rust
// Recognize "Create web server on port <number>"
if line.contains("Create web server on port") {
    let port = extract_port(&line)?;
    return Ok(Expression::CreateWebServer(port));
}

// Recognize "Add route <path> with method <method> to server"
if line.contains("Add route") && line.contains("with method") {
    let (path, method) = extract_route_info(&line)?;
    // Parse handler block...
    return Ok(Expression::AddRoute(path, method, handler));
}

// Recognize "Start server"
if line.trim() == "Start server" {
    return Ok(Expression::StartServer);
}
```

### Step 2: VM Integration

**File**: `runtime/src/vm/vm.rs`

Add evaluation cases:
```rust
Expression::CreateWebServer(port) => {
    let server = crate::stdlib::http::WebServer::new(*port);
    // Store server in VM state
    self.server = Some(server);
    Ok(Value::String("Server created".to_string()))
}

Expression::AddRoute(path, method, handler) => {
    if let Some(ref mut server) = self.server {
        let handler_fn = compile_handler(handler)?;
        server.add_route(path.clone(), method.clone(), Arc::new(handler_fn));
        Ok(Value::String("Route added".to_string()))
    } else {
        Err(anyhow!("No server created. Use 'Create web server' first."))
    }
}

Expression::StartServer => {
    if let Some(server) = self.server.take() {
        server.start()?; // Blocking
        Ok(Value::String("Server stopped".to_string()))
    } else {
        Err(anyhow!("No server to start"))
    }
}
```

### Step 3: Handler Compilation

Handlers need to:
1. Accept HttpRequest as input
2. Return HttpResponse as output

**Approach**:
```rust
fn compile_handler(expr: &Expression) -> Result<RouteHandler> {
    // Create a closure that evaluates the expression
    // with request data in scope
    Ok(Arc::new(move |request: HttpRequest| {
        // Set up VM with request data
        let mut vm = VM::new();
        vm.set_variable("request", request_to_value(&request));
        
        // Execute handler expression
        let result = vm.evaluate(expr)?;
        
        // Convert result to HttpResponse
        value_to_response(&result)
    }))
}
```

---

## 📦 Task Master Web App Structure

Once parser is ready, we can build the full app:

```
Task_Master_Web/
├── src/
│   ├── main.poh              # Web server entry point
│   ├── routes/
│   │   ├── home.poh          # Homepage route
│   │   ├── api_tasks.poh     # Task API routes
│   │   └── api_stats.poh     # Statistics API
│   └── storage.poh           # File-based storage
├── public/
│   ├── index.html            # Single-page app
│   ├── css/
│   │   └── style.css         # Modern styling
│   └── js/
│       └── app.js            # Frontend logic
├── templates/
│   └── layout.html           # Optional server-side rendering
└── data/
    └── tasks.json            # Task storage
```

### Backend: `src/main.poh`
```pohlang
Start Program

# Load tasks from file
Make tasks to load tasks from "data/tasks.json"

# Create server
Make server to Create web server on port 3000

# Serve homepage
Add route "/" with method "GET" to server:
    Set html to read file at "public/index.html"
    Return html response with html
End

# API: Get all tasks
Add route "/api/tasks" with method "GET" to server:
    Return json response with tasks
End

# API: Create task
Add route "/api/tasks" with method "POST" to server:
    Set task_data to parse json request["body"]
    Set new_id to (total of tasks) plus 1
    
    Make new_task to Dictionary contains
        "id" set to new_id,
        "title" set to task_data["title"],
        "description" set to task_data["description"],
        "status" set to "pending",
        "priority" set to task_data["priority"],
        "created" set to current time
    
    Use append new_task to tasks
    Use save tasks to "data/tasks.json"
    
    Return json response with new_task with status 201
End

# API: Complete task
Add route "/api/tasks/:id/complete" with method "PUT" to server:
    Set task_id to request["params"]["id"]
    # Find and update task...
    Return json response with updated_task
End

# API: Delete task
Add route "/api/tasks/:id" with method "DELETE" to server:
    Set task_id to request["params"]["id"]
    # Remove task...
    Return json response with Dictionary contains "deleted" set to True
End

Write "🚀 Task Master Web App running on http://localhost:3000"
Start server

End Program
```

### Frontend: `public/index.html`
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Task Master - Web App</title>
    <link rel="stylesheet" href="/css/style.css">
</head>
<body>
    <div class="container">
        <header>
            <h1>📝 Task Master</h1>
            <p>Your Personal Task Manager - Powered by PohLang</p>
        </header>

        <main>
            <!-- Add Task Form -->
            <section class="add-task">
                <h2>Add New Task</h2>
                <form id="add-task-form">
                    <input type="text" id="task-title" placeholder="Task title" required>
                    <textarea id="task-description" placeholder="Description"></textarea>
                    <select id="task-priority">
                        <option value="low">Low Priority</option>
                        <option value="medium" selected>Medium Priority</option>
                        <option value="high">High Priority</option>
                    </select>
                    <button type="submit">Add Task</button>
                </form>
            </section>

            <!-- Task List -->
            <section class="task-list">
                <div class="filters">
                    <button class="filter-btn active" data-filter="all">All</button>
                    <button class="filter-btn" data-filter="pending">Pending</button>
                    <button class="filter-btn" data-filter="completed">Completed</button>
                </div>

                <div id="tasks-container">
                    <!-- Tasks loaded dynamically -->
                </div>
            </section>

            <!-- Statistics -->
            <section class="statistics">
                <h2>Statistics</h2>
                <div class="stats-grid">
                    <div class="stat-card">
                        <span class="stat-value" id="total-tasks">0</span>
                        <span class="stat-label">Total Tasks</span>
                    </div>
                    <div class="stat-card">
                        <span class="stat-value" id="pending-tasks">0</span>
                        <span class="stat-label">Pending</span>
                    </div>
                    <div class="stat-card">
                        <span class="stat-value" id="completed-tasks">0</span>
                        <span class="stat-label">Completed</span>
                    </div>
                </div>
            </section>
        </main>
    </div>

    <script src="/js/app.js"></script>
</body>
</html>
```

### Frontend Logic: `public/js/app.js`
```javascript
// Task Master Web App - Frontend
const API_BASE = '/api';

// Load tasks on page load
document.addEventListener('DOMContentLoaded', () => {
    loadTasks();
    setupEventListeners();
});

// Load all tasks
async function loadTasks(filter = 'all') {
    try {
        const response = await fetch(`${API_BASE}/tasks`);
        const tasks = await response.json();
        
        const filteredTasks = filterTasks(tasks, filter);
        displayTasks(filteredTasks);
        updateStatistics(tasks);
    } catch (error) {
        console.error('Error loading tasks:', error);
        showError('Failed to load tasks');
    }
}

// Display tasks
function displayTasks(tasks) {
    const container = document.getElementById('tasks-container');
    
    if (tasks.length === 0) {
        container.innerHTML = '<p class="no-tasks">No tasks found</p>';
        return;
    }
    
    container.innerHTML = tasks.map(task => `
        <div class="task-card ${task.status} priority-${task.priority}">
            <div class="task-header">
                <h3>${task.title}</h3>
                <span class="priority-badge">${task.priority}</span>
            </div>
            <p class="task-description">${task.description || 'No description'}</p>
            <div class="task-actions">
                ${task.status === 'pending' ? 
                    `<button onclick="completeTask(${task.id})">✓ Complete</button>` :
                    '<span class="completed-badge">✓ Completed</span>'
                }
                <button onclick="deleteTask(${task.id})" class="delete-btn">🗑️ Delete</button>
            </div>
        </div>
    `).join('');
}

// Add new task
async function addTask(title, description, priority) {
    try {
        const response = await fetch(`${API_BASE}/tasks`, {
            method: 'POST',
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify({ title, description, priority })
        });
        
        if (response.ok) {
            loadTasks();
            showSuccess('Task added successfully!');
        }
    } catch (error) {
        console.error('Error adding task:', error);
        showError('Failed to add task');
    }
}

// Complete task
async function completeTask(id) {
    try {
        const response = await fetch(`${API_BASE}/tasks/${id}/complete`, {
            method: 'PUT'
        });
        
        if (response.ok) {
            loadTasks();
            showSuccess('Task completed!');
        }
    } catch (error) {
        console.error('Error completing task:', error);
        showError('Failed to complete task');
    }
}

// Delete task
async function deleteTask(id) {
    if (!confirm('Are you sure you want to delete this task?')) return;
    
    try {
        const response = await fetch(`${API_BASE}/tasks/${id}`, {
            method: 'DELETE'
        });
        
        if (response.ok) {
            loadTasks();
            showSuccess('Task deleted!');
        }
    } catch (error) {
        console.error('Error deleting task:', error);
        showError('Failed to delete task');
    }
}

// Filter tasks
function filterTasks(tasks, filter) {
    if (filter === 'all') return tasks;
    return tasks.filter(task => task.status === filter);
}

// Update statistics
function updateStatistics(tasks) {
    document.getElementById('total-tasks').textContent = tasks.length;
    document.getElementById('pending-tasks').textContent = 
        tasks.filter(t => t.status === 'pending').length;
    document.getElementById('completed-tasks').textContent = 
        tasks.filter(t => t.status === 'completed').length;
}

// Setup event listeners
function setupEventListeners() {
    // Add task form
    document.getElementById('add-task-form').addEventListener('submit', (e) => {
        e.preventDefault();
        const title = document.getElementById('task-title').value;
        const description = document.getElementById('task-description').value;
        const priority = document.getElementById('task-priority').value;
        
        addTask(title, description, priority);
        e.target.reset();
    });
    
    // Filter buttons
    document.querySelectorAll('.filter-btn').forEach(btn => {
        btn.addEventListener('click', () => {
            document.querySelectorAll('.filter-btn').forEach(b => b.classList.remove('active'));
            btn.classList.add('active');
            loadTasks(btn.dataset.filter);
        });
    });
}

// Show success message
function showSuccess(message) {
    // Implement toast notification
    console.log('✓', message);
}

// Show error message
function showError(message) {
    // Implement toast notification
    console.error('✗', message);
}
```

---

## 🎨 Next Steps

### Immediate (Parser Integration):
1. Add `CreateWebServer`, `AddRoute`, `StartServer` to parser
2. Update VM to handle web expressions
3. Test with simple "Hello World" web example

### Short-term (Basic Web App):
1. Create `examples/web_hello.poh` - minimal working example
2. Add static file serving
3. Test routing and response types

### Medium-term (Full Task Master):
1. Build complete Task Master web backend in PohLang
2. Create beautiful frontend with HTML/CSS/JS
3. Implement all CRUD operations via REST API
4. Add file persistence

### Long-term (Framework Features):
1. URL parameters: `/tasks/:id`
2. Middleware support (logging, auth)
3. WebSocket support for real-time updates
4. Session management
5. CORS handling
6. Static file directory serving

---

## 🚀 When Complete

Task Master will be accessible as:
- **Web App**: http://localhost:3000
- **API**: REST endpoints at `/api/*`
- **Cross-platform**: Works on any device with a browser
- **Real-time**: Updates without page refresh

This will make PohLang a **true full-stack language**! 🎉

---

**Status**: Runtime complete ✅ | Parser in progress 🚧 | Web app ready to build ⏳
