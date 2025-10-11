use anyhow::{anyhow, Result};
use serde_json::Value as JsonValue;

/// Simple template engine for HTML templating
/// Supports {{variable}} syntax for variable substitution
///
/// # Example Template
/// ```html
/// <h1>{{title}}</h1>
/// <p>{{content}}</p>
/// ```

/// Renders a template with variable substitution
///
/// # Arguments
/// * `template` - The template string with {{variable}} placeholders
/// * `data` - JSON object containing variable values
///
/// # Returns
/// * `Ok(String)` - The rendered template
/// * `Err` - If rendering fails
///
/// # Example
/// ```
/// let template = "<h1>{{title}}</h1>";
/// let data = json!({"title": "Hello World"});
/// let rendered = render_template(template, &data)?;
/// ```
pub fn render_template(template: &str, data: &JsonValue) -> Result<String> {
    let mut result = template.to_string();
    
    // Extract all variables from template
    let vars = extract_variables(template);
    
    // Replace each variable with its value from data
    for var in vars {
        let placeholder = format!("{{{{{}}}}}", var);
        let value = get_nested_value(data, &var);
        result = result.replace(&placeholder, &value);
    }
    
    Ok(result)
}

/// Extracts variable names from template
/// Finds all {{variable}} patterns
fn extract_variables(template: &str) -> Vec<String> {
    let mut vars = Vec::new();
    let mut chars = template.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '{' {
            if let Some(&next) = chars.peek() {
                if next == '{' {
                    chars.next(); // consume second {
                    
                    // Read variable name until }}
                    let mut var_name = String::new();
                    let mut found_closing = false;
                    
                    while let Some(c) = chars.next() {
                        if c == '}' {
                            if let Some(&next_c) = chars.peek() {
                                if next_c == '}' {
                                    chars.next(); // consume second }
                                    found_closing = true;
                                    break;
                                }
                            }
                        }
                        var_name.push(c);
                    }
                    
                    if found_closing {
                        vars.push(var_name.trim().to_string());
                    }
                }
            }
        }
    }
    
    vars
}

/// Gets a value from JSON data, supporting nested paths with dot notation
/// e.g., "user.name" -> data["user"]["name"]
fn get_nested_value(data: &JsonValue, path: &str) -> String {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current = data;
    
    for part in &parts {
        current = match current.get(part) {
            Some(val) => val,
            None => return String::new(),
        };
    }
    
    // Convert JSON value to string
    match current {
        JsonValue::String(s) => s.clone(),
        JsonValue::Number(n) => n.to_string(),
        JsonValue::Bool(b) => b.to_string(),
        JsonValue::Null => String::new(),
        _ => current.to_string(),
    }
}

/// Renders a template with a list of items (for loops)
///
/// # Arguments
/// * `template` - Template with {{#each items}}...{{/each}} blocks
/// * `data` - JSON object with array data
///
/// # Example
/// ```
/// let template = "{{#each users}}<p>{{name}}</p>{{/each}}";
/// let data = json!({"users": [{"name": "Alice"}, {"name": "Bob"}]});
/// ```
pub fn render_template_with_loops(template: &str, data: &JsonValue) -> Result<String> {
    let mut result = template.to_string();
    
    // Find and process {{#each}} blocks
    while let Some(start) = result.find("{{#each ") {
        let after_start = &result[start + 8..];
        let end_of_tag = after_start.find("}}").ok_or_else(|| anyhow!("Unclosed {{#each tag"))?;
        let array_name = after_start[..end_of_tag].trim();
        
        let block_start = start + 8 + end_of_tag + 2;
        let end_tag = "{{/each}}".to_string();
        let block_end = result[block_start..].find(&end_tag)
            .ok_or_else(|| anyhow!("Missing {{/each}} tag"))?;
        
        let block_template = &result[block_start..block_start + block_end];
        let full_block_end = block_start + block_end + end_tag.len();
        
        // Get array from data
        let array = data.get(array_name)
            .and_then(|v| v.as_array())
            .ok_or_else(|| anyhow!("Array '{}' not found", array_name))?;
        
        // Render block for each item
        let mut rendered_items = String::new();
        for item in array {
            let rendered = render_template(block_template, item)?;
            rendered_items.push_str(&rendered);
        }
        
        // Replace the entire {{#each}}...{{/each}} block
        result.replace_range(start..full_block_end, &rendered_items);
    }
    
    // Now render remaining variables
    render_template(&result, data)
}

