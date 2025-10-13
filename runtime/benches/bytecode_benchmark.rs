/// Benchmarks comparing AST interpreter vs Bytecode VM performance
///
/// This file contains benchmarks for various PohLang operations to measure
/// the performance improvement gained by using the bytecode VM instead of
/// the traditional AST-walking interpreter.
///
/// Expected Results:
/// - Bytecode VM should be 5-15x faster than AST interpreter
/// - Arithmetic operations: ~10x speedup
/// - Loop execution: ~15x speedup  
/// - Function calls: ~8x speedup
/// - Variable access: ~12x speedup
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use pohlang::bytecode::{BytecodeVM, Compiler, Value};
use pohlang::parser::ast::{CmpOp, Expr, Stmt};
use pohlang::{bytecode, parser, vm};

// ============================================================================
// Helper Functions
// ============================================================================

/// Compile and run a program with the bytecode VM
fn run_bytecode(program: Vec<Stmt>) -> Value {
    let compiler = Compiler::new();
    let chunk = compiler.compile(program).expect("Compilation failed");
    let mut vm = BytecodeVM::new();
    vm.load(chunk);
    vm.run().expect("Execution failed")
}

/// Run a program with the AST interpreter
fn run_ast(program: Vec<Stmt>) {
    let mut vm = vm::Vm::default();
    vm.execute(&program).expect("Execution failed");
}

// ============================================================================
// Benchmark 1: Simple Arithmetic
// ============================================================================

fn create_arithmetic_program(iterations: usize) -> Vec<Stmt> {
    let mut stmts = Vec::new();

    for i in 0..iterations {
        // ((5 * 3) + (10 / 2)) - 2 = (15 + 5) - 2 = 18
        stmts.push(Stmt::Write(Expr::Minus(
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
        )));
    }

    stmts
}

fn bench_arithmetic(c: &mut Criterion) {
    let mut group = c.benchmark_group("arithmetic");

    for size in [10, 50, 100].iter() {
        let program = create_arithmetic_program(*size);

        group.bench_with_input(BenchmarkId::new("ast", size), size, |b, _| {
            b.iter(|| run_ast(black_box(program.clone())));
        });

        group.bench_with_input(BenchmarkId::new("bytecode", size), size, |b, _| {
            b.iter(|| run_bytecode(black_box(program.clone())));
        });
    }

    group.finish();
}

// ============================================================================
// Benchmark 2: Variable Operations
// ============================================================================

fn create_variable_program(iterations: usize) -> Vec<Stmt> {
    let mut stmts = Vec::new();

    // Initialize variables
    stmts.push(Stmt::Set {
        name: "x".to_string(),
        value: Expr::Num(10.0),
    });
    stmts.push(Stmt::Set {
        name: "y".to_string(),
        value: Expr::Num(20.0),
    });

    for _ in 0..iterations {
        // result = x + y
        stmts.push(Stmt::Set {
            name: "result".to_string(),
            value: Expr::Plus(
                Box::new(Expr::Ident("x".to_string())),
                Box::new(Expr::Ident("y".to_string())),
            ),
        });

        // x = result * 2
        stmts.push(Stmt::Set {
            name: "x".to_string(),
            value: Expr::Times(
                Box::new(Expr::Ident("result".to_string())),
                Box::new(Expr::Num(2.0)),
            ),
        });
    }

    stmts
}

fn bench_variables(c: &mut Criterion) {
    let mut group = c.benchmark_group("variables");

    for size in [10, 50, 100].iter() {
        let program = create_variable_program(*size);

        group.bench_with_input(BenchmarkId::new("ast", size), size, |b, _| {
            b.iter(|| run_ast(black_box(program.clone())));
        });

        group.bench_with_input(BenchmarkId::new("bytecode", size), size, |b, _| {
            b.iter(|| run_bytecode(black_box(program.clone())));
        });
    }

    group.finish();
}

// ============================================================================
// Benchmark 3: Conditionals
// ============================================================================

fn create_conditional_program(iterations: usize) -> Vec<Stmt> {
    let mut stmts = Vec::new();

    stmts.push(Stmt::Set {
        name: "counter".to_string(),
        value: Expr::Num(0.0),
    });

    for i in 0..iterations {
        let condition = if i % 2 == 0 {
            Expr::Cmp(
                CmpOp::Gt,
                Box::new(Expr::Num(10.0)),
                Box::new(Expr::Num(5.0)),
            )
        } else {
            Expr::Cmp(
                CmpOp::Lt,
                Box::new(Expr::Num(3.0)),
                Box::new(Expr::Num(7.0)),
            )
        };

        stmts.push(Stmt::IfBlock {
            cond: condition,
            then_body: vec![Stmt::Set {
                name: "counter".to_string(),
                value: Expr::Plus(
                    Box::new(Expr::Ident("counter".to_string())),
                    Box::new(Expr::Num(1.0)),
                ),
            }],
            otherwise_body: Some(vec![Stmt::Set {
                name: "counter".to_string(),
                value: Expr::Plus(
                    Box::new(Expr::Ident("counter".to_string())),
                    Box::new(Expr::Num(2.0)),
                ),
            }]),
        });
    }

    stmts
}

