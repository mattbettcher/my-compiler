
// we will handle lints later...
//#![allow(dead_code, unused_variables, unused_labels, unused_imports, unused_mut)]

mod lex;
mod parse;
mod eval;

use lex::Lex;
use parse::CompilerErr;
use crate::{eval::Evaluator, parse::Parse};
use std::io::{stdin,stdout,Write};

fn main() -> Result<(), CompilerErr> {
    let mut s=String::new();
    print!("Please enter an expression: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("You did not enter an expression. Try again with something like 1 + 2 * 3");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    let mut l = Lex::new(&s);  // 1 + (6 / 2) * 3 = 10
    let mut p = Parse::new();
    let expr = p.parse_expr(&mut l, 1)?;
    println!("AST:\n{:#?}", expr);
    let mut eval = Evaluator::new();
    eval.init_var("x", 6);
    let result = eval.compute_expr(expr);
    println!("Final result: {:?}", result);
    
    Ok(())
}


