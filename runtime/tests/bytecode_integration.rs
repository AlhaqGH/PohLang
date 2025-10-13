/// Integration tests for the complete bytecode pipeline
/// Tests: AST → Compiler → VM → Result

#[cfg(test)]
mod tests {
    use pohlang::bytecode::{BytecodeVM, Compiler, Value};
    use pohlang::parser::ast::{CmpOp, Expr, Stmt};

    fn compile_and_run(program: Vec<Stmt>) -> Result<Value, String> {
        let compiler = Compiler::new();
        let chunk = compiler.compile(program).map_err(|e| e.to_string())?;

        let mut vm = BytecodeVM::new();
        vm.load(chunk);
        vm.run().map_err(|e| e.to_string())
    }

    fn compile_and_run_with_output(program: Vec<Stmt>) -> Result<(Value, Vec<String>), String> {
        let compiler = Compiler::new();
        let chunk = compiler.compile(program).map_err(|e| e.to_string())?;

        let mut vm = BytecodeVM::new();
        vm.load(chunk);
        let result = vm.run().map_err(|e| e.to_string())?;
        let output = vm.get_output();
        Ok((result, output))
    }

    #[test]
    fn test_simple_arithmetic() {
        let program = vec![Stmt::Write(Expr::Plus(
            Box::new(Expr::Num(10.0)),
            Box::new(Expr::Num(20.0)),
        ))];

        let (_, output) = compile_and_run_with_output(program).unwrap();
        assert_eq!(output, vec!["30"]);
    }

    #[test]
    fn test_complex_arithmetic() {
        // ((5 * 3) + (10 / 2)) - 2 = (15 + 5) - 2 = 18
        let program = vec![Stmt::Write(Expr::Minus(
            Box::new(Expr::Plus(
                Box::new(Expr::Times(
                    Box::new(Expr::Num(5.0)),
                    Box::new(Expr::Num(3.0)),
                )),
                Box::new(Expr::DividedBy(
                    Box::new(Expr::Num(10.0)),
                    Box::new(Expr::Num(2.0)),
                )),
            )),
            Box::new(Expr::Num(2.0)),
        ))];

        let (_, output) = compile_and_run_with_output(program).unwrap();
        assert_eq!(output, vec!["18"]);
    }

    #[test]
    fn test_variable_assignment_and_use() {
        let program = vec![
            Stmt::Set {
                name: "x".to_string(),
                value: Expr::Num(42.0),
            },
            Stmt::Write(Expr::Ident("x".to_string())),
        ];

        let (_, output) = compile_and_run_with_output(program).unwrap();
        assert_eq!(output, vec!["42"]);
    }

    #[test]
    fn test_variable_arithmetic() {
        let program = vec![
            Stmt::Set {
                name: "a".to_string(),
                value: Expr::Num(10.0),
            },
            Stmt::Set {
                name: "b".to_string(),
                value: Expr::Num(20.0),
            },
            Stmt::Write(Expr::Plus(
                Box::new(Expr::Ident("a".to_string())),
                Box::new(Expr::Ident("b".to_string())),
            )),
        ];

        let (_, output) = compile_and_run_with_output(program).unwrap();
        assert_eq!(output, vec!["30"]);
    }

    #[test]
    fn test_comparison_operations() {
        let program = vec![
            Stmt::Write(Expr::Cmp(
                CmpOp::Gt,
                Box::new(Expr::Num(15.0)),
                Box::new(Expr::Num(10.0)),
            )),
            Stmt::Write(Expr::Cmp(
                CmpOp::Lt,
                Box::new(Expr::Num(5.0)),
                Box::new(Expr::Num(10.0)),
            )),
            Stmt::Write(Expr::Cmp(
                CmpOp::Eq,
                Box::new(Expr::Num(10.0)),
                Box::new(Expr::Num(10.0)),
            )),
        ];

        let (_, output) = compile_and_run_with_output(program).unwrap();
        assert_eq!(output, vec!["true", "true", "true"]);
    }

    #[test]
    fn test_logical_operations() {
        let program = vec![
            Stmt::Write(Expr::And(
                Box::new(Expr::Bool(true)),
                Box::new(Expr::Bool(false)),
            )),
            Stmt::Write(Expr::Or(
                Box::new(Expr::Bool(true)),
                Box::new(Expr::Bool(false)),
            )),
            Stmt::Write(Expr::Not(Box::new(Expr::Bool(false)))),
        ];

        let (_, output) = compile_and_run_with_output(program).unwrap();
        assert_eq!(output, vec!["false", "true", "true"]);
    }

