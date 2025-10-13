/// PohLang Bytecode Virtual Machine
///
/// Stack-based VM for executing bytecode instructions
use super::{BytecodeChunk, Constant, Instruction};
use std::fmt;

/// Runtime value representation
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
        }
    }
}

impl Value {
    /// Check if value is truthy
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Null => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
        }
    }

    /// Convert constant to value
    pub fn from_constant(constant: &Constant) -> Self {
        match constant {
            Constant::Number(n) => Value::Number(*n),
            Constant::String(s) => Value::String(s.clone()),
            Constant::Boolean(b) => Value::Boolean(*b),
            Constant::Null => Value::Null,
        }
    }
}

/// VM runtime errors
#[derive(Debug, Clone)]
pub enum VMError {
    StackUnderflow,
    StackOverflow,
    InvalidConstantIndex(u32),
    InvalidLocalIndex(u32),
    InvalidInstruction,
    InvalidJump(i32),
    TypeError(String),
    DivisionByZero,
    Other(String),
}

impl fmt::Display for VMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VMError::StackUnderflow => write!(f, "Stack underflow"),
            VMError::StackOverflow => write!(f, "Stack overflow (max 1024 values)"),
            VMError::InvalidConstantIndex(idx) => write!(f, "Invalid constant index: {}", idx),
            VMError::InvalidLocalIndex(idx) => write!(f, "Invalid local variable index: {}", idx),
            VMError::InvalidInstruction => write!(f, "Invalid instruction"),
            VMError::InvalidJump(offset) => write!(f, "Invalid jump offset: {}", offset),
            VMError::TypeError(msg) => write!(f, "Type error: {}", msg),
            VMError::DivisionByZero => write!(f, "Division by zero"),
            VMError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for VMError {}

pub type VMResult<T> = Result<T, VMError>;

/// Call frame for function calls
#[derive(Debug, Clone)]
struct CallFrame {
    return_ip: usize,
    locals_base: usize,
}

/// Bytecode Virtual Machine
pub struct BytecodeVM {
    /// Value stack
    stack: Vec<Value>,

    /// Local variables
    locals: Vec<Value>,

    /// Call frames
    call_stack: Vec<CallFrame>,

    /// Instruction pointer
    ip: usize,

    /// Current chunk being executed
    chunk: Option<BytecodeChunk>,

    /// Output buffer (for testing)
    pub output: Vec<String>,
}

impl BytecodeVM {
    /// Create a new VM
    pub fn new() -> Self {
        Self {
            stack: Vec::with_capacity(256),
            locals: vec![Value::Null; 256],
            call_stack: Vec::new(),
            ip: 0,
            chunk: None,
            output: Vec::new(),
        }
    }

    /// Load a bytecode chunk and prepare for execution
    pub fn load(&mut self, chunk: BytecodeChunk) {
        self.chunk = Some(chunk);
        self.ip = 0;
        self.stack.clear();
        self.output.clear();
    }

    /// Execute the loaded bytecode chunk
    pub fn run(&mut self) -> VMResult<Value> {
        if self.chunk.is_none() {
            return Err(VMError::Other("No bytecode loaded".to_string()));
        }

        loop {
            // Check if we've reached the end
            let chunk = self.chunk.as_ref().unwrap();
            if self.ip >= chunk.code.len() {
                // Return top of stack or null
                return Ok(self.pop().unwrap_or(Value::Null));
            }

            // Fetch and execute instruction
            let instruction = &chunk.code[self.ip].clone();
            self.ip += 1;

            match self.execute_instruction(instruction) {
                Ok(Some(value)) => return Ok(value), // Return instruction hit
                Ok(None) => continue,                // Continue execution
                Err(e) => return Err(e),
            }
        }
    }

