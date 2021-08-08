use std::collections::HashMap;

use crate::parse::{CompilerErr, Expr, Op, Value};

pub struct Evaluator {
    vars: HashMap<String, Value>,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            vars: HashMap::new(),
        }
    }

    // temp for testing!
    pub fn init_var(&mut self, name: &str, value: i64) {
        self.vars.insert(name.into(), Value::Int(value));
    }

    /// simple postorder visitor pattern to evaluate an expression
    pub fn compute_expr(&self, e: Expr) -> Result<Value, CompilerErr> {
        match e {
            Expr::Lit(v) => { Ok(v) },
            Expr::Var(name) => {
                if let Some(v) = self.vars.get(&name) {
                    Ok(*v)
                } else {
                    Err(CompilerErr::VariableNotInit)
                }
            },
            Expr::BinOp(op, lhs, rhs) => {
                if let Ok(x) = self.compute_expr(*lhs) {
                    if let Ok(y) = self.compute_expr(*rhs) {
                        return self.compute_op(op, x, y);
                    }
                }
                Err(CompilerErr::Unknown)   // what error here?
            },
        }
    }

    /// saw this trick on JTs onehour language - match types first!
    fn compute_op(&self, op: Op, lhs: Value, rhs: Value) -> Result<Value, CompilerErr> {
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
        }
    }
}