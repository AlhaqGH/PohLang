/// PohLang Bytecode Virtual Machine
///
/// Stack-based VM for executing bytecode instructions
use super::{BytecodeChunk, Constant, Instruction};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::time::{Duration, Instant};

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

/// Cache entry for inline caching of global lookups
#[derive(Debug, Clone)]
struct CacheEntry {
    key_hash: u64,
    version: u64,
    value: Value,
}

const GLOBAL_CACHE_SIZE: usize = 256;

/// VM execution statistics for profiling and optimization
#[derive(Debug, Clone)]
pub struct VMStats {
    /// Total instructions executed
    pub total_instructions: u64,

    /// Execution time
    pub execution_time: Duration,

    /// Instruction counts by type
    pub instruction_counts: HashMap<String, u64>,

    /// Cache hit/miss statistics
    pub cache_hits: u64,
    pub cache_misses: u64,

    /// Stack statistics
    pub max_stack_depth: usize,

    /// Number of function calls
    pub function_calls: u64,
}

impl VMStats {
    fn new() -> Self {
        Self {
            total_instructions: 0,
            execution_time: Duration::ZERO,
            instruction_counts: HashMap::new(),
            cache_hits: 0,
            cache_misses: 0,
            max_stack_depth: 0,
            function_calls: 0,
        }
    }

    fn record_instruction(&mut self, instruction_name: &str) {
        self.total_instructions += 1;
        *self
            .instruction_counts
            .entry(instruction_name.to_string())
            .or_insert(0) += 1;
    }

    fn update_max_stack(&mut self, stack_size: usize) {
        if stack_size > self.max_stack_depth {
            self.max_stack_depth = stack_size;
        }
    }

    /// Format statistics as a readable string
    pub fn format_report(&self) -> String {
        let mut report = String::new();

        report.push_str(&format!("=== VM Execution Statistics ===\n"));
        report.push_str(&format!(
            "Total Instructions: {}\n",
            self.total_instructions
        ));
        report.push_str(&format!("Execution Time: {:.2?}\n", self.execution_time));

        if self.total_instructions > 0 {
            let ips = self.total_instructions as f64 / self.execution_time.as_secs_f64();
            report.push_str(&format!("Instructions/sec: {:.0}\n", ips));
        }

        report.push_str(&format!("\nStack:\n"));
        report.push_str(&format!("  Max Depth: {}\n", self.max_stack_depth));

        report.push_str(&format!("\nCache:\n"));
        let total_cache_ops = self.cache_hits + self.cache_misses;
        if total_cache_ops > 0 {
            let hit_rate = (self.cache_hits as f64 / total_cache_ops as f64) * 100.0;
            report.push_str(&format!("  Hits: {} ({:.1}%)\n", self.cache_hits, hit_rate));
            report.push_str(&format!("  Misses: {}\n", self.cache_misses));
        } else {
            report.push_str(&format!("  No cache operations\n"));
        }

        report.push_str(&format!("\nTop Instructions:\n"));
        let mut sorted_instructions: Vec<_> = self.instruction_counts.iter().collect();
        sorted_instructions.sort_by(|a, b| b.1.cmp(a.1));
        for (name, count) in sorted_instructions.iter().take(10) {
            let percentage = (*count * 100) as f64 / self.total_instructions as f64;
            report.push_str(&format!("  {:20} {:8} ({:.1}%)\n", name, count, percentage));
        }

        report
    }
}

/// Bytecode Virtual Machine
pub struct BytecodeVM {
    /// Value stack
    stack: Vec<Value>,

    /// Local variables
    locals: Vec<Value>,

    /// Global variables
    globals: HashMap<String, Value>,

    /// Direct-mapped cache for global lookups
    global_cache: Vec<Option<CacheEntry>>,

    /// Call frames
    call_stack: Vec<CallFrame>,

    /// Instruction pointer
    ip: usize,

    /// Current chunk being executed
    chunk: Option<BytecodeChunk>,

    /// Output buffer (for testing)
    pub output: Vec<String>,

    /// Version counter for cache invalidation
    cache_version: u64,

    /// Execution statistics (optional profiling)
    stats: Option<VMStats>,

    /// Start time for execution timing
    start_time: Option<Instant>,
}