    /// Execute a single instruction
    /// Returns Some(value) if a Return instruction is hit, None otherwise
    fn execute_instruction(&mut self, instruction: &Instruction) -> VMResult<Option<Value>> {
        match instruction {
            // === Literals ===
            Instruction::LoadConst(idx) => {
                let chunk = self.chunk.as_ref().unwrap();
                let constant = chunk
                    .constants
                    .get(*idx as usize)
                    .ok_or(VMError::InvalidConstantIndex(*idx))?;
                let value = Value::from_constant(constant);
                self.push(value)?;
            }

            Instruction::LoadTrue => {
                self.push(Value::Boolean(true))?;
            }

            Instruction::LoadFalse => {
                self.push(Value::Boolean(false))?;
            }

            Instruction::LoadNull => {
                self.push(Value::Null)?;
            }

            // === Variables ===
            Instruction::LoadLocal(idx) => {
                let value = self
                    .locals
                    .get(*idx as usize)
                    .ok_or(VMError::InvalidLocalIndex(*idx))?
                    .clone();
                self.push(value)?;
            }

            Instruction::StoreLocal(idx) => {
                let value = self.pop()?;
                if (*idx as usize) < self.locals.len() {
                    self.locals[*idx as usize] = value;
                } else {
                    return Err(VMError::InvalidLocalIndex(*idx));
                }
            }

            // === Arithmetic Operations ===
            Instruction::Add => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = match (a, b) {
                    (Value::Number(x), Value::Number(y)) => Value::Number(x + y),
                    (Value::String(x), Value::String(y)) => Value::String(format!("{}{}", x, y)),
                    _ => {
                        return Err(VMError::TypeError(
                            "Add requires numbers or strings".to_string(),
                        ))
                    }
                };
                self.push(result)?;
            }

            Instruction::Subtract => {
                let b = self.pop()?;
                let a = self.pop()?;
                match (a, b) {
                    (Value::Number(x), Value::Number(y)) => self.push(Value::Number(x - y))?,
                    _ => return Err(VMError::TypeError("Subtract requires numbers".to_string())),
                }
            }

            Instruction::Multiply => {
                let b = self.pop()?;
                let a = self.pop()?;
                match (a, b) {
                    (Value::Number(x), Value::Number(y)) => self.push(Value::Number(x * y))?,
                    _ => return Err(VMError::TypeError("Multiply requires numbers".to_string())),
                }
            }

            Instruction::Divide => {
                let b = self.pop()?;
                let a = self.pop()?;
                match (a, b) {
                    (Value::Number(x), Value::Number(y)) => {
                        if y == 0.0 {
                            return Err(VMError::DivisionByZero);
                        }
                        self.push(Value::Number(x / y))?
                    }
                    _ => return Err(VMError::TypeError("Divide requires numbers".to_string())),
                }
            }

            Instruction::Negate => {
                let value = self.pop()?;
                match value {
                    Value::Number(n) => self.push(Value::Number(-n))?,
                    _ => return Err(VMError::TypeError("Negate requires a number".to_string())),
                }
            }

            // === Comparison Operations ===
            Instruction::Equal => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(Value::Boolean(a == b))?;
            }

            Instruction::NotEqual => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(Value::Boolean(a != b))?;
            }

            Instruction::Less => {
                let b = self.pop()?;
                let a = self.pop()?;
                match (a, b) {
                    (Value::Number(x), Value::Number(y)) => self.push(Value::Boolean(x < y))?,
                    _ => return Err(VMError::TypeError("Less requires numbers".to_string())),
                }
            }

            Instruction::LessEqual => {
                let b = self.pop()?;
                let a = self.pop()?;
                match (a, b) {
                    (Value::Number(x), Value::Number(y)) => self.push(Value::Boolean(x <= y))?,
                    _ => return Err(VMError::TypeError("LessEqual requires numbers".to_string())),
                }
            }

            Instruction::Greater => {
                let b = self.pop()?;
                let a = self.pop()?;
                match (a, b) {
                    (Value::Number(x), Value::Number(y)) => self.push(Value::Boolean(x > y))?,
                    _ => return Err(VMError::TypeError("Greater requires numbers".to_string())),
                }
            }

            Instruction::GreaterEqual => {
                let b = self.pop()?;
                let a = self.pop()?;
                match (a, b) {
                    (Value::Number(x), Value::Number(y)) => self.push(Value::Boolean(x >= y))?,
                    _ => {
                        return Err(VMError::TypeError(
                            "GreaterEqual requires numbers".to_string(),
                        ))
                    }
                }
            }

            // === Logical Operations ===
            Instruction::Not => {
                let value = self.pop()?;
                self.push(Value::Boolean(!value.is_truthy()))?;
            }

            Instruction::And => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(Value::Boolean(a.is_truthy() && b.is_truthy()))?;
            }

            Instruction::Or => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(Value::Boolean(a.is_truthy() || b.is_truthy()))?;
            }

            // === Control Flow ===
            Instruction::Jump(offset) => {
                self.ip = (self.ip as i32 + offset) as usize;
            }

            Instruction::JumpIfFalse(offset) => {
                let value = self.pop()?;
                if !value.is_truthy() {
                    self.ip = (self.ip as i32 + offset) as usize;
                }
            }

            Instruction::Loop(offset) => {
                self.ip = (self.ip as i32 - offset) as usize;
            }

            Instruction::Call(argc) => {
                // For now, we'll just pop the arguments and function
                // Full implementation would require function objects
                for _ in 0..*argc {
                    self.pop()?;
                }
                self.pop()?; // Pop function
                self.push(Value::Null)?; // Push null as return value for now
            }

            Instruction::Return => {
                let return_value = if !self.stack.is_empty() {
                    self.pop()?
                } else {
                    Value::Null
                };
                return Ok(Some(return_value));
            }

            // === Stack Operations ===
            Instruction::Pop => {
                self.pop()?;
            }

            // === I/O Operations ===
            Instruction::Print => {
                let value = self.pop()?;
                let output = value.to_string();
                self.output.push(output.clone());
                println!("{}", output);
            }

            Instruction::Input => {
                // For now, we'll push null (in real implementation, would read from stdin)
                self.push(Value::Null)?;
            }

            // === Other instructions (not yet implemented) ===
            Instruction::Halt => {
                return Ok(Some(Value::Null));
            }

            _ => {
                return Err(VMError::Other(format!(
                    "Unimplemented instruction: {:?}",
                    instruction
                )));
            }
        }

        Ok(None)
    }

    /// Push a value onto the stack
    fn push(&mut self, value: Value) -> VMResult<()> {
        if self.stack.len() >= 1024 {
            return Err(VMError::StackOverflow);
        }
        self.stack.push(value);
        Ok(())
    }

    /// Pop a value from the stack
    fn pop(&mut self) -> VMResult<Value> {
        self.stack.pop().ok_or(VMError::StackUnderflow)
    }

    /// Peek at the top of the stack without popping
    #[allow(dead_code)]
    fn peek(&self) -> VMResult<&Value> {
        self.stack.last().ok_or(VMError::StackUnderflow)
    }

    /// Get the current output buffer
    pub fn get_output(&self) -> Vec<String> {
        self.output.clone()
    }

    /// Clear the output buffer
    pub fn clear_output(&mut self) {
        self.output.clear();
    }
}

