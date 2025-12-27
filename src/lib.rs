#![feature(box_patterns)]

mod compiler;
pub(crate) mod lexer;
pub(crate) mod parser;

pub use compiler::compile;



