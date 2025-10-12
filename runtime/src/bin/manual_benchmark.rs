/// Simple manual benchmarks to quickly measure AST vs Bytecode performance
/// Run with: cargo run --release --bin manual_benchmark

use pohlang::{parser, bytecode, vm};
use pohlang::parser::ast::{Expr, Stmt};
use pohlang::bytecode::{Compiler, BytecodeVM};
use std::time::Instant;

fn run_bytecode(program: Vec<Stmt>) {
    let compiler = Compiler::new();
    let chunk = compiler.compile(program).expect("Compilation failed");
    let mut vm = BytecodeVM::new();
    vm.load(chunk);
    vm.run().expect("Execution failed");
}

fn run_ast(program: Vec<Stmt>) {
    let mut vm = vm::Vm::default();
    vm.execute(&program).expect("Execution failed");
}

fn benchmark(name: &str, program: Vec<Stmt>, iterations: usize) {
    println!("\n{}", "=".repeat(60));
    println!("Benchmark: {} ({} iterations)", name, iterations);
    println!("{}", "=".repeat(60));
    
    // Warm up
    for _ in 0..5 {
        run_ast(program.clone());
        run_bytecode(program.clone());
    }
    
    // Benchmark AST
    let ast_start = Instant::now();
    for _ in 0..iterations {
        run_ast(program.clone());
    }
    let ast_duration = ast_start.elapsed();
    let ast_ms = ast_duration.as_secs_f64() * 1000.0;
    
    // Benchmark Bytecode
    let bc_start = Instant::now();
    for _ in 0..iterations {
        run_bytecode(program.clone());
    }
    let bc_duration = bc_start.elapsed();
    let bc_ms = bc_duration.as_secs_f64() * 1000.0;
    
    // Calculate speedup
    let speedup = ast_ms / bc_ms;
    
    println!("AST Interpreter:  {:.3} ms ({:.3} ms per iteration)", ast_ms, ast_ms / iterations as f64);
    println!("Bytecode VM:      {:.3} ms ({:.3} ms per iteration)", bc_ms, bc_ms / iterations as f64);
    println!("Speedup:          {:.2}x faster", speedup);
    
    if speedup >= 10.0 {
        println!("Status:           ✓ EXCELLENT (>10x speedup)");
    } else if speedup >= 5.0 {
        println!("Status:           ✓ GOOD (5-10x speedup)");
    } else if speedup >= 2.0 {
        println!("Status:           ~ OK (2-5x speedup)");
    } else {
        println!("Status:           ✗ POOR (<2x speedup)");
    }
}

fn create_arithmetic_benchmark(size: usize) -> Vec<Stmt> {
    let mut stmts = Vec::new();
    
    // Initialize a variable to store results
    stmts.push(Stmt::Set {
        name: "result".to_string(),
        value: Expr::Num(0.0),
    });
    
    for _ in 0..size {
        // result = ((5 * 3) + (10 / 2)) - 2 = 18
        stmts.push(Stmt::Set {
            name: "result".to_string(),
            value: Expr::Minus(
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
            ),
        });
    }
    stmts
}

fn create_variable_benchmark(size: usize) -> Vec<Stmt> {
    let mut stmts = Vec::new();
    
    stmts.push(Stmt::Set {
        name: "x".to_string(),
        value: Expr::Num(10.0),
    });
    stmts.push(Stmt::Set {
        name: "y".to_string(),
        value: Expr::Num(20.0),
    });
    
    for _ in 0..size {
        stmts.push(Stmt::Set {
            name: "result".to_string(),
            value: Expr::Plus(
                Box::new(Expr::Ident("x".to_string())),
                Box::new(Expr::Ident("y".to_string())),
            ),
        });
        
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

fn create_conditional_benchmark(size: usize) -> Vec<Stmt> {
    let mut stmts = Vec::new();
    
    stmts.push(Stmt::Set {
        name: "counter".to_string(),
        value: Expr::Num(0.0),
    });
    
    for i in 0..size {
        let condition = if i % 2 == 0 {
            Expr::Cmp(
                parser::ast::CmpOp::Gt,
                Box::new(Expr::Num(10.0)),
                Box::new(Expr::Num(5.0)),
            )
        } else {
            Expr::Cmp(
                parser::ast::CmpOp::Lt,
                Box::new(Expr::Num(3.0)),
                Box::new(Expr::Num(7.0)),
            )
        };
        
        stmts.push(Stmt::IfBlock {
            cond: condition,
            then_body: vec![
                Stmt::Set {
                    name: "counter".to_string(),
                    value: Expr::Plus(
                        Box::new(Expr::Ident("counter".to_string())),
                        Box::new(Expr::Num(1.0)),
                    ),
                }
            ],
            otherwise_body: Some(vec![
                Stmt::Set {
                    name: "counter".to_string(),
                    value: Expr::Plus(
                        Box::new(Expr::Ident("counter".to_string())),
                        Box::new(Expr::Num(2.0)),
                    ),
                }
            ]),
        });
    }
    
    stmts
}

fn create_mixed_benchmark(size: usize) -> Vec<Stmt> {
    let mut stmts = Vec::new();
    
    stmts.push(Stmt::Set {
        name: "x".to_string(),
        value: Expr::Num(10.0),
    });
    stmts.push(Stmt::Set {
        name: "y".to_string(),
        value: Expr::Num(5.0),
    });
    
    for i in 0..size {
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
        
        // Conditional every 3rd iteration
        if i % 3 == 0 {
            stmts.push(Stmt::IfBlock {
                cond: Expr::Cmp(
                    parser::ast::CmpOp::Gt,
                    Box::new(Expr::Ident("result".to_string())),
                    Box::new(Expr::Num(20.0)),
                ),
                then_body: vec![
                    Stmt::Set {
                        name: "x".to_string(),
                        value: Expr::Minus(
                            Box::new(Expr::Ident("x".to_string())),
                            Box::new(Expr::Num(1.0)),
                        ),
                    }
                ],
                otherwise_body: None,
            });
        }
    }
    
    stmts
}

fn main() {
    println!("\n{}", "=".repeat(60));
    println!("PohLang Bytecode VM Performance Benchmarks");
    println!("{}", "=".repeat(60));
    println!("\nComparing AST Interpreter vs Bytecode VM performance");
    println!("Goal: Achieve 5-15x speedup with bytecode execution");
    
    // Run benchmarks
    let iterations = 100;
    
    benchmark("Arithmetic Operations (50 ops)", 
              create_arithmetic_benchmark(50), 
              iterations);
    
    benchmark("Variable Operations (50 ops)", 
              create_variable_benchmark(50), 
              iterations);
    
    benchmark("Conditional Branches (50 ops)", 
              create_conditional_benchmark(50), 
              iterations);
    
    benchmark("Mixed Workload (50 ops)", 
              create_mixed_benchmark(50), 
              iterations);
    
    println!("\n{}", "=".repeat(60));
    println!("Summary");
    println!("{}", "=".repeat(60));
    println!("All benchmarks complete! Check results above.");
    println!("Expected: 5-15x speedup across all workloads");
    println!("{}", "=".repeat(60));
    println!();
}
