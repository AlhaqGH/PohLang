/// PohLang Bytecode Module
/// 
/// This module contains the bytecode compiler, VM, and related utilities
/// for compiling and executing PohLang programs as bytecode.

pub mod instruction;
pub mod constant;
pub mod compiler;
pub mod vm;

pub use instruction::Instruction;
pub use constant::{Constant, ConstantPool};
pub use compiler::{Compiler, CompilerError, CompileResult};
pub use vm::{BytecodeVM, Value, VMError, VMResult};

/// Bytecode chunk containing instructions and constants
#[derive(Debug, Clone)]
pub struct BytecodeChunk {
    /// Bytecode format version
    pub version: u32,
    
    /// Constant pool
    pub constants: Vec<Constant>,
    
    /// Bytecode instructions
    pub code: Vec<Instruction>,
    
    /// Optional debug information
    pub debug_info: Option<DebugInfo>,
}

/// Debug information for bytecode
#[derive(Debug, Clone)]
pub struct DebugInfo {
    /// Source file name
    pub source_file: String,
    
    /// Line number for each instruction
    pub line_numbers: Vec<u32>,
    
    /// Variable names (local index -> name)
    pub variable_names: Vec<String>,
}

impl BytecodeChunk {
    /// Create a new bytecode chunk with specified version
    pub fn new(version: u32) -> Self {
        Self {
            version,
            constants: Vec::new(),
            code: Vec::new(),
            debug_info: None,
        }
    }
    
    /// Get the number of instructions
    pub fn instruction_count(&self) -> usize {
        self.code.len()
    }
    
    /// Get the size in bytes (approximate)
    pub fn size_bytes(&self) -> usize {
        let const_size: usize = self.constants.iter().map(|c| c.size()).sum();
        let code_size: usize = self.code.iter().map(|i| i.size()).sum();
        12 + const_size + code_size // 12 bytes for header
    }
}

impl Default for BytecodeChunk {
    fn default() -> Self {
        Self::new(1) // Default to version 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bytecode_chunk_creation() {
        let chunk = BytecodeChunk::new(1);
        assert_eq!(chunk.version, 1);
        assert_eq!(chunk.instruction_count(), 0);
        assert!(chunk.debug_info.is_none());
    }
    
    #[test]
    fn test_bytecode_chunk_with_code() {
        let mut chunk = BytecodeChunk::new(1);
        chunk.constants.push(Constant::Number(42.0));
        chunk.code.push(Instruction::LoadConst(0));
        chunk.code.push(Instruction::Print);
        chunk.code.push(Instruction::Halt);
        
        assert_eq!(chunk.instruction_count(), 3);
        assert!(chunk.size_bytes() > 0);
    }
}
