use pohlang::{bytecode, parser, vm};
use std::fs;
use std::path::PathBuf;

#[derive(clap::Parser, Debug)]
#[command(name = "pohlang", version, about = "PohLang compiler/runtime")]
struct Args {
    /// Run a .poh file with the embedded VM
    #[arg(long)]
    run: bool,

    /// Enable hot reload / watch mode (like Flutter)
    #[arg(long)]
    watch: bool,

    /// Compile to bytecode .pbc file
    #[arg(long)]
    compile: bool,

    /// Compile and run with bytecode VM
    #[arg(long)]
    bytecode: bool,

    /// Run pre-compiled .pbc bytecode file
    #[arg(long)]
    run_bytecode: bool,

    /// Disassemble .pbc file to show bytecode instructions
    #[arg(long)]
    disassemble: bool,

    /// Show execution statistics (instruction counts, timing, cache stats)
    #[arg(long)]
    stats: bool,

    /// Ahead-of-time compile to a native executable (stub)
    #[arg(long)]
    aot: bool,

    /// Input .poh or .pbc file
    input: PathBuf,

    /// Output path (for --compile or --aot)
    #[arg(short, long)]
    out: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = <Args as clap::Parser>::parse();

    // Handle --run-bytecode: Execute pre-compiled .pbc file
    if args.run_bytecode {
        let bytes = fs::read(&args.input)?;
        let chunk = bytecode::BytecodeDeserializer::deserialize(&bytes)?;
        let mut vm = bytecode::BytecodeVM::new();
        vm.load(chunk);
        let _result = vm.run()?;

        // Show statistics if requested
        if args.stats {
            // vm.print_stats();
        }
        return Ok(());
    }

    // Handle --disassemble: Show bytecode instructions
    if args.disassemble {
        let bytes = fs::read(&args.input)?;
        let chunk = bytecode::BytecodeDeserializer::deserialize(&bytes)?;
        println!("=== Bytecode Disassembly ===");
        println!("Version: {}", chunk.version);
        println!("Constants: {} entries", chunk.constants.len());
        for (i, constant) in chunk.constants.iter().enumerate() {
            println!("  [{}] {:?}", i, constant);
        }
        println!("\nCode: {} instructions", chunk.code.len());
        for (i, instruction) in chunk.code.iter().enumerate() {
            println!("  {:04} {:?}", i, instruction);
        }
        if let Some(debug_info) = &chunk.debug_info {
            println!("\nDebug Info:");
            println!("  Source: {}", debug_info.source_file);
            println!("  Line numbers: {} entries", debug_info.line_numbers.len());
            println!("  Variables: {} entries", debug_info.variable_names.len());
        }
        return Ok(());
    }

    // For all other modes, we need to parse the source
    let src = fs::read_to_string(&args.input)?;
    let program = parser::parse(&src)?;

    // Handle --compile: Compile .poh to .pbc
    if args.compile {
        let compiler = bytecode::Compiler::new();
        let chunk = compiler.compile(program)?;
        let bc_path = args.out.unwrap_or_else(|| args.input.with_extension("pbc"));
        bytecode::BytecodeSerializer::save_to_file(&chunk, &bc_path)?;
        println!("✓ Compiled to {}", bc_path.display());
        println!(
            "  {} constants, {} instructions",
            chunk.constants.len(),
            chunk.code.len()
        );
        return Ok(());
    }

    // Handle --bytecode: Compile and run with bytecode VM
    if args.bytecode {
        let compiler = bytecode::Compiler::new();
        let chunk = compiler.compile(program)?;
        let mut vm = bytecode::BytecodeVM::new();
        vm.load(chunk);
        let _result = vm.run()?;

        // Show statistics if requested
        if args.stats {
            // vm.print_stats();
        }
        return Ok(());
    }

    // Handle --run: Execute with AST interpreter
    if args.run {
        let base_dir = args
            .input
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .to_path_buf();

        let mut vm = vm::Vm::with_base_dir(base_dir.clone());

        // Set the current file being executed
        vm.set_current_file(args.input.display().to_string());

        // Enable hot reload if --watch flag is set
        if args.watch {
            vm.enable_hot_reload(vec![base_dir.clone()]);
            println!("🔥 Hot reload enabled! Changes will be detected automatically.");
            println!("💡 Watching: {}", base_dir.clone().display());
        }

        vm.execute(&program)?;
        return Ok(());
    }

    // Handle --aot: Stub for future AOT compilation
    if args.aot {
        eprintln!("AOT compilation is not yet implemented. Use --compile to generate bytecode.");
        return Ok(());
    }

    // Default: show help
    eprintln!("Nothing to do. Use one of:");
    eprintln!("  --run           Execute with AST interpreter");
    eprintln!("  --bytecode      Compile and run with bytecode VM");
    eprintln!("  --compile       Compile to .pbc file");
    eprintln!("  --run-bytecode  Execute .pbc file");
    eprintln!("  --disassemble   Show bytecode instructions");
    Ok(())
}
