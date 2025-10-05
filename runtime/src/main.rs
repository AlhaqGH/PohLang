use pohlang::{parser, vm};
use std::fs;
use std::path::PathBuf;

#[derive(clap::Parser, Debug)]
#[command(name = "pohlang", version, about = "PohLang compiler/runtime")]
struct Args {
    /// Run a .poh file with the embedded VM
    #[arg(long)]
    run: bool,

    /// Compile to bytecode (future: native)
    #[arg(long)]
    compile: bool,

    /// Ahead-of-time compile to a native executable (stub)
    #[arg(long)]
    aot: bool,

    /// Input .poh file
    input: PathBuf,

    /// Output path (for --compile or --aot)
    #[arg(short, long)]
    out: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = <Args as clap::Parser>::parse();

    // Load source program
    let src = fs::read_to_string(&args.input)?;
    let program = parser::parse(&src)?;

    if args.run {
        let mut vm = vm::Vm::with_base_dir(
            args.input
                .parent()
                .unwrap_or_else(|| std::path::Path::new("."))
                .to_path_buf(),
        );
        vm.execute(&program)?;
        return Ok(());
    }

    if args.compile {
        let bytecode = vm::compile(&program);
        let bc_path = args.out.unwrap_or_else(|| args.input.with_extension("pbc"));
        fs::write(&bc_path, bytecode)?;
        println!("Wrote {}", bc_path.display());
        return Ok(());
    }

    if args.aot {
        // Stub for AOT: for now, emit a tiny runner shell script or warn.
        // Future: link a static runtime and embed bytecode.
        eprintln!("AOT compilation is not yet implemented. Use --compile to generate bytecode.");
        return Ok(());
    }

    eprintln!("Nothing to do. Use --run, --compile, or --aot.");
    Ok(())
}