/// Renders a template with conditionals
///
/// # Example
/// ```
/// let template = "{{#if show}}<p>Visible</p>{{/if}}";
/// let data = json!({"show": true});
/// ```
pub fn render_template_with_conditionals(template: &str, data: &JsonValue) -> Result<String> {
    let mut result = template.to_string();
    
    // Find and process {{#if}} blocks
    while let Some(start) = result.find("{{#if ") {
        let after_start = &result[start + 6..];
        let end_of_tag = after_start.find("}}").ok_or_else(|| anyhow!("Unclosed {{#if tag"))?;
        let condition_name = after_start[..end_of_tag].trim();
        
        let block_start = start + 6 + end_of_tag + 2;
        let end_tag = "{{/if}}".to_string();
        let block_end = result[block_start..].find(&end_tag)
            .ok_or_else(|| anyhow!("Missing {{/if}} tag"))?;
        
        let block_template = &result[block_start..block_start + block_end];
        let full_block_end = block_start + block_end + end_tag.len();
        
        // Evaluate condition
        let condition_value = data.get(condition_name)
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        
        let replacement = if condition_value {
            render_template(block_template, data)?
        } else {
            String::new()
        };
        
        result.replace_range(start..full_block_end, &replacement);
    }
    
    Ok(result)
}

/// Full template rendering with all features
pub fn render_full(template: &str, data: &JsonValue) -> Result<String> {
    let with_conditionals = render_template_with_conditionals(template, data)?;
    let with_loops = render_template_with_loops(&with_conditionals, data)?;
    render_template(&with_loops, data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_simple_template() {
        let template = "<h1>{{title}}</h1>";
        let data = json!({"title": "Hello World"});
        let result = render_template(template, &data).unwrap();
        assert_eq!(result, "<h1>Hello World</h1>");
    }

    #[test]
    fn test_multiple_variables() {
        let template = "<h1>{{title}}</h1><p>{{content}}</p>";
        let data = json!({
            "title": "Welcome",
            "content": "This is a test"
        });
        let result = render_template(template, &data).unwrap();
        assert!(result.contains("Welcome"));
        assert!(result.contains("This is a test"));
    }

    #[test]
    fn test_nested_values() {
        let template = "<p>{{user.name}}</p>";
        let data = json!({
            "user": {
                "name": "Alice"
            }
        });
        let result = render_template(template, &data).unwrap();
        assert_eq!(result, "<p>Alice</p>");
    }

    #[test]
    fn test_extract_variables() {
        let template = "{{name}} is {{age}} years old";
        let vars = extract_variables(template);
        assert_eq!(vars.len(), 2);
        assert!(vars.contains(&"name".to_string()));
        assert!(vars.contains(&"age".to_string()));
    }

    #[test]
    fn test_missing_variable() {
        let template = "<p>{{missing}}</p>";
        let data = json!({"present": "value"});
        let result = render_template(template, &data).unwrap();
        assert_eq!(result, "<p></p>");
    }

    #[test]
    fn test_number_rendering() {
        let template = "<p>Count: {{count}}</p>";
        let data = json!({"count": 42});
        let result = render_template(template, &data).unwrap();
        assert_eq!(result, "<p>Count: 42</p>");
    }

    #[test]
    fn test_boolean_rendering() {
        let template = "<p>Active: {{active}}</p>";
        let data = json!({"active": true});
        let result = render_template(template, &data).unwrap();
        assert_eq!(result, "<p>Active: true</p>");
    }
}