impl BytecodeVM {
    /// Create a new VM
    pub fn new() -> Self {
        Self {
            stack: Vec::with_capacity(256),
            locals: vec![Value::Null; 256],
            globals: HashMap::new(),
            global_cache: vec![None; GLOBAL_CACHE_SIZE],
            call_stack: Vec::new(),
            ip: 0,
            chunk: None,
            output: Vec::new(),
            cache_version: 0,
            stats: None,
            start_time: None,
        }
    }

    /// Enable statistics collection
    pub fn enable_stats(&mut self) {
        self.stats = Some(VMStats::new());
    }

    /// Get statistics (if enabled)
    pub fn get_stats(&self) -> Option<&VMStats> {
        self.stats.as_ref()
    }

    /// Get formatted statistics report
    pub fn stats_report(&self) -> Option<String> {
        self.stats.as_ref().map(|s| s.format_report())
    }

    /// Load a bytecode chunk and prepare for execution
    pub fn load(&mut self, chunk: BytecodeChunk) {
        self.chunk = Some(chunk);
        self.ip = 0;
        self.stack.clear();
        for slot in self.locals.iter_mut() {
            *slot = Value::Null;
        }
        self.output.clear();
        self.invalidate_all_caches();
    }

    /// Execute the loaded bytecode chunk
    pub fn run(&mut self) -> VMResult<Value> {
        if self.chunk.is_none() {
            return Err(VMError::Other("No bytecode loaded".to_string()));
        }

        // Start timing if stats enabled
        if self.stats.is_some() {
            self.start_time = Some(Instant::now());
        }

        let result = self.run_loop();

        // Record execution time if stats enabled
        if let (Some(stats), Some(start)) = (&mut self.stats, self.start_time) {
            stats.execution_time = start.elapsed();
        }

        result
    }

