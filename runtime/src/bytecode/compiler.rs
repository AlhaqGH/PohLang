use super::{BytecodeChunk, Constant, Instruction};
use crate::parser::ast::{CmpOp, Expr, Program, Stmt};
use std::collections::HashMap;

/// Compiler error types
#[derive(Debug, Clone)]
pub enum CompilerError {
    UndefinedVariable(String),
    TooManyConstants,
    TooManyLocals,
    InvalidJumpTarget,
    NestedFunctionNotSupported,
    Other(String),
}

impl std::fmt::Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilerError::UndefinedVariable(name) => write!(f, "Undefined variable: {}", name),
            CompilerError::TooManyConstants => write!(f, "Too many constants (max 2^32)"),
            CompilerError::TooManyLocals => write!(f, "Too many local variables (max 256)"),
            CompilerError::InvalidJumpTarget => write!(f, "Invalid jump target"),
            CompilerError::NestedFunctionNotSupported => {
                write!(f, "Nested function definitions not supported")
            }
            CompilerError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for CompilerError {}

pub type CompileResult<T> = Result<T, CompilerError>;

/// Compilation context tracking local variables and their indices
#[derive(Debug)]
struct CompilerContext {
    locals: HashMap<String, u32>,
    local_count: u32,
    scope_depth: u32,
}

impl CompilerContext {
    fn new() -> Self {
        Self {
            locals: HashMap::new(),
            local_count: 0,
            scope_depth: 0,
        }
    }

    fn enter_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn exit_scope(&mut self) {
        self.scope_depth -= 1;
        // Remove locals from this scope (simple version - remove all for now)
        // In a more sophisticated implementation, we'd track scope depth per variable
    }

    fn define_local(&mut self, name: String) -> CompileResult<u32> {
        if self.local_count >= 256 {
            return Err(CompilerError::TooManyLocals);
        }
        let index = self.local_count;
        self.locals.insert(name, index);
        self.local_count += 1;
        Ok(index)
    }

    fn get_local(&self, name: &str) -> Option<u32> {
        self.locals.get(name).copied()
    }
}

/// Bytecode compiler that converts AST to bytecode
pub struct Compiler {
    chunk: BytecodeChunk,
    context: CompilerContext,
}

impl Compiler {
    /// Create a new compiler with bytecode format version 1
    pub fn new() -> Self {
        Self {
            chunk: BytecodeChunk::new(1),
            context: CompilerContext::new(),
        }
    }

    /// Compile a program (list of statements) to bytecode
    pub fn compile(mut self, program: Program) -> CompileResult<BytecodeChunk> {
        for stmt in program {
            self.compile_stmt(stmt)?;
        }
        // Add implicit return at end
        self.emit(Instruction::Return);
        Ok(self.chunk)
    }

    /// Emit a bytecode instruction
    fn emit(&mut self, instruction: Instruction) {
        self.chunk.code.push(instruction);
    }

    /// Add a constant to the constant pool and return its index
    fn add_constant(&mut self, constant: Constant) -> CompileResult<u32> {
        let index = self.chunk.constants.len() as u32;
        if index == u32::MAX {
            return Err(CompilerError::TooManyConstants);
        }
        self.chunk.constants.push(constant);
        Ok(index)
    }

    /// Get current instruction index (for jump calculations)
    fn current_offset(&self) -> usize {
        self.chunk.code.len()
    }

    /// Patch a jump instruction at the given offset with the current position
    fn patch_jump(&mut self, offset: usize) -> CompileResult<()> {
        let jump_distance = self.current_offset() - offset - 1;
        if jump_distance > u32::MAX as usize {
            return Err(CompilerError::InvalidJumpTarget);
        }

        // Update the jump instruction with the calculated distance
        match &mut self.chunk.code[offset] {
            Instruction::Jump(ref mut target) => *target = jump_distance as i32,
            Instruction::JumpIfFalse(ref mut target) => *target = jump_distance as i32,
            Instruction::PushTryHandler(ref mut target) => *target = jump_distance as u32,
            _ => return Err(CompilerError::InvalidJumpTarget),
        }
        Ok(())
    }

