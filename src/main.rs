
// we will handle lints later...
//#![allow(dead_code, unused_variables, unused_labels, unused_imports, unused_mut)]

mod lex;
mod parse;
mod eval;

use lex::Lex;
use crate::{eval::Evaluator, parse::{CompilerErr, Parse, Value}};

fn main() -> Result<(), CompilerErr> {
    for arg in std::env::args().skip(1) {
        let contents = std::fs::read_to_string(arg).unwrap();
        let mut l = Lex::new(&contents);
        let mut p = Parse::new();
        let statements = p.parse_statements(&mut l)?;
        let mut eval = Evaluator::new();
        let mut result = Value::Int(0);
        for statement in &statements {
            result = eval.eval_statement(statement)?;
        }
        println!("Final result: {:?}", result);
    }
    
    Ok(())
}
