use core::panic;
use std::collections::HashMap;

use crate::lex::{Lex, LexValue, TokenKind};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Value {
    Int(i64),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
}

impl From<TokenKind> for Op {
    fn from(v: TokenKind) -> Self {
        match v {
            TokenKind::Asterisk => Op::Mul,
            TokenKind::Slash => Op::Div,
            TokenKind::Plus => Op::Add,
            TokenKind::Hyphen => Op::Sub,
            TokenKind::Circumflex => Op::Exp,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Lit(Value),
    Var(String),
    BinOp(Op, Box<Expr>, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Assign(String, Expr),
    Expr(Expr),
}

#[derive(Debug)]
pub enum CompilerErr {
    Unknown,
    InvalidAtom,
    VariableNotInit,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Assoc {
    Left,
    Right
}

pub struct Parse {
    op_map: HashMap<TokenKind, (usize, Assoc)>,
}

impl Parse {
    pub fn new() -> Self {
        Parse {
            op_map:
                [(TokenKind::Plus, (1, Assoc::Left)),
                (TokenKind::Hyphen, (1, Assoc::Left)),
                (TokenKind::Asterisk, (2, Assoc::Left)),
                (TokenKind::Slash, (2, Assoc::Left)),
                (TokenKind::Circumflex, (3, Assoc::Right))]
                .iter().cloned().collect(),
        }
    }

    fn parse_ident(&mut self, l: &mut Lex) -> Result<String, CompilerErr> {
        let mut result = Err(CompilerErr::Unknown);
        if let Some(t) = &l.cur {
            match t.kind {
                TokenKind::Ident => {
                    match &t.value {
                        LexValue::Ident(name) => {
                            result = Ok(name.clone());
                            l.next();
                        },
                        _ => panic!(),
                    }
                },
                _ => panic!(),
            }
        }
        result
    }

    pub fn parse_statements(&mut self, l: &mut Lex) -> Result<Vec<Statement>, CompilerErr> {
        let mut stmts = vec![];
        // 2 type of statements for now
        loop {
            if let Some(t) =  &l.cur {
                match t.kind {
                    TokenKind::Let => { stmts.push(self.parse_assign(l)?); },
                    _ => { 
                        stmts.push(Statement::Expr(self.parse_expr(l, 0)?));
                        l.expect(TokenKind::SemiColon);
                    },
                }    
            } else {
                break;
            }
        }
        Ok(stmts)
    }

    pub fn parse_assign(&mut self, l: &mut Lex) -> Result<Statement, CompilerErr> {
        l.expect(TokenKind::Let);
        let name = self.parse_ident(l)?;
        l.expect(TokenKind::Eq);
        let expr = self.parse_expr(l, 0)?;
        l.expect(TokenKind::SemiColon);
        Ok(Statement::Assign(name, expr))
    }

    pub fn parse_expr(&mut self, l: &mut Lex, min_prec: usize) -> Result<Expr, CompilerErr> {
        let mut lhs = self.parse_atom(l)?;
    
        loop {
            if let Some(t) = &l.cur {
                if let Some((prec, assoc)) = self.op_map.get(&t.kind) {
                    if *prec < min_prec {
                        break;
                    }
                    let next_min_prec = if *assoc == Assoc::Left { *prec + 1 } else { *prec };
                    
                    let op = Op::from(t.kind);
                    l.next();
    
                    let rhs = self.parse_expr(l, next_min_prec)?;
                    lhs = Expr::BinOp(op, Box::new(lhs), Box::new(rhs));
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        return Ok(lhs)
    }
    
    fn parse_atom(&mut self, l: &mut Lex) -> Result<Expr, CompilerErr> {
        use TokenKind::*;
        let mut result = Err(CompilerErr::Unknown);
        
        if let Some(t) = &l.cur {
            match &t.kind {
                Int => {
                    match &t.value {
                        LexValue::Int(n) => { 
                            result = Ok(Expr::Lit(Value::Int(*n)));
                            l.next();
                        },
                        _ => panic!(), 
                    }
                },
                Ident => { 
                    match &t.value {
                        LexValue::Ident(n) => { 
                            result = Ok(Expr::Var(n.clone()));
                            l.next();
                        },
                        _ => panic!(),
                    }
                },
                LeftParen => { 
                    l.next();
                    result = self.parse_expr(l, 1);
                    if !l.expect(TokenKind::RightParen) { 
                        // lexer will give an error here
                        // we choose to carry on...for now 😎
                    }
                },
                _ => {
                    // error
                    println!("Parsed `{:?}` but expected a `(` or `int`", t.kind);
                    return Err(CompilerErr::InvalidAtom);
                },
            }
        }
        result
    }
}

#[test]
fn parse_expr_test() -> Result<(), CompilerErr> {
    let mut l = Lex::new("1 + 2");
    let mut p = Parse::new();
    let result = p.parse_expr(&mut l, 0)?;
    assert_eq!(result, Expr::BinOp(Op::Add, Box::new(Expr::Lit(Value::Int(1))), Box::new(Expr::Lit(Value::Int(2)))));
    Ok(())
}

#[test]
fn parse_assign_test() -> Result<(), CompilerErr> {
    let mut l = Lex::new("let my_var = 10 + 11");
    let mut p = Parse::new();
    let result = p.parse_assign(&mut l)?;
    assert_eq!(result, Statement::Assign("my_var".into(), 
        Expr::BinOp(Op::Add, Box::new(Expr::Lit(Value::Int(10))), Box::new(Expr::Lit(Value::Int(11))))));
    Ok(())
}