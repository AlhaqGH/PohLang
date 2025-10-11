/// Constant pool for bytecode
/// 
/// Stores literal values that are referenced by bytecode instructions.
/// Constants are deduplicated to save space.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A constant value in the constant pool
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Constant {
    /// Floating-point number
    Number(f64),
    
    /// String value
    String(String),
    
    /// Boolean value
    Boolean(bool),
    
    /// Null value
    Null,
}

impl Constant {
    /// Returns the size in bytes when serialized
    pub fn size(&self) -> usize {
        match self {
            Constant::Number(_) => 9,  // 1 byte tag + 8 bytes f64
            Constant::Boolean(_) => 2, // 1 byte tag + 1 byte bool
            Constant::Null => 1,       // 1 byte tag only
            Constant::String(s) => 5 + s.len(), // 1 byte tag + 4 bytes length + string
        }
    }
}

/// Constant pool managing all constant values
#[derive(Debug, Clone)]
pub struct ConstantPool {
    constants: Vec<Constant>,
    // For deduplication during compilation
    constant_map: HashMap<String, u32>,
}

impl ConstantPool {
    /// Create a new empty constant pool
    pub fn new() -> Self {
        Self {
            constants: Vec::new(),
            constant_map: HashMap::new(),
        }
    }
    
    /// Add a constant to the pool, returns its index
    /// Deduplicates identical constants
    pub fn add_constant(&mut self, constant: Constant) -> u32 {
        // Create a key for deduplication
        let key = match &constant {
            Constant::Number(n) => format!("n:{}", n),
            Constant::String(s) => format!("s:{}", s),
            Constant::Boolean(b) => format!("b:{}", b),
            Constant::Null => "null".to_string(),
        };
        
        // Check if we already have this constant
        if let Some(&idx) = self.constant_map.get(&key) {
            return idx;
        }
        
        // Add new constant
        let idx = self.constants.len() as u32;
        self.constants.push(constant);
        self.constant_map.insert(key, idx);
        idx
    }
    
    /// Get a constant by index
    pub fn get(&self, index: u32) -> Option<&Constant> {
        self.constants.get(index as usize)
    }
    
    /// Get the number of constants in the pool
    pub fn len(&self) -> usize {
        self.constants.len()
    }
    
    /// Check if the pool is empty
    pub fn is_empty(&self) -> bool {
        self.constants.is_empty()
    }
    
    /// Get all constants as a slice
    pub fn as_slice(&self) -> &[Constant] {
        &self.constants
    }
    
    /// Convert to a vector (for serialization)
    pub fn into_vec(self) -> Vec<Constant> {
        self.constants
    }
    
    /// Create from a vector (for deserialization)
    pub fn from_vec(constants: Vec<Constant>) -> Self {
        let mut pool = Self::new();
        pool.constants = constants;
        // Rebuild the map for deduplication
        for (idx, constant) in pool.constants.iter().enumerate() {
            let key = match constant {
                Constant::Number(n) => format!("n:{}", n),
                Constant::String(s) => format!("s:{}", s),
                Constant::Boolean(b) => format!("b:{}", b),
                Constant::Null => "null".to_string(),
            };
            pool.constant_map.insert(key, idx as u32);
        }
        pool
    }
}

impl Default for ConstantPool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add_constant() {
        let mut pool = ConstantPool::new();
        
        let idx1 = pool.add_constant(Constant::Number(42.0));
        let idx2 = pool.add_constant(Constant::String("hello".to_string()));
        let idx3 = pool.add_constant(Constant::Boolean(true));
        
        assert_eq!(idx1, 0);
        assert_eq!(idx2, 1);
        assert_eq!(idx3, 2);
        assert_eq!(pool.len(), 3);
    }
    
    #[test]
    fn test_deduplication() {
        let mut pool = ConstantPool::new();
        
        let idx1 = pool.add_constant(Constant::Number(42.0));
        let idx2 = pool.add_constant(Constant::Number(42.0)); // Same value
        
        assert_eq!(idx1, idx2); // Should return same index
        assert_eq!(pool.len(), 1); // Should only store once
    }
    
    #[test]
    fn test_get_constant() {
        let mut pool = ConstantPool::new();
        
        let idx = pool.add_constant(Constant::Number(3.14));
        let constant = pool.get(idx).unwrap();
        
        assert_eq!(*constant, Constant::Number(3.14));
    }
    
    #[test]
    fn test_constant_size() {
        assert_eq!(Constant::Number(42.0).size(), 9);
        assert_eq!(Constant::Boolean(true).size(), 2);
        assert_eq!(Constant::Null.size(), 1);
        assert_eq!(Constant::String("test".to_string()).size(), 9); // 5 + 4
    }
}
