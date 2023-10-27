//Translator will depend on results of type_checker, parser
mod translator;

use crate::parser;
use crate::type_checker;

pub use translator::*;