impl Default for BytecodeVM {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_push_pop() {
        let mut vm = BytecodeVM::new();
        vm.push(Value::Number(42.0)).unwrap();
        vm.push(Value::String("hello".to_string())).unwrap();

        assert_eq!(vm.pop().unwrap(), Value::String("hello".to_string()));
        assert_eq!(vm.pop().unwrap(), Value::Number(42.0));
    }

    #[test]
    fn test_vm_load_const() {
        let mut chunk = BytecodeChunk::new(1);
        chunk.constants.push(Constant::Number(42.0));
        chunk.code.push(Instruction::LoadConst(0));
        chunk.code.push(Instruction::Return);

        let mut vm = BytecodeVM::new();
        vm.load(chunk);
        let result = vm.run().unwrap();

        assert_eq!(result, Value::Number(42.0));
    }

    #[test]
    fn test_vm_arithmetic() {
        let mut chunk = BytecodeChunk::new(1);
        chunk.constants.push(Constant::Number(10.0));
        chunk.constants.push(Constant::Number(20.0));
        chunk.code.push(Instruction::LoadConst(0));
        chunk.code.push(Instruction::LoadConst(1));
        chunk.code.push(Instruction::Add);
        chunk.code.push(Instruction::Return);

        let mut vm = BytecodeVM::new();
        vm.load(chunk);
        let result = vm.run().unwrap();

        assert_eq!(result, Value::Number(30.0));
    }

    #[test]
    fn test_vm_variables() {
        let mut chunk = BytecodeChunk::new(1);
        chunk.constants.push(Constant::Number(42.0));
        chunk.code.push(Instruction::LoadConst(0));
        chunk.code.push(Instruction::StoreLocal(0));
        chunk.code.push(Instruction::LoadLocal(0));
        chunk.code.push(Instruction::Return);

        let mut vm = BytecodeVM::new();
        vm.load(chunk);
        let result = vm.run().unwrap();

        assert_eq!(result, Value::Number(42.0));
    }

    #[test]
    fn test_vm_comparison() {
        let mut chunk = BytecodeChunk::new(1);
        chunk.constants.push(Constant::Number(10.0));
        chunk.constants.push(Constant::Number(20.0));
        chunk.code.push(Instruction::LoadConst(0));
        chunk.code.push(Instruction::LoadConst(1));
        chunk.code.push(Instruction::Less);
        chunk.code.push(Instruction::Return);

        let mut vm = BytecodeVM::new();
        vm.load(chunk);
        let result = vm.run().unwrap();

        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_vm_print() {
        let mut chunk = BytecodeChunk::new(1);
        chunk
            .constants
            .push(Constant::String("Hello, VM!".to_string()));
        chunk.code.push(Instruction::LoadConst(0));
        chunk.code.push(Instruction::Print);
        chunk.code.push(Instruction::Return);

        let mut vm = BytecodeVM::new();
        vm.load(chunk);
        vm.run().unwrap();

        assert_eq!(vm.output, vec!["Hello, VM!"]);
    }
}
