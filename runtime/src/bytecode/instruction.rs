/// PohLang Bytecode Instruction Set
/// 
/// Stack-based bytecode instructions for the PohLang virtual machine.
/// Designed for simplicity, portability, and performance.

use std::fmt;

/// Bytecode instruction enum
/// 
/// All instructions operate on a value stack. The VM maintains:
/// - Value stack (operands)
/// - Call stack (function frames)
/// - Global variables (by name)
/// - Local variables (by index within frame)
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    // === Literals (Push constants) ===
    /// Push a constant from the constant pool onto the stack
    /// Operand: index into constant pool (u32)
    LoadConst(u32),
    
    /// Push boolean true onto the stack
    LoadTrue,
    
    /// Push boolean false onto the stack
    LoadFalse,
    
    /// Push null onto the stack
    LoadNull,
    
    // === Variables ===
    /// Load a local variable onto the stack
    /// Operand: local variable index (u32)
    LoadLocal(u32),
    
    /// Store top of stack to a local variable
    /// Operand: local variable index (u32)
    StoreLocal(u32),
    
    /// Load a global variable onto the stack
    /// Operand: variable name (String)
    LoadGlobal(String),
    
    /// Store top of stack to a global variable
    /// Operand: variable name (String)
    StoreGlobal(String),
    
    // === Arithmetic Operations ===
    /// Pop two values, add them, push result
    /// Stack: [... a b] -> [... a+b]
    Add,
    
    /// Pop two values, subtract second from first, push result
    /// Stack: [... a b] -> [... a-b]
    Subtract,
    
    /// Pop two values, multiply them, push result
    /// Stack: [... a b] -> [... a*b]
    Multiply,
    
    /// Pop two values, divide first by second, push result
    /// Stack: [... a b] -> [... a/b]
    Divide,
    
    /// Pop one value, negate it, push result
    /// Stack: [... a] -> [... -a]
    Negate,
    
    // === Comparison Operations ===
    /// Pop two values, push true if equal
    /// Stack: [... a b] -> [... a==b]
    Equal,
    
    /// Pop two values, push true if not equal
    /// Stack: [... a b] -> [... a!=b]
    NotEqual,
    
    /// Pop two values, push true if first > second
    /// Stack: [... a b] -> [... a>b]
    Greater,
    
    /// Pop two values, push true if first >= second
    /// Stack: [... a b] -> [... a>=b]
    GreaterEqual,
    
    /// Pop two values, push true if first < second
    /// Stack: [... a b] -> [... a<b]
    Less,
    
    /// Pop two values, push true if first <= second
    /// Stack: [... a b] -> [... a<=b]
    LessEqual,
    
    // === Logical Operations ===
    /// Pop two values, push logical AND
    /// Stack: [... a b] -> [... a&&b]
    And,
    
    /// Pop two values, push logical OR
    /// Stack: [... a b] -> [... a||b]
    Or,
    
    /// Pop one value, push logical NOT
    /// Stack: [... a] -> [... !a]
    Not,
    
    // === Control Flow ===
    /// Unconditional jump to offset
    /// Operand: instruction offset (i32, can be negative for loops)
    Jump(i32),
    
    /// Jump to offset if top of stack is false
    /// Stack: [... condition] -> [...]
    /// Operand: instruction offset (i32)
    JumpIfFalse(i32),
    
    /// Jump to offset if top of stack is true
    /// Stack: [... condition] -> [...]
    /// Operand: instruction offset (i32)
    JumpIfTrue(i32),
    
    /// Jump backward (for loops)
    /// Operand: instruction offset (i32, negative)
    Loop(i32),
    
    // === Functions ===
    /// Call a function with N arguments
    /// Stack: [... function arg1 arg2 ... argN] -> [... result]
    /// Operand: number of arguments (u8)
    Call(u8),
    
    /// Return from current function
    /// Stack: [... return_value] -> [... return_value] (in caller's frame)
    Return,
    
    // === Collections ===
    /// Build a list from N stack items
    /// Stack: [... item1 item2 ... itemN] -> [... list]
    /// Operand: number of items (u32)
    BuildList(u32),
    
    /// Build a dictionary from N*2 stack items (key-value pairs)
    /// Stack: [... k1 v1 k2 v2 ... kN vN] -> [... dict]
    /// Operand: number of pairs (u32)
    BuildDict(u32),
    
    /// Index into a collection
    /// Stack: [... collection index] -> [... collection[index]]
    Index,
    
    /// Store into a collection
    /// Stack: [... collection index value] -> [...]
    IndexStore,
    
    // === Error Handling ===
    /// Push a try handler onto the exception stack
    /// Operand: jump offset to catch handler (u32)
    PushTryHandler(u32),
    
    /// Pop a try handler from the exception stack
    PopTryHandler,
    
    /// Throw an exception
    /// Stack: [... error_value] -> (unwinds to catch handler)
    Throw,
    
    // === I/O Operations ===
    /// Pop a value and print it
    /// Stack: [... value] -> [...]
    Print,
    
    /// Read input from user, push as string
    /// Stack: [...] -> [... input_string]
    Input,
    
    /// Write to file
    /// Stack: [... content filepath] -> [...]
    WriteFile,
    
    /// Read from file
    /// Stack: [... filepath] -> [... content]
    ReadFile,
    
    // === Web Framework ===
    /// Create a web server
    /// Stack: [... port] -> [... server]
    CreateWebServer,
    
    /// Add route to server
    /// Stack: [... server path method handler] -> [...]
    AddRoute,
    
    /// Start web server (blocking)
    /// Stack: [... server] -> [...]
    StartServer,
    
    /// Create HTML response
    /// Stack: [... html_content] -> [... response]
    HtmlResponse,
    
    /// Create JSON response
    /// Stack: [... json_data] -> [... response]
    JsonResponse,
    
    // === Stack Manipulation ===
    /// Remove and discard the top stack item
    /// Stack: [... a] -> [...]
    Pop,
    
    /// Duplicate the top stack item
    /// Stack: [... a] -> [... a a]
    Duplicate,
    
    /// Swap the top two stack items
    /// Stack: [... a b] -> [... b a]
    Swap,
    
    // === Special ===
    /// Halt execution
    Halt,
}