    /// Internal run loop
    fn run_loop(&mut self) -> VMResult<Value> {
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

            // Record instruction in stats
            if let Some(stats) = &mut self.stats {
                stats.record_instruction(
                    &format!("{:?}", instruction)
                        .split('(')
                        .next()
                        .unwrap_or("Unknown"),
                );
                stats.update_max_stack(self.stack.len());
            }

            match self.execute_instruction(instruction) {
                Ok(Some(value)) => return Ok(value), // Return instruction hit
                Ok(None) => continue,                // Continue execution
                Err(e) => return Err(self.format_error(e)), // Format error with line number
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

            Instruction::LoadGlobal(name) => {
                let value = self.load_global_cached(name)?;
                self.push(value)?;
            }

            Instruction::StoreGlobal(name) => {
                let value = self.pop()?;
                self.store_global_cached(name, value)?;
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

    /// Get the current line number for error reporting
    fn get_current_line(&self) -> Option<u32> {
        let chunk = self.chunk.as_ref()?;
        let debug_info = chunk.debug_info.as_ref()?;

        // ip is already incremented, so we need ip - 1
        let instruction_index = self.ip.saturating_sub(1);
        debug_info.line_numbers.get(instruction_index).copied()
    }

    /// Format an error with line number information
    fn format_error(&self, error: VMError) -> VMError {
        if let Some(line) = self.get_current_line() {
            VMError::Other(format!("{} (at line {})", error, line))
        } else {
            error
        }
    }

    fn load_global_cached(&mut self, name: &str) -> VMResult<Value> {
        let key_hash = Self::hash_key(name);
        let index = Self::cache_index(key_hash);

        if let Some(entry) = self.global_cache[index].as_ref() {
            if entry.key_hash == key_hash && entry.version == self.cache_version {
                // Cache hit!
                if let Some(stats) = &mut self.stats {
                    stats.cache_hits += 1;
                }
                return Ok(entry.value.clone());
            }
        }

        // Cache miss
        if let Some(stats) = &mut self.stats {
            stats.cache_misses += 1;
        }

        let value = self.globals.get(name).cloned().unwrap_or(Value::Null);

        self.global_cache[index] = Some(CacheEntry {
            key_hash,
            version: self.cache_version,
            value: value.clone(),
        });

        Ok(value)
    }

    fn store_global_cached(&mut self, name: &str, value: Value) -> VMResult<()> {
        self.cache_version = self.cache_version.wrapping_add(1);
        let key_hash = Self::hash_key(name);
        let index = Self::cache_index(key_hash);

        self.globals.insert(name.to_string(), value.clone());
        self.global_cache[index] = Some(CacheEntry {
            key_hash,
            version: self.cache_version,
            value,
        });
        Ok(())
    }

    fn invalidate_all_caches(&mut self) {
        self.cache_version = self.cache_version.wrapping_add(1);
        for slot in self.global_cache.iter_mut() {
            *slot = None;
        }
    }

    #[inline]
    fn hash_key(name: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        hasher.finish()
    }

    #[inline]
    fn cache_index(key_hash: u64) -> usize {
        (key_hash as usize) & (GLOBAL_CACHE_SIZE - 1)
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

    #[test]
    fn test_vm_global_store_and_load() {
        let mut chunk = BytecodeChunk::new(1);
        chunk.constants.push(Constant::Number(42.0));
        chunk.code.push(Instruction::LoadConst(0));
        chunk.code.push(Instruction::StoreGlobal("x".to_string()));
        chunk.code.push(Instruction::LoadGlobal("x".to_string()));
        chunk.code.push(Instruction::Return);

        let mut vm = BytecodeVM::new();
        vm.load(chunk);
        let result = vm.run().unwrap();

        assert_eq!(result, Value::Number(42.0));
    }

    #[test]
    fn test_vm_global_cache_invalidation() {
        let mut chunk = BytecodeChunk::new(1);
        chunk.constants.push(Constant::Number(1.0));
        chunk.constants.push(Constant::Number(2.0));
        chunk.code.push(Instruction::LoadConst(0));
        chunk.code.push(Instruction::StoreGlobal("y".to_string()));
        chunk.code.push(Instruction::LoadGlobal("y".to_string()));
        chunk.code.push(Instruction::Pop);
        chunk.code.push(Instruction::LoadConst(1));
        chunk.code.push(Instruction::StoreGlobal("y".to_string()));
        chunk.code.push(Instruction::LoadGlobal("y".to_string()));
        chunk.code.push(Instruction::Return);

        let mut vm = BytecodeVM::new();
        vm.load(chunk);
        let result = vm.run().unwrap();

        assert_eq!(result, Value::Number(2.0));
    }

    #[test]
    fn test_vm_error_with_line_numbers() {
        use super::DebugInfo;

        let mut chunk = BytecodeChunk::new(1);
        chunk.constants.push(Constant::Number(10.0));
        chunk.constants.push(Constant::Number(0.0));
        chunk.code.push(Instruction::LoadConst(0)); // Line 1
        chunk.code.push(Instruction::LoadConst(1)); // Line 2
        chunk.code.push(Instruction::Divide); // Line 3 - division by zero
        chunk.code.push(Instruction::Return); // Line 4

        // Add debug info with line numbers
        chunk.debug_info = Some(DebugInfo {
            source_file: "test.poh".to_string(),
            line_numbers: vec![1, 2, 3, 4],
            variable_names: Vec::new(),
        });

        let mut vm = BytecodeVM::new();
        vm.load(chunk);
        let result = vm.run();

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("line 3"),
            "Error message should include line number: {}",
            error_msg
        );
    }

    #[test]
    fn test_vm_statistics() {
        let mut chunk = BytecodeChunk::new(1);
        chunk.constants.push(Constant::Number(5.0));
        chunk.constants.push(Constant::Number(3.0));

        // Simple program: load two numbers, add them, return
        chunk.code.push(Instruction::LoadConst(0));
        chunk.code.push(Instruction::StoreGlobal("x".to_string()));
        chunk.code.push(Instruction::LoadGlobal("x".to_string()));
        chunk.code.push(Instruction::LoadConst(1));
        chunk.code.push(Instruction::Add);
        chunk.code.push(Instruction::Return);

        let mut vm = BytecodeVM::new();
        vm.enable_stats(); // Enable statistics collection
        vm.load(chunk);
        let result = vm.run().unwrap();

        assert_eq!(result, Value::Number(8.0));

        // Check statistics
        let stats = vm.get_stats().expect("Stats should be available");
        assert_eq!(stats.total_instructions, 6); // 6 instructions executed
                                                 // Execution time can be 0ns on very fast runners; don't assert > 0 to avoid flakiness
        assert!(stats.max_stack_depth > 0); // Stack was used

        // Check cache statistics: one LoadGlobal executed => first access is a miss, no hits yet
        assert_eq!(stats.cache_hits, 0);
        assert_eq!(stats.cache_misses, 1);

        // Print report for manual inspection
        if let Some(report) = vm.stats_report() {
            println!("\n{}", report);
        }
    }
}
