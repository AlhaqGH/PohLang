use anyhow::{anyhow, Result};
use serde_json::{Value as JsonValue, Map};

/// Parses a JSON string into a structured value
/// 
/// # Arguments
/// * `json_str` - The JSON string to parse
/// 
/// # Returns
/// * `Ok(JsonValue)` - The parsed JSON value
/// * `Err` - If the JSON is invalid
/// 
/// # Example
/// ```
/// let json = parse_json(r#"{"name": "Alice", "age": 25}"#)?;
/// ```
pub fn parse_json(json_str: &str) -> Result<JsonValue> {
    serde_json::from_str(json_str)
        .map_err(|e| anyhow!("Failed to parse JSON: {}", e))
}

/// Converts a value to a JSON string
/// 
/// # Arguments
/// * `value` - The JSON value to stringify
/// 
/// # Returns
/// * `Ok(String)` - The JSON string representation
/// * `Err` - If serialization fails
/// 
/// # Example
/// ```
/// let json_str = json_stringify(&json_value)?;
/// ```
pub fn json_stringify(value: &JsonValue) -> Result<String> {
    serde_json::to_string(value)
        .map_err(|e| anyhow!("Failed to stringify JSON: {}", e))
}

/// Converts a value to a pretty-printed JSON string
/// 
/// # Arguments
/// * `value` - The JSON value to stringify
/// 
/// # Returns
/// * `Ok(String)` - The pretty-printed JSON string
/// * `Err` - If serialization fails
pub fn json_stringify_pretty(value: &JsonValue) -> Result<String> {
    serde_json::to_string_pretty(value)
        .map_err(|e| anyhow!("Failed to stringify JSON: {}", e))
}

/// Gets a value from a JSON object by key
/// 
/// # Arguments
/// * `json` - The JSON object
/// * `key` - The key to look up
/// 
/// # Returns
/// * `Ok(JsonValue)` - The value at the key
/// * `Err` - If the key doesn't exist or JSON is not an object
pub fn json_get(json: &JsonValue, key: &str) -> Result<JsonValue> {
    json.get(key)
        .cloned()
        .ok_or_else(|| anyhow!("Key '{}' not found in JSON object", key))
}

/// Sets a value in a JSON object
/// 
/// # Arguments
/// * `json` - The JSON object to modify
/// * `key` - The key to set
/// * `value` - The value to set
/// 
/// # Returns
/// * `Ok(())` - If successful
/// * `Err` - If JSON is not an object
pub fn json_set(json: &mut JsonValue, key: String, value: JsonValue) -> Result<()> {
    if let Some(obj) = json.as_object_mut() {
        obj.insert(key, value);
        Ok(())
    } else {
        Err(anyhow!("Cannot set key on non-object JSON value"))
    }
}

/// Creates a new empty JSON object
pub fn json_object_new() -> JsonValue {
    JsonValue::Object(Map::new())
}

/// Creates a new empty JSON array
pub fn json_array_new() -> JsonValue {
    JsonValue::Array(Vec::new())
}

/// Adds an item to a JSON array
/// 
/// # Arguments
/// * `json` - The JSON array to modify
/// * `value` - The value to add
/// 
/// # Returns
/// * `Ok(())` - If successful
/// * `Err` - If JSON is not an array
pub fn json_array_push(json: &mut JsonValue, value: JsonValue) -> Result<()> {
    if let Some(arr) = json.as_array_mut() {
        arr.push(value);
        Ok(())
    } else {
        Err(anyhow!("Cannot push to non-array JSON value"))
    }
}

