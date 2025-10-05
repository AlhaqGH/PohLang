pub mod instructions;
#[allow(clippy::module_inception)]
pub mod vm;

pub use vm::{compile, Vm};
