// Basic bytecode infrastructure tests
use pohlang::bytecode::{BytecodeChunk, Constant, ConstantPool, Instruction};

#[test]
fn test_instruction_basic() {
    let inst = Instruction::Add;
    assert_eq!(inst.size(), 1);
    assert_eq!(inst.name(), "Add");
    println!("✓ Instruction basic test passed");
}

#[test]
fn test_constant_pool_basic() {
    let mut pool = ConstantPool::new();
    let idx1 = pool.add_constant(Constant::Number(42.0));
    let idx2 = pool.add_constant(Constant::String("hello".to_string()));
    let idx3 = pool.add_constant(Constant::Number(42.0)); // duplicate

    assert_eq!(idx1, 0);
    assert_eq!(idx2, 1);
    assert_eq!(idx1, idx3); // deduplication works
    assert_eq!(pool.constants.len(), 2);
    println!("✓ Constant pool basic test passed");
}

#[test]
fn test_bytecode_chunk_basic() {
    let mut chunk = BytecodeChunk::new(1);
    chunk.constants.push(Constant::Number(10.0));
    chunk.code.push(Instruction::LoadConst(0));
    chunk.code.push(Instruction::Return);

    assert_eq!(chunk.instruction_count(), 2);
    assert!(chunk.size_bytes() > 0);
    println!("✓ Bytecode chunk basic test passed");
}

#[test]
fn test_all_instruction_variants_have_size() {
    // Test that all instruction variants have a size defined
    let instructions = vec![
        Instruction::LoadConst(0),
        Instruction::LoadTrue,
        Instruction::LoadFalse,
        Instruction::LoadNull,
        Instruction::LoadLocal(0),
        Instruction::StoreLocal(0),
        Instruction::LoadGlobal("x".to_string()),
        Instruction::StoreGlobal("x".to_string()),
        Instruction::Add,
        Instruction::Subtract,
        Instruction::Multiply,
        Instruction::Divide,
        Instruction::Negate,
        Instruction::Equal,
        Instruction::NotEqual,
        Instruction::Less,
        Instruction::LessEqual,
        Instruction::Greater,
        Instruction::GreaterEqual,
        Instruction::Not,
        Instruction::And,
        Instruction::Or,
        Instruction::Jump(0),
        Instruction::JumpIfFalse(0),
        Instruction::JumpIfTrue(0),
        Instruction::Loop(0),
        Instruction::Call(0),
        Instruction::Return,
        Instruction::BuildList(0),
        Instruction::BuildDict(0),
        Instruction::Index,
        Instruction::IndexStore,
        Instruction::PushTryHandler(0),
        Instruction::PopTryHandler,
        Instruction::Throw,
        Instruction::Print,
        Instruction::Input,
        Instruction::WriteFile,
        Instruction::ReadFile,
        Instruction::CreateWebServer,
        Instruction::AddRoute,
        Instruction::StartServer,
        Instruction::HtmlResponse,
        Instruction::JsonResponse,
        Instruction::Pop,
        Instruction::Duplicate,
        Instruction::Swap,
        Instruction::Halt,
    ];

    for inst in instructions {
        let size = inst.size();
        assert!(size > 0, "Instruction {} should have size > 0", inst.name());
    }
    println!("✓ All instruction variants have valid sizes");
}

#[test]
fn test_constant_types() {
    let num = Constant::Number(3.14);
    let string = Constant::String("test".to_string());
    let boolean = Constant::Boolean(true);
    let null = Constant::Null;

    // Just verify they can be created and stored
    let mut pool = ConstantPool::new();
    pool.add_constant(num);
    pool.add_constant(string);
    pool.add_constant(boolean);
    pool.add_constant(null);

    assert_eq!(pool.constants.len(), 4);
    println!("✓ All constant types work");
}