/// Gets the length of a JSON array or object
/// 
/// # Arguments
/// * `json` - The JSON value
/// 
/// # Returns
/// * `Ok(usize)` - The length
/// * `Err` - If JSON is not an array or object
pub fn json_length(json: &JsonValue) -> Result<usize> {
    match json {
        JsonValue::Array(arr) => Ok(arr.len()),
        JsonValue::Object(obj) => Ok(obj.len()),
        _ => Err(anyhow!("Cannot get length of JSON value that is not array or object")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_json_object() {
        let json_str = r#"{"name": "Alice", "age": 30}"#;
        let result = parse_json(json_str);
        assert!(result.is_ok());
        
        let value = result.unwrap();
        assert!(value.is_object());
        assert_eq!(value["name"], "Alice");
        assert_eq!(value["age"], 30);
    }
    
    #[test]
    fn test_parse_json_array() {
        let json_str = r#"[1, 2, 3, 4, 5]"#;
        let result = parse_json(json_str);
        assert!(result.is_ok());
        
        let value = result.unwrap();
        assert!(value.is_array());
        assert_eq!(value.as_array().unwrap().len(), 5);
    }
    
    #[test]
    fn test_parse_json_invalid() {
        let json_str = r#"{"invalid": }"#;
        let result = parse_json(json_str);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_json_stringify() {
        let json_str = r#"{"name":"Bob","age":25}"#;
        let value = parse_json(json_str).unwrap();
        let result = json_stringify(&value);
        assert!(result.is_ok());
        
        let stringified = result.unwrap();
        assert!(stringified.contains("Bob"));
        assert!(stringified.contains("25"));
    }
    
    #[test]
    fn test_json_stringify_pretty() {
        let json_str = r#"{"name":"Carol","age":35}"#;
        let value = parse_json(json_str).unwrap();
        let result = json_stringify_pretty(&value);
        assert!(result.is_ok());
        
        let pretty = result.unwrap();
        assert!(pretty.contains("Carol"));
        assert!(pretty.contains("\n")); // Pretty print should have newlines
    }
    
    #[test]
    fn test_json_roundtrip() {
        let original = r#"{"items":[1,2,3],"status":"ok"}"#;
        let parsed = parse_json(original).unwrap();
        let stringified = json_stringify(&parsed).unwrap();
        let reparsed = parse_json(&stringified).unwrap();
        
        assert_eq!(parsed, reparsed);
    }
    
    #[test]
    fn test_json_get() {
        let json_str = r#"{"name":"Dave","age":40}"#;
        let value = parse_json(json_str).unwrap();
        let name = json_get(&value, "name");
        assert!(name.is_ok());
        assert_eq!(name.unwrap(), "Dave");
    }
    
    #[test]
    fn test_json_get_missing_key() {
        let json_str = r#"{"name":"Eve"}"#;
        let value = parse_json(json_str).unwrap();
        let age = json_get(&value, "age");
        assert!(age.is_err());
    }
    
    #[test]
    fn test_json_set() {
        let json_str = r#"{"name":"Frank"}"#;
        let mut value = parse_json(json_str).unwrap();
        let result = json_set(&mut value, "age".to_string(), JsonValue::from(45));
        assert!(result.is_ok());
        assert_eq!(value["age"], 45);
    }
    
    #[test]
    fn test_json_object_new() {
        let obj = json_object_new();
        assert!(obj.is_object());
        assert_eq!(json_length(&obj).unwrap(), 0);
    }
    
    #[test]
    fn test_json_array_new() {
        let arr = json_array_new();
        assert!(arr.is_array());
        assert_eq!(json_length(&arr).unwrap(), 0);
    }
    
    #[test]
    fn test_json_array_push() {
        let mut arr = json_array_new();
        let result = json_array_push(&mut arr, JsonValue::from(1));
        assert!(result.is_ok());
        assert_eq!(json_length(&arr).unwrap(), 1);
        assert_eq!(arr[0], 1);
    }
    
    #[test]
    fn test_json_length_array() {
        let json_str = r#"[1, 2, 3]"#;
        let value = parse_json(json_str).unwrap();
        let len = json_length(&value);
        assert!(len.is_ok());
        assert_eq!(len.unwrap(), 3);
    }
    
    #[test]
    fn test_json_length_object() {
        let json_str = r#"{"a":1,"b":2}"#;
        let value = parse_json(json_str).unwrap();
        let len = json_length(&value);
        assert!(len.is_ok());
        assert_eq!(len.unwrap(), 2);
    }
}
