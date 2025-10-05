pub mod core;
pub mod parser;
pub mod vm;

// Optional: expose a simple runtime API
pub fn execute_program(program: &parser::Program) -> anyhow::Result<()> {
    let mut v = vm::Vm::default();
    v.execute(program)
}
