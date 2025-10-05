pub mod ast;
pub mod lexer;
#[allow(clippy::module_inception)]
pub mod parser;
pub mod phrases;

pub use ast::{CmpOp, Expr, Param, Program, Stmt};
pub use parser::parse;
