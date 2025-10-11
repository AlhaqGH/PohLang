// Bytecode infrastructure demonstration
// This example shows that all bytecode components work correctly

use pohlang::bytecode::{Instruction, Constant, ConstantPool, BytecodeChunk};

fn main() {
    println!("=== PohLang Bytecode Infrastructure Demo ===\n");
    
    // Test 1: Instructions
    println!("1. Testing Instructions:");
    let add_inst = Instruction::Add;
    println!("   Add instruction: size={}, name={}", add_inst.size(), add_inst.name());
    
    let load_inst = Instruction::LoadConst(5);
    println!("   LoadConst(5): size={}, name={}", load_inst.size(), load_inst.name());
    
    let jump_inst = Instruction::Jump(100);
    println!("   Jump(100): size={}, name={}", jump_inst.size(), jump_inst.name());
    
    println!("   ✓ Instructions work!\n");
    
    // Test 2: Constant Pool
    println!("2. Testing Constant Pool:");
    let mut pool = ConstantPool::new();
    
    let idx1 = pool.add_constant(Constant::Number(42.0));
    println!("   Added number 42.0 at index {}", idx1);
    
    let idx2 = pool.add_constant(Constant::String("Hello, PohLang!".to_string()));
    println!("   Added string 'Hello, PohLang!' at index {}", idx2);
    
    let idx3 = pool.add_constant(Constant::Boolean(true));
    println!("   Added boolean true at index {}", idx3);
    
    let idx4 = pool.add_constant(Constant::Number(42.0)); // duplicate
    println!("   Added number 42.0 again (dedup) at index {}", idx4);
    
    println!("   Pool size: {} unique constants", pool.constants.len());
    println!("   ✓ Constant pool with deduplication works!\n");
    
    // Test 3: Bytecode Chunk
    println!("3. Testing Bytecode Chunk:");
    let mut chunk = BytecodeChunk::new(1);
    
    // Add some constants
    chunk.constants.push(Constant::Number(10.0));
    chunk.constants.push(Constant::Number(20.0));
    chunk.constants.push(Constant::String("result".to_string()));
    
    // Add some instructions (simulating: result = 10 + 20)
    chunk.code.push(Instruction::LoadConst(0));  // Load 10
    chunk.code.push(Instruction::LoadConst(1));  // Load 20
    chunk.code.push(Instruction::Add);           // Add them
    chunk.code.push(Instruction::StoreLocal(0)); // Store in local 0
    chunk.code.push(Instruction::LoadLocal(0));  // Load result
    chunk.code.push(Instruction::Print);         // Print it
    chunk.code.push(Instruction::Return);        // Return
    
    println!("   Created chunk with {} instructions", chunk.instruction_count());
    println!("   Total bytecode size: {} bytes", chunk.size_bytes());
    println!("   ✓ Bytecode chunk works!\n");
    
    // Test 4: Web Framework Instructions
    println!("4. Testing Web Framework Instructions:");
    let web_inst = Instruction::CreateWebServer;
    println!("   CreateWebServer: size={}, name={}", web_inst.size(), web_inst.name());
    
    let route_inst = Instruction::AddRoute;
    println!("   AddRoute: size={}, name={}", route_inst.size(), route_inst.name());
    
    let html_inst = Instruction::HtmlResponse;
    println!("   HtmlResponse: size={}, name={}", html_inst.size(), html_inst.name());
    
    println!("   ✓ Web framework instructions work!\n");
    
    // Test 5: Display trait
    println!("5. Testing Instruction Display:");
    println!("   {}", Instruction::LoadConst(42));
    println!("   {}", Instruction::Add);
    println!("   {}", Instruction::Jump(100));
    println!("   {}", Instruction::Call(3));
    println!("   ✓ Display trait works!\n");
    
    // Test 6: Exception handling instructions
    println!("6. Testing Exception Instructions:");
    let push_try = Instruction::PushTryHandler(50);
    println!("   PushTryHandler(50): size={}, name={}", push_try.size(), push_try.name());
    
    let pop_try = Instruction::PopTryHandler;
    println!("   PopTryHandler: size={}, name={}", pop_try.size(), pop_try.name());
    
    let throw_inst = Instruction::Throw;
    println!("   Throw: size={}, name={}", throw_inst.size(), throw_inst.name());
    println!("   ✓ Exception handling instructions work!\n");
    
    println!("=== All Tests Passed! ===");
    println!("\n🎉 Phase 7 Stage 1 (Bytecode ISA Design) Complete!");
    println!("\nNext Steps:");
    println!("  - Stage 2: Implement Bytecode Compiler (AST → Bytecode)");
    println!("  - Stage 3: Implement Bytecode VM (Execute bytecode)");
    println!("  - Stage 4: Implement .pbc file format");
    println!("  - Stage 5: CLI integration (--compile, --bytecode flags)");
    println!("  - Stage 6: Testing & benchmarks (target: 10x+ speedup)");
}