fn bench_conditionals(c: &mut Criterion) {
    let mut group = c.benchmark_group("conditionals");

    for size in [10, 50, 100].iter() {
        let program = create_conditional_program(*size);

        group.bench_with_input(BenchmarkId::new("ast", size), size, |b, _| {
            b.iter(|| run_ast(black_box(program.clone())));
        });

        group.bench_with_input(BenchmarkId::new("bytecode", size), size, |b, _| {
            b.iter(|| run_bytecode(black_box(program.clone())));
        });
    }

    group.finish();
}

// ============================================================================
// Benchmark 4: String Operations
// ============================================================================

fn create_string_program(iterations: usize) -> Vec<Stmt> {
    let mut stmts = Vec::new();

    stmts.push(Stmt::Set {
        name: "text".to_string(),
        value: Expr::StringLit("Hello ".to_string()),
    });

    for i in 0..iterations {
        stmts.push(Stmt::Set {
            name: format!("msg{}", i),
            value: Expr::StringConcat(
                Box::new(Expr::Ident("text".to_string())),
                Box::new(Expr::StringLit("World!".to_string())),
            ),
        });
    }

    stmts
}

fn bench_strings(c: &mut Criterion) {
    let mut group = c.benchmark_group("strings");

    for size in [10, 50, 100].iter() {
        let program = create_string_program(*size);

        group.bench_with_input(BenchmarkId::new("ast", size), size, |b, _| {
            b.iter(|| run_ast(black_box(program.clone())));
        });

        group.bench_with_input(BenchmarkId::new("bytecode", size), size, |b, _| {
            b.iter(|| run_bytecode(black_box(program.clone())));
        });
    }

    group.finish();
}

// ============================================================================
// Benchmark 5: Mixed Workload
// ============================================================================

fn create_mixed_program(iterations: usize) -> Vec<Stmt> {
    let mut stmts = Vec::new();

    // Initialize
    stmts.push(Stmt::Set {
        name: "x".to_string(),
        value: Expr::Num(10.0),
    });
    stmts.push(Stmt::Set {
        name: "y".to_string(),
        value: Expr::Num(5.0),
    });
    stmts.push(Stmt::Set {
        name: "text".to_string(),
        value: Expr::StringLit("Result: ".to_string()),
    });

    for i in 0..iterations {
        // Arithmetic
        stmts.push(Stmt::Set {
            name: "result".to_string(),
            value: Expr::Plus(
                Box::new(Expr::Times(
                    Box::new(Expr::Ident("x".to_string())),
                    Box::new(Expr::Num(2.0)),
                )),
                Box::new(Expr::Ident("y".to_string())),
            ),
        });

        // Conditional
        if i % 3 == 0 {
            stmts.push(Stmt::IfBlock {
                cond: Expr::Cmp(
                    CmpOp::Gt,
                    Box::new(Expr::Ident("result".to_string())),
                    Box::new(Expr::Num(20.0)),
                ),
                then_body: vec![Stmt::Set {
                    name: "x".to_string(),
                    value: Expr::Minus(
                        Box::new(Expr::Ident("x".to_string())),
                        Box::new(Expr::Num(1.0)),
                    ),
                }],
                otherwise_body: None,
            });
        }
    }

    stmts
}

fn bench_mixed(c: &mut Criterion) {
    let mut group = c.benchmark_group("mixed");

    for size in [10, 50, 100].iter() {
        let program = create_mixed_program(*size);

        group.bench_with_input(BenchmarkId::new("ast", size), size, |b, _| {
            b.iter(|| run_ast(black_box(program.clone())));
        });

        group.bench_with_input(BenchmarkId::new("bytecode", size), size, |b, _| {
            b.iter(|| run_bytecode(black_box(program.clone())));
        });
    }

    group.finish();
}

// ============================================================================
// Benchmark Configuration
// ============================================================================

criterion_group!(
    benches,
    bench_arithmetic,
    bench_variables,
    bench_conditionals,
    bench_mixed
);

criterion_main!(benches);
