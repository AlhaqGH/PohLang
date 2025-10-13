/// Bytecode Compiler Test Suite
///
/// Comprehensive tests for the bytecode compiler

#[cfg(test)]
mod tests {
    use pohlang::bytecode::{Compiler, Constant, Instruction};
    use pohlang::parser::ast::{CmpOp, Expr, Stmt};

    #[test]
    fn test_compile_empty_program() {
        let compiler = Compiler::new();
        let program = vec![];
        let chunk = compiler.compile(program).unwrap();

        // Should just have a Return
        assert_eq!(chunk.instruction_count(), 1);
        assert!(matches!(chunk.code[0], Instruction::Return));
    }

    #[test]
    fn test_compile_print_number() {
        let compiler = Compiler::new();
        let program = vec![Stmt::Write(Expr::Num(42.0))];
        let chunk = compiler.compile(program).unwrap();

        // Should have: LoadConst(0), Print, Return
        assert_eq!(chunk.instruction_count(), 3);
        assert_eq!(chunk.constants.len(), 1);
        assert!(matches!(chunk.constants[0], Constant::Number(n) if n == 42.0));
        assert!(matches!(chunk.code[0], Instruction::LoadConst(0)));
        assert!(matches!(chunk.code[1], Instruction::Print));
        assert!(matches!(chunk.code[2], Instruction::Return));
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
        assert!(matches!(chunk.code[2], Instruction::Add));
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
        assert!(matches!(chunk.code[0], Instruction::LoadConst(0)));
        assert!(matches!(chunk.code[1], Instruction::StoreLocal(0)));
        assert!(matches!(chunk.code[2], Instruction::LoadLocal(0)));
        assert!(matches!(chunk.code[3], Instruction::Print));
        assert!(matches!(chunk.code[4], Instruction::Return));
    }

    #[test]
    fn test_compile_comparison() {
        let compiler = Compiler::new();
        let program = vec![
            Stmt::Set {
                name: "x".to_string(),
                value: Expr::Num(15.0),
            },
            Stmt::Write(Expr::Cmp(
                CmpOp::Gt,
                Box::new(Expr::Ident("x".to_string())),
                Box::new(Expr::Num(10.0)),
            )),
        ];
        let chunk = compiler.compile(program).unwrap();

        // Verify Greater instruction is emitted
        assert!(chunk
            .code
            .iter()
            .any(|inst| matches!(inst, Instruction::Greater)));
    }

    #[test]
    fn test_compile_if_statement() {
        let compiler = Compiler::new();
        let program = vec![
            Stmt::Set {
                name: "x".to_string(),
                value: Expr::Num(15.0),
            },
            Stmt::IfInline {
                cond: Expr::Cmp(
                    CmpOp::Gt,
                    Box::new(Expr::Ident("x".to_string())),
                    Box::new(Expr::Num(10.0)),
                ),
                then_write: Expr::Str("big".to_string()),
                otherwise_write: Some(Expr::Str("small".to_string())),
            },
        ];
        let chunk = compiler.compile(program).unwrap();

        // Verify Jump instructions are present
        assert!(chunk
            .code
            .iter()
            .any(|inst| matches!(inst, Instruction::JumpIfFalse(_))));
        assert!(chunk
            .code
            .iter()
            .any(|inst| matches!(inst, Instruction::Jump(_))));
    }

    #[test]
    fn test_compile_while_loop() {
        let compiler = Compiler::new();
        let program = vec![
            Stmt::Set {
                name: "counter".to_string(),
                value: Expr::Num(3.0),
            },
            Stmt::WhileBlock {
                cond: Expr::Cmp(
                    CmpOp::Gt,
                    Box::new(Expr::Ident("counter".to_string())),
                    Box::new(Expr::Num(0.0)),
                ),
                body: vec![
                    Stmt::Write(Expr::Ident("counter".to_string())),
                    Stmt::Set {
                        name: "counter".to_string(),
                        value: Expr::Minus(
                            Box::new(Expr::Ident("counter".to_string())),
                            Box::new(Expr::Num(1.0)),
                        ),
                    },
                ],
            },
        ];
        let chunk = compiler.compile(program).unwrap();

        // Verify Loop instruction is present
        assert!(chunk
            .code
            .iter()
            .any(|inst| matches!(inst, Instruction::Loop(_))));
    }