    // ========================================================================
    // Statement Compilation
    // ========================================================================

    fn compile_stmt(&mut self, stmt: Stmt) -> CompileResult<()> {
        match stmt {
            Stmt::Write(expr) => {
                self.compile_expr(expr)?;
                self.emit(Instruction::Print);
            }

            Stmt::AskFor { var_name } => {
                self.emit(Instruction::Input);
                let local_idx = self.context.define_local(var_name)?;
                self.emit(Instruction::StoreLocal(local_idx));
            }

            Stmt::Set { name, value } => {
                self.compile_expr(value)?;

                // Check if variable exists, if not create it
                let local_idx = if let Some(idx) = self.context.get_local(&name) {
                    idx
                } else {
                    self.context.define_local(name)?
                };

                self.emit(Instruction::StoreLocal(local_idx));
            }

            Stmt::Return(expr_opt) => {
                if let Some(expr) = expr_opt {
                    self.compile_expr(expr)?;
                } else {
                    // Return null if no expression
                    let null_idx = self.add_constant(Constant::Null)?;
                    self.emit(Instruction::LoadConst(null_idx));
                }
                self.emit(Instruction::Return);
            }

            Stmt::IfInline {
                cond,
                then_write,
                otherwise_write,
            } => {
                // Compile condition
                self.compile_expr(cond)?;

                // Jump to else if false
                let else_jump = self.current_offset();
                self.emit(Instruction::JumpIfFalse(0)); // Placeholder

                // Then branch
                self.compile_expr(then_write)?;
                self.emit(Instruction::Print);

                if let Some(else_write) = otherwise_write {
                    // Jump over else
                    let end_jump = self.current_offset();
                    self.emit(Instruction::Jump(0)); // Placeholder

                    // Patch else jump
                    self.patch_jump(else_jump)?;

                    // Else branch
                    self.compile_expr(else_write)?;
                    self.emit(Instruction::Print);

                    // Patch end jump
                    self.patch_jump(end_jump)?;
                } else {
                    // No else, just patch the jump
                    self.patch_jump(else_jump)?;
                }
            }

            Stmt::IfBlock {
                cond,
                then_body,
                otherwise_body,
            } => {
                // Compile condition
                self.compile_expr(cond)?;

                // Jump to else if false
                let else_jump = self.current_offset();
                self.emit(Instruction::JumpIfFalse(0)); // Placeholder

                // Then block
                self.context.enter_scope();
                for stmt in then_body {
                    self.compile_stmt(stmt)?;
                }
                self.context.exit_scope();

                if let Some(else_body) = otherwise_body {
                    // Jump over else
                    let end_jump = self.current_offset();
                    self.emit(Instruction::Jump(0)); // Placeholder

                    // Patch else jump
                    self.patch_jump(else_jump)?;

                    // Else block
                    self.context.enter_scope();
                    for stmt in else_body {
                        self.compile_stmt(stmt)?;
                    }
                    self.context.exit_scope();

                    // Patch end jump
                    self.patch_jump(end_jump)?;
                } else {
                    // No else, just patch the jump
                    self.patch_jump(else_jump)?;
                }
            }

            Stmt::WhileBlock { cond, body } => {
                let loop_start = self.current_offset();

                // Compile condition
                self.compile_expr(cond)?;

                // Jump to end if false
                let exit_jump = self.current_offset();
                self.emit(Instruction::JumpIfFalse(0)); // Placeholder

                // Loop body
                self.context.enter_scope();
                for stmt in body {
                    self.compile_stmt(stmt)?;
                }
                self.context.exit_scope();

                // Jump back to start
                let loop_distance = self.current_offset() - loop_start + 1;
                self.emit(Instruction::Loop(loop_distance as i32));

                // Patch exit jump
                self.patch_jump(exit_jump)?;
            }

            Stmt::RepeatBlock { count, body } => {
                // Compile count expression
                self.compile_expr(count)?;

                // Store in a temporary local
                let counter_idx = self.context.define_local("__repeat_count__".to_string())?;
                self.emit(Instruction::StoreLocal(counter_idx));

                let loop_start = self.current_offset();

                // Load counter and check if > 0
                self.emit(Instruction::LoadLocal(counter_idx));
                let zero_idx = self.add_constant(Constant::Number(0.0))?;
                self.emit(Instruction::LoadConst(zero_idx));
                self.emit(Instruction::Greater);

                // Jump to end if false
                let exit_jump = self.current_offset();
                self.emit(Instruction::JumpIfFalse(0)); // Placeholder

                // Loop body
                self.context.enter_scope();
                for stmt in body {
                    self.compile_stmt(stmt)?;
                }
                self.context.exit_scope();

                // Decrement counter
                self.emit(Instruction::LoadLocal(counter_idx));
                let one_idx = self.add_constant(Constant::Number(1.0))?;
                self.emit(Instruction::LoadConst(one_idx));
                self.emit(Instruction::Subtract);
                self.emit(Instruction::StoreLocal(counter_idx));

                // Jump back to start
                let loop_distance = self.current_offset() - loop_start + 1;
                self.emit(Instruction::Loop(loop_distance as i32));

                // Patch exit jump
                self.patch_jump(exit_jump)?;
            }

            Stmt::Use { name, args } => {
                // Compile arguments
                let arg_count = args.len() as u8;
                for arg in args {
                    self.compile_expr(arg)?;
                }

                // Call the function

                // Load function by name (we'll need to look it up)
                if let Some(fn_idx) = self.context.get_local(&name) {
                    self.emit(Instruction::LoadLocal(fn_idx));
                } else {
                    // For now, we'll emit a call instruction with the function name as a constant
                    let name_idx = self.add_constant(Constant::String(name))?;
                    self.emit(Instruction::LoadConst(name_idx));
                }

                self.emit(Instruction::Call(arg_count));
            }

            Stmt::TryCatch {
                try_block,
                catch_handlers,
                finally_block,
            } => {
                // Push try handler
                let catch_jump = self.current_offset();
                self.emit(Instruction::PushTryHandler(0)); // Placeholder

                // Try block
                self.context.enter_scope();
                for stmt in try_block {
                    self.compile_stmt(stmt)?;
                }
                self.context.exit_scope();

                // Pop try handler
                self.emit(Instruction::PopTryHandler);

                // Jump over catch handlers
                let end_jump = self.current_offset();
                self.emit(Instruction::Jump(0)); // Placeholder

                // Patch catch jump
                self.patch_jump(catch_jump)?;

                // Catch handlers
                for handler in catch_handlers {
                    if let Some(var_name) = handler.var_name {
                        let error_idx = self.context.define_local(var_name)?;
                        self.emit(Instruction::StoreLocal(error_idx));
                    } else {
                        self.emit(Instruction::Pop);
                    }

                    self.context.enter_scope();
                    for stmt in handler.block {
                        self.compile_stmt(stmt)?;
                    }
                    self.context.exit_scope();
                }

                // Patch end jump
                self.patch_jump(end_jump)?;

                // Finally block
                if let Some(finally_stmts) = finally_block {
                    for stmt in finally_stmts {
                        self.compile_stmt(stmt)?;
                    }
                }
            }

            Stmt::Throw(expr) => {
                self.compile_expr(expr)?;
                self.emit(Instruction::Throw);
            }

            Stmt::FuncInline { name, params, body } => {
                // For now, we'll store the function as a constant
                // In a full implementation, we'd compile it to a separate chunk
                let fn_idx = self.context.define_local(name)?;

                // Compile the function body as an expression
                self.compile_expr(body)?;
                self.emit(Instruction::StoreLocal(fn_idx));
            }

            Stmt::FuncBlock { name, params, body } => {
                // For now, we'll skip function compilation
                // In a full implementation, we'd compile it to a separate chunk
                let fn_idx = self.context.define_local(name)?;
                // Store a placeholder
                let null_idx = self.add_constant(Constant::Null)?;
                self.emit(Instruction::LoadConst(null_idx));
                self.emit(Instruction::StoreLocal(fn_idx));
            }

            Stmt::AddRoute {
                path,
                method,
                handler,
            } => {
                // Compile path and method
                self.compile_expr(path)?;
                self.compile_expr(method)?;

                // For now, handler is not compiled (would need closure support)
                // We'll emit the AddRoute instruction
                self.emit(Instruction::AddRoute);
            }

            Stmt::StartServer => {
                self.emit(Instruction::StartServer);
            }

            Stmt::ImportLocal { path } => {
                // For now, we'll skip imports
                // In a full implementation, we'd load the module
            }

            Stmt::ImportSystem {
                name,
                alias,
                exposing,
            } => {
                // For now, we'll skip imports
                // In a full implementation, we'd load the system module
            }
        }

        Ok(())
    }

    // ========================================================================
    // Expression Compilation
    // ========================================================================

    fn compile_expr(&mut self, expr: Expr) -> CompileResult<()> {
        match expr {
            // Literals
            Expr::Num(n) => {
                let idx = self.add_constant(Constant::Number(n))?;
                self.emit(Instruction::LoadConst(idx));
            }

            Expr::Str(s) => {
                let idx = self.add_constant(Constant::String(s))?;
                self.emit(Instruction::LoadConst(idx));
            }

            Expr::Bool(b) => {
                let idx = self.add_constant(Constant::Boolean(b))?;
                self.emit(Instruction::LoadConst(idx));
            }

            Expr::Null => {
                let idx = self.add_constant(Constant::Null)?;
                self.emit(Instruction::LoadConst(idx));
            }

            // Variables
            Expr::Ident(name) => {
                if let Some(idx) = self.context.get_local(&name) {
                    self.emit(Instruction::LoadLocal(idx));
                } else {
                    return Err(CompilerError::UndefinedVariable(name));
                }
            }

            // Arithmetic operations
            Expr::Plus(left, right) => {
                self.compile_expr(*left)?;
                self.compile_expr(*right)?;
                self.emit(Instruction::Add);
            }

            Expr::Minus(left, right) => {
                self.compile_expr(*left)?;
                self.compile_expr(*right)?;
                self.emit(Instruction::Subtract);
            }

            Expr::Times(left, right) => {
                self.compile_expr(*left)?;
                self.compile_expr(*right)?;
                self.emit(Instruction::Multiply);
            }

            Expr::DividedBy(left, right) => {
                self.compile_expr(*left)?;
                self.compile_expr(*right)?;
                self.emit(Instruction::Divide);
            }

            // Logical operations
            Expr::And(left, right) => {
                self.compile_expr(*left)?;
                self.compile_expr(*right)?;
                self.emit(Instruction::And);
            }

            Expr::Or(left, right) => {
                self.compile_expr(*left)?;
                self.compile_expr(*right)?;
                self.emit(Instruction::Or);
            }

            Expr::Not(expr) => {
                self.compile_expr(*expr)?;
                self.emit(Instruction::Not);
            }

            // Comparison operations
            Expr::Cmp(op, left, right) => {
                self.compile_expr(*left)?;
                self.compile_expr(*right)?;
                match op {
                    CmpOp::Eq => self.emit(Instruction::Equal),
                    CmpOp::Ne => self.emit(Instruction::NotEqual),
                    CmpOp::Lt => self.emit(Instruction::Less),
                    CmpOp::Le => self.emit(Instruction::LessEqual),
                    CmpOp::Gt => self.emit(Instruction::Greater),
                    CmpOp::Ge => self.emit(Instruction::GreaterEqual),
                }
            }

            // Collections
            Expr::ListLit(items) => {
                // Compile all items
                for item in items {
                    self.compile_expr(item)?;
                }
                // TODO: Implement MakeList instruction
                // self.emit(Instruction::MakeList);
                return Err(CompilerError::Other(
                    "ListLit not yet supported".to_string(),
                ));
            }

            Expr::DictLit(pairs) => {
                // Compile all key-value pairs
                for (key, value) in pairs {
                    let key_idx = self.add_constant(Constant::String(key))?;
                    self.emit(Instruction::LoadConst(key_idx));
                    self.compile_expr(value)?;
                }
                // TODO: Implement MakeDict instruction
                // self.emit(Instruction::MakeDict);
                return Err(CompilerError::Other(
                    "DictLit not yet supported".to_string(),
                ));
            }

            Expr::Index(collection, index) => {
                self.compile_expr(*collection)?;
                self.compile_expr(*index)?;
                // TODO: Implement IndexGet instruction
                // self.emit(Instruction::IndexGet);
                return Err(CompilerError::Other("Index not yet supported".to_string()));
            }

            Expr::Contains(collection, item) => {
                self.compile_expr(*collection)?;
                self.compile_expr(*item)?;
                // TODO: Implement Contains instruction
                // self.emit(Instruction::Contains);
                return Err(CompilerError::Other(
                    "Contains not yet supported".to_string(),
                ));
            }

            Expr::Append(list, item) => {
                self.compile_expr(*list)?;
                self.compile_expr(*item)?;
                // TODO: Implement Append instruction
                // self.emit(Instruction::Append);
                return Err(CompilerError::Other("Append not yet supported".to_string()));
            }

            Expr::Remove(list, item) => {
                self.compile_expr(*list)?;
                self.compile_expr(*item)?;
                // TODO: Implement Remove instruction
                // self.emit(Instruction::Remove);
                return Err(CompilerError::Other("Remove not yet supported".to_string()));
            }

            Expr::CountOf(collection) => {
                self.compile_expr(*collection)?;
                // TODO: Implement Length instruction
                // self.emit(Instruction::Length);
                return Err(CompilerError::Other(
                    "CountOf not yet supported".to_string(),
                ));
            }

            // Function calls
            Expr::Call { name, args } => {
                // Compile arguments
                let arg_count = args.len() as u8;
                for arg in args {
                    self.compile_expr(arg)?;
                }

                // Load function by name
                if let Some(fn_idx) = self.context.get_local(&name) {
                    self.emit(Instruction::LoadLocal(fn_idx));
                } else {
                    let name_idx = self.add_constant(Constant::String(name))?;
                    self.emit(Instruction::LoadConst(name_idx));
                }

                self.emit(Instruction::Call(arg_count));
            }

            // Web framework operations
            Expr::CreateWebServer(port) => {
                self.compile_expr(*port)?;
                self.emit(Instruction::CreateWebServer);
            }

            Expr::HtmlResponse(content) => {
                self.compile_expr(*content)?;
                self.emit(Instruction::HtmlResponse);
            }

            Expr::JsonResponse(data) => {
                self.compile_expr(*data)?;
                self.emit(Instruction::JsonResponse);
            }

            // For now, other expressions will be unsupported
            // We can add them incrementally
            _ => {
                return Err(CompilerError::Other(format!(
                    "Unsupported expression: {:?}",
                    expr
                )));
            }
        }

        Ok(())
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_literal_number() {
        let compiler = Compiler::new();
        let program = vec![];
        let chunk = compiler.compile(program).unwrap();
        assert_eq!(chunk.instruction_count(), 1); // Just return
    }

    #[test]
    fn test_compile_print_number() {
        let compiler = Compiler::new();
        let program = vec![Stmt::Write(Expr::Num(42.0))];
        let chunk = compiler.compile(program).unwrap();

        // Should have: LoadConst, Print, Return
        assert_eq!(chunk.instruction_count(), 3);
        assert_eq!(chunk.constants.len(), 1);
    }

    #[test]
    fn test_compile_arithmetic() {
        let compiler = Compiler::new();
        let program = vec![Stmt::Write(Expr::Plus(
            Box::new(Expr::Num(10.0)),
            Box::new(Expr::Num(20.0)),
        ))];
        let chunk = compiler.compile(program).unwrap();

        // Should have: LoadConst(10), LoadConst(20), Add, Print, Return
        assert_eq!(chunk.instruction_count(), 5);
        assert_eq!(chunk.constants.len(), 2);
    }

    #[test]
    fn test_compile_variable_assignment() {
        let compiler = Compiler::new();
        let program = vec![
            Stmt::Set {
                name: "x".to_string(),
                value: Expr::Num(42.0),
            },
            Stmt::Write(Expr::Ident("x".to_string())),
        ];
        let chunk = compiler.compile(program).unwrap();

        // Should have: LoadConst(42), StoreLocal(0), LoadLocal(0), Print, Return
        assert_eq!(chunk.instruction_count(), 5);
    }
}
