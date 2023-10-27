//The type checker will depend on output of the parser

mod type_checker;

//Import parser module
use crate::parser;

pub use type_checker::*;