    #[test]
    fn test_string_output() {
        let program = vec![Stmt::Write(Expr::Str("Hello, World!".to_string()))];

        let (_, output) = compile_and_run_with_output(program).unwrap();
        assert_eq!(output, vec!["Hello, World!"]);
    }

    #[test]
    fn test_string_concatenation() {
        let program = vec![
            Stmt::Set {
                name: "first".to_string(),
                value: Expr::Str("Hello".to_string()),
            },
            Stmt::Set {
                name: "second".to_string(),
                value: Expr::Str(" World".to_string()),
            },
            Stmt::Write(Expr::Plus(
                Box::new(Expr::Ident("first".to_string())),
                Box::new(Expr::Ident("second".to_string())),
            )),
        ];

        let (_, output) = compile_and_run_with_output(program).unwrap();
        assert_eq!(output, vec!["Hello World"]);
    }

    #[test]
    fn test_if_inline_true_branch() {
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

        let (_, output) = compile_and_run_with_output(program).unwrap();
        assert_eq!(output, vec!["big"]);
    }

    #[test]
    fn test_if_inline_false_branch() {
        let program = vec![
            Stmt::Set {
                name: "x".to_string(),
                value: Expr::Num(5.0),
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

        let (_, output) = compile_and_run_with_output(program).unwrap();
        assert_eq!(output, vec!["small"]);
    }

    #[test]
    fn test_if_block() {
        let program = vec![
            Stmt::Set {
                name: "score".to_string(),
                value: Expr::Num(85.0),
            },
            Stmt::IfBlock {
                cond: Expr::Cmp(
                    CmpOp::Ge,
                    Box::new(Expr::Ident("score".to_string())),
                    Box::new(Expr::Num(80.0)),
                ),
                then_body: vec![Stmt::Write(Expr::Str("Pass".to_string()))],
                otherwise_body: Some(vec![Stmt::Write(Expr::Str("Fail".to_string()))]),
            },
        ];

        let (_, output) = compile_and_run_with_output(program).unwrap();
        assert_eq!(output, vec!["Pass"]);
    }

    #[test]
    fn test_multiple_statements() {
        let program = vec![
            Stmt::Write(Expr::Str("Line 1".to_string())),
            Stmt::Write(Expr::Str("Line 2".to_string())),
            Stmt::Write(Expr::Str("Line 3".to_string())),
        ];

        let (_, output) = compile_and_run_with_output(program).unwrap();
        assert_eq!(output, vec!["Line 1", "Line 2", "Line 3"]);
    }

    #[test]
    fn test_return_value() {
        let program = vec![
            Stmt::Set {
                name: "result".to_string(),
                value: Expr::Num(42.0),
            },
            Stmt::Return(Some(Expr::Ident("result".to_string()))),
        ];

        let result = compile_and_run(program).unwrap();
        assert_eq!(result, Value::Number(42.0));
    }

    #[test]
    fn test_null_handling() {
        let program = vec![Stmt::Write(Expr::Null)];

        let (_, output) = compile_and_run_with_output(program).unwrap();
        assert_eq!(output, vec!["null"]);
    }

    #[test]
    fn test_boolean_literals() {
        let program = vec![
            Stmt::Write(Expr::Bool(true)),
            Stmt::Write(Expr::Bool(false)),
        ];

        let (_, output) = compile_and_run_with_output(program).unwrap();
        assert_eq!(output, vec!["true", "false"]);
    }

    #[test]
    fn test_division_by_zero_error() {
        let program = vec![Stmt::Write(Expr::DividedBy(
            Box::new(Expr::Num(10.0)),
            Box::new(Expr::Num(0.0)),
        ))];

        let result = compile_and_run(program);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Division by zero"));
    }

    #[test]
    fn test_undefined_variable_error() {
        let program = vec![Stmt::Write(Expr::Ident("undefined".to_string()))];

        let result = compile_and_run(program);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined variable"));
    }

    #[test]
    fn test_type_error_arithmetic() {
        // This should fail at runtime: trying to subtract strings
        let program = vec![Stmt::Write(Expr::Minus(
            Box::new(Expr::Str("hello".to_string())),
            Box::new(Expr::Str("world".to_string())),
        ))];

        let result = compile_and_run(program);
        assert!(result.is_err());
    }

    #[test]
    fn test_variable_reassignment() {
        let program = vec![
            Stmt::Set {
                name: "x".to_string(),
                value: Expr::Num(10.0),
            },
            Stmt::Write(Expr::Ident("x".to_string())),
            Stmt::Set {
                name: "x".to_string(),
                value: Expr::Num(20.0),
            },
            Stmt::Write(Expr::Ident("x".to_string())),
        ];

        let (_, output) = compile_and_run_with_output(program).unwrap();
        assert_eq!(output, vec!["10", "20"]);
    }
}
