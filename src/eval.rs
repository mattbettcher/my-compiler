use std::collections::HashMap;

use crate::parse::{CompilerErr, Expr, Op, Statement, Value};

pub struct Evaluator {
    vars: HashMap<String, Value>,
    funcs: HashMap<String, Vec<Statement>>,
}

const MAIN: &str = "main";

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            vars: HashMap::new(),
            funcs: HashMap::new(),
        }
    }

    pub fn eval_main(&mut self) -> Result<Value, CompilerErr> { 
        if self.funcs.contains_key("main".into()) {
            let main= Expr::Call(MAIN.into());
            self.eval_expr(&main)
        } else {
            Err(CompilerErr::MainNotDef)
        }
    }

    // temp for testing!
    fn init_var(&mut self, name: &str, value: i64) {
        self.vars.insert(name.into(), Value::Int(value));
    }

    pub fn eval_statement(&mut self, stmt: &Statement) -> Result<Value, CompilerErr> {
        use Statement::*;
        match stmt {
            DeclareFn(name, block) => {
                self.funcs.insert(name.clone(), block.clone());
                Ok(Value::None)
            },
            Assign(name, expr) => {
                let value = self.eval_expr(expr)?;
                match value {
                    Value::Int(i) => { 
                        self.init_var(name, i);
                    },
                    _ => panic!(),
                };
                return Ok(value);
            },
            Return(expr) => {
                return self.eval_expr(expr);
            },
        }
    }

    /// simple postorder visitor pattern to evaluate an expression
    pub fn eval_expr(&mut self, e: &Expr) -> Result<Value, CompilerErr> {
        match e {
            Expr::Lit(v) => { Ok(*v) },
            Expr::Var(name) => {
                if let Some(v) = self.vars.get(name) {
                    Ok(*v)
                } else {
                    Err(CompilerErr::VariableNotInit)
                }
            },
            Expr::Call(name) => {
                let block = self.funcs.get(name);
                if block.is_some() {
                    let mut result = Value::None;
                    for statement in block.unwrap().clone() {
                        result = self.eval_statement(&statement)?;
                    }
                    Ok(result)
                } else {
                    Err(CompilerErr::FuncNotDef)
                }
            },
            Expr::BinOp(op, lhs, rhs) => {
                let x = self.eval_expr(lhs)?;
                let y = self.eval_expr(rhs)?;
                self.eval_bin_op(*op, x, y)
            },
        }
    }

    /// saw this trick on JTs onehour language - match types first!
    fn eval_bin_op(&self, op: Op, lhs: Value, rhs: Value) -> Result<Value, CompilerErr> {
        match (lhs, rhs) {
            (Value::Int(x), Value::Int(y)) => {
                match op {
                    Op::Add => Ok(Value::Int(x + y)),
                    Op::Sub => Ok(Value::Int(x - y)),
                    Op::Mul => Ok(Value::Int(x * y)),
                    Op::Div => Ok(Value::Int(x / y)),
                    Op::Exp => Ok(Value::Int(x.pow(y as u32))),
                }
            },
            (_, _) => panic!(),
        }
    }
}