impl Instruction {
    /// Returns the size in bytes of this instruction when encoded
    pub fn size(&self) -> usize {
        use Instruction::*;
        match self {
            // Instructions with no operands: 1 byte (opcode)
            LoadTrue | LoadFalse | LoadNull |
            Add | Subtract | Multiply | Divide | Negate |
            Equal | NotEqual | Greater | GreaterEqual | Less | LessEqual |
            And | Or | Not |
            Return | Index | IndexStore |
            Throw | Print | Input |
            CreateWebServer | StartServer | HtmlResponse | JsonResponse |
            Pop | Duplicate | Swap | Halt => 1,
            
            // Instructions with u8 operand: 2 bytes
            Call(_) => 2,
            
            // Instructions with u32 operand: 5 bytes
            LoadConst(_) | LoadLocal(_) | StoreLocal(_) |
            BuildList(_) | BuildDict(_) | PushTryHandler(_) => 5,
            
            // Pop try handler
            PopTryHandler => 1,
            
            // Instructions with i32 operand: 5 bytes
            Jump(_) | JumpIfFalse(_) | JumpIfTrue(_) | Loop(_) => 5,
            
            // Instructions with String operand: variable (4 bytes length + string bytes)
            LoadGlobal(s) | StoreGlobal(s) => 5 + s.len(),
            
            // File operations
            WriteFile | ReadFile => 1,
            
            // Route handling
            AddRoute => 1,
        }
    }
    
    /// Returns a human-readable name for this instruction
    pub fn name(&self) -> &'static str {
        use Instruction::*;
        match self {
            LoadConst(_) => "LoadConst",
            LoadTrue => "LoadTrue",
            LoadFalse => "LoadFalse",
            LoadNull => "LoadNull",
            LoadLocal(_) => "LoadLocal",
            StoreLocal(_) => "StoreLocal",
            LoadGlobal(_) => "LoadGlobal",
            StoreGlobal(_) => "StoreGlobal",
            Add => "Add",
            Subtract => "Subtract",
            Multiply => "Multiply",
            Divide => "Divide",
            Negate => "Negate",
            Equal => "Equal",
            NotEqual => "NotEqual",
            Greater => "Greater",
            GreaterEqual => "GreaterEqual",
            Less => "Less",
            LessEqual => "LessEqual",
            And => "And",
            Or => "Or",
            Not => "Not",
            Jump(_) => "Jump",
            JumpIfFalse(_) => "JumpIfFalse",
            JumpIfTrue(_) => "JumpIfTrue",
            Loop(_) => "Loop",
            Call(_) => "Call",
            Return => "Return",
            BuildList(_) => "BuildList",
            BuildDict(_) => "BuildDict",
            Index => "Index",
            IndexStore => "IndexStore",
            PushTryHandler(_) => "PushTryHandler",
            PopTryHandler => "PopTryHandler",
            Throw => "Throw",
            Print => "Print",
            Input => "Input",
            WriteFile => "WriteFile",
            ReadFile => "ReadFile",
            CreateWebServer => "CreateWebServer",
            AddRoute => "AddRoute",
            StartServer => "StartServer",
            HtmlResponse => "HtmlResponse",
            JsonResponse => "JsonResponse",
            Pop => "Pop",
            Duplicate => "Duplicate",
            Swap => "Swap",
            Halt => "Halt",
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Instruction::*;
        match self {
            LoadConst(idx) => write!(f, "LoadConst {}", idx),
            LoadLocal(idx) => write!(f, "LoadLocal {}", idx),
            StoreLocal(idx) => write!(f, "StoreLocal {}", idx),
            LoadGlobal(name) => write!(f, "LoadGlobal \"{}\"", name),
            StoreGlobal(name) => write!(f, "StoreGlobal \"{}\"", name),
            Jump(offset) => write!(f, "Jump {}", offset),
            JumpIfFalse(offset) => write!(f, "JumpIfFalse {}", offset),
            JumpIfTrue(offset) => write!(f, "JumpIfTrue {}", offset),
            Loop(offset) => write!(f, "Loop {}", offset),
            Call(argc) => write!(f, "Call {}", argc),
            BuildList(count) => write!(f, "BuildList {}", count),
            BuildDict(count) => write!(f, "BuildDict {}", count),
            PushTryHandler(offset) => write!(f, "PushTryHandler {}", offset),
            _ => write!(f, "{}", self.name()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_instruction_size() {
        assert_eq!(Instruction::Add.size(), 1);
        assert_eq!(Instruction::LoadConst(0).size(), 5);
        assert_eq!(Instruction::Call(3).size(), 2);
        assert_eq!(Instruction::LoadGlobal("x".to_string()).size(), 6); // 5 + 1
    }
    
    #[test]
    fn test_instruction_name() {
        assert_eq!(Instruction::Add.name(), "Add");
        assert_eq!(Instruction::LoadConst(5).name(), "LoadConst");
        assert_eq!(Instruction::Jump(10).name(), "Jump");
    }
    
    #[test]
    fn test_instruction_display() {
        assert_eq!(format!("{}", Instruction::Add), "Add");
        assert_eq!(format!("{}", Instruction::LoadConst(5)), "LoadConst 5");
        assert_eq!(format!("{}", Instruction::LoadGlobal("x".to_string())), "LoadGlobal \"x\"");
        assert_eq!(format!("{}", Instruction::Call(3)), "Call 3");
    }
}
