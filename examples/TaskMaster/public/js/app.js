// Task Master - Frontend JavaScript
let tasks = [];

// DOM Elements
const taskInput = document.getElementById('taskInput');
const addBtn = document.getElementById('addBtn');
const taskList = document.getElementById('taskList');
const taskCount = document.getElementById('taskCount');

// Load tasks on page load
document.addEventListener('DOMContentLoaded', () => {
    loadTasks();
    
    // Add task on button click
    addBtn.addEventListener('click', addTask);
    
    // Add task on Enter key
    taskInput.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') {
            addTask();
        }
    });
});

// Load tasks from backend
async function loadTasks() {
    try {
        const response = await fetch('/api/tasks');
        const data = await response.json();
        tasks = data.tasks || [];
        renderTasks();
    } catch (error) {
        console.error('Error loading tasks:', error);
    }
}

// Add new task
async function addTask() {
    const title = taskInput.value.trim();
    if (!title) return;
    
    try {
        const response = await fetch('/api/tasks', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ title })
        });
        
        const data = await response.json();
        if (data.success) {
            tasks.push(data.task);
            taskInput.value = '';
            renderTasks();
        }
    } catch (error) {
        console.error('Error adding task:', error);
    }
}

// Toggle task completion
function toggleTask(id) {
    const task = tasks.find(t => t.id === id);
    if (task) {
        task.completed = !task.completed;
        renderTasks();
    }
}

// Delete task
function deleteTask(id) {
    tasks = tasks.filter(t => t.id !== id);
    renderTasks();
}

// Render tasks to DOM
function renderTasks() {
    taskCount.textContent = tasks.length;
    
    if (tasks.length === 0) {
        taskList.innerHTML = '<li class="empty-state">No tasks yet. Add one above!</li>';
        return;
    }
    
    taskList.innerHTML = tasks.map(task => `
        <li class="task-item ${task.completed ? 'completed' : ''}">
            <input 
                type="checkbox" 
                class="task-checkbox"
                ${task.completed ? 'checked' : ''}
                onchange="toggleTask(${task.id})"
            />
            <span class="task-title">${escapeHtml(task.title)}</span>
            <button class="task-delete" onclick="deleteTask(${task.id})">Delete</button>
        </li>
    `).join('');
}

// Escape HTML to prevent XSS
function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}
