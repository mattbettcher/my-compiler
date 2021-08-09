 
// we will handle lints later...
//#![allow(dead_code, unused_variables, unused_labels, unused_imports, unused_mut)]

mod lex;
mod parse;
mod eval;

use lex::Lex;
use crate::{eval::Evaluator, parse::{CompilerErr, Parse}};

fn main() -> Result<(), CompilerErr> {
    for arg in std::env::args().skip(1) {
        let contents = std::fs::read_to_string(arg).unwrap();
        let mut l = Lex::new(&contents);
        let mut p = Parse::new();
        let statements = p.parse_statements(&mut l)?;
        let mut eval = Evaluator::new();
        for statement in &statements {
            eval.eval_statement(statement)?;
        }
        let result = eval.eval_main();
        println!("Final result: {:?}", result);
    }
    
    Ok(())
}