    #[test]
    fn test_compile_all_arithmetic_ops() {
        let ops = vec![
            (
                Expr::Plus(Box::new(Expr::Num(1.0)), Box::new(Expr::Num(2.0))),
                Instruction::Add,
            ),
            (
                Expr::Minus(Box::new(Expr::Num(1.0)), Box::new(Expr::Num(2.0))),
                Instruction::Subtract,
            ),
            (
                Expr::Times(Box::new(Expr::Num(1.0)), Box::new(Expr::Num(2.0))),
                Instruction::Multiply,
            ),
            (
                Expr::DividedBy(Box::new(Expr::Num(1.0)), Box::new(Expr::Num(2.0))),
                Instruction::Divide,
            ),
        ];

        for (expr, expected_inst) in ops {
            let compiler = Compiler::new();
            let program = vec![Stmt::Write(expr)];
            let chunk = compiler.compile(program).unwrap();

            // Verify the operation instruction is present
            assert!(
                chunk
                    .code
                    .iter()
                    .any(|inst| std::mem::discriminant(inst)
                        == std::mem::discriminant(&expected_inst)),
                "Expected {:?} instruction",
                expected_inst
            );
        }
    }

    #[test]
    fn test_compile_all_comparison_ops() {
        let ops = vec![
            (CmpOp::Eq, Instruction::Equal),
            (CmpOp::Ne, Instruction::NotEqual),
            (CmpOp::Lt, Instruction::Less),
            (CmpOp::Le, Instruction::LessEqual),
            (CmpOp::Gt, Instruction::Greater),
            (CmpOp::Ge, Instruction::GreaterEqual),
        ];

        for (op, expected_inst) in ops {
            let compiler = Compiler::new();
            let program = vec![Stmt::Write(Expr::Cmp(
                op,
                Box::new(Expr::Num(1.0)),
                Box::new(Expr::Num(2.0)),
            ))];
            let chunk = compiler.compile(program).unwrap();

            // Verify the comparison instruction is present
            assert!(
                chunk
                    .code
                    .iter()
                    .any(|inst| std::mem::discriminant(inst)
                        == std::mem::discriminant(&expected_inst)),
                "Expected {:?} instruction",
                expected_inst
            );
        }
    }

    #[test]
    fn test_compile_logical_ops() {
        let compiler = Compiler::new();
        let program = vec![Stmt::Write(Expr::And(
            Box::new(Expr::Bool(true)),
            Box::new(Expr::Bool(false)),
        ))];
        let chunk = compiler.compile(program).unwrap();
        assert!(chunk
            .code
            .iter()
            .any(|inst| matches!(inst, Instruction::And)));

        let compiler = Compiler::new();
        let program = vec![Stmt::Write(Expr::Or(
            Box::new(Expr::Bool(true)),
            Box::new(Expr::Bool(false)),
        ))];
        let chunk = compiler.compile(program).unwrap();
        assert!(chunk
            .code
            .iter()
            .any(|inst| matches!(inst, Instruction::Or)));

        let compiler = Compiler::new();
        let program = vec![Stmt::Write(Expr::Not(Box::new(Expr::Bool(true))))];
        let chunk = compiler.compile(program).unwrap();
        assert!(chunk
            .code
            .iter()
            .any(|inst| matches!(inst, Instruction::Not)));
    }

    #[test]
    fn test_undefined_variable_error() {
        let compiler = Compiler::new();
        let program = vec![Stmt::Write(Expr::Ident("undefined_var".to_string()))];
        let result = compiler.compile(program);
        assert!(result.is_err());
    }

    #[test]
    fn test_compile_string_literal() {
        let compiler = Compiler::new();
        let program = vec![Stmt::Write(Expr::Str("Hello, PohLang!".to_string()))];
        let chunk = compiler.compile(program).unwrap();

        assert_eq!(chunk.constants.len(), 1);
        assert!(matches!(&chunk.constants[0], Constant::String(s) if s == "Hello, PohLang!"));
    }

    #[test]
    fn test_compile_boolean_literals() {
        let compiler = Compiler::new();
        let program = vec![
            Stmt::Write(Expr::Bool(true)),
            Stmt::Write(Expr::Bool(false)),
        ];
        let chunk = compiler.compile(program).unwrap();

        assert_eq!(chunk.constants.len(), 2);
        assert!(matches!(chunk.constants[0], Constant::Boolean(true)));
        assert!(matches!(chunk.constants[1], Constant::Boolean(false)));
    }

    #[test]
    fn test_compile_null_literal() {
        let compiler = Compiler::new();
        let program = vec![Stmt::Write(Expr::Null)];
        let chunk = compiler.compile(program).unwrap();

        assert_eq!(chunk.constants.len(), 1);
        assert!(matches!(chunk.constants[0], Constant::Null));
    }
}
