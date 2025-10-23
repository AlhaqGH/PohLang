/// VM Benchmark - Compare AST vs Bytecode Performance
///
/// This example demonstrates Phase 8 optimizations including:
/// - Inline caching for global variables
/// - Enhanced error messages with line numbers
/// - VM execution statistics
use pohlang::bytecode::{BytecodeVM, Compiler};
use pohlang::parser;
use std::time::Instant;

fn main() {
    println!("=== PohLang VM Benchmark (Phase 8) ===\n");

    // Test 1: Variable-heavy program (tests inline caching)
    println!("Test 1: Variable Operations (Inline Cache Test)");
    let program1 = r#"
Start Program
Set x to 0
Set y to 0
Repeat 1000 times
    Set x to x plus 1
    Set y to y plus 2
End Repeat
Write x
Write y
End Program
    "#;
    run_benchmark("Variable Operations", program1);

    // Test 2: Arithmetic-heavy program
    println!("\nTest 2: Arithmetic Operations");
    let program2 = r#"
Start Program
Set result to 0
Repeat 500 times
    Set result to result plus 10
    Set result to result times 2
    Set result to result divided by 2
End Repeat
Write result
End Program
    "#;
    run_benchmark("Arithmetic Operations", program2);

    // Test 3: Nested operations (stack depth test)
    println!("\nTest 3: Nested Expressions");
    let program3 = r#"
Start Program
Set a to 5
Set b to 3
Set c to 2
Set result to a times b plus c times a minus b
Write result
End Program
    "#;
    run_benchmark("Nested Expressions", program3);

    println!("\n=== Benchmark Complete ===");
}

fn run_benchmark(name: &str, source: &str) {
    // Parse the program
    let program = match parser::parse(source) {
        Ok(prog) => prog,
        Err(e) => {
            eprintln!("Parse error: {:?}", e);
            return;
        }
    };

    // Compile to bytecode
    let compiler = Compiler::new();
    let chunk = match compiler.compile(program) {
        Ok(chunk) => chunk,
        Err(e) => {
            eprintln!("Compile error: {:?}", e);
            return;
        }
    };

    println!("  Instructions: {}", chunk.instruction_count());
    println!("  Bytecode size: {} bytes", chunk.size_bytes());

    // Run with statistics enabled
    let mut vm = BytecodeVM::new();
    vm.enable_stats();
    vm.load(chunk);

    let start = Instant::now();
    match vm.run() {
        Ok(_) => {
            let duration = start.elapsed();
            println!("  Execution time: {:.2?}", duration);

            // Print statistics
            if let Some(stats) = vm.get_stats() {
                println!(
                    "  Total instructions executed: {}",
                    stats.total_instructions
                );
                println!("  Max stack depth: {}", stats.max_stack_depth);

                let total_cache = stats.cache_hits + stats.cache_misses;
                if total_cache > 0 {
                    let hit_rate = (stats.cache_hits as f64 / total_cache as f64) * 100.0;
                    println!(
                        "  Cache hit rate: {:.1}% ({}/{})",
                        hit_rate, stats.cache_hits, total_cache
                    );
                }

                if stats.total_instructions > 0 {
                    let ips = stats.total_instructions as f64 / duration.as_secs_f64();
                    println!("  Instructions/sec: {:.0}", ips);
                }
            }
        }
        Err(e) => {
            eprintln!("  Runtime error: {}", e);
            // Print partial statistics even on error
            if let Some(report) = vm.stats_report() {
                println!("\n{}", report);
            }
        }
    }
}
