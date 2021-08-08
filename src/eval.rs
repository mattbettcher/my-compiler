use crate::parse::{Expr, Value, Op};

pub struct Evaluator {}

impl Evaluator {
    /// simple postorder visitor pattern to evaluate an expression
    pub fn compute_expr(&self, e: Expr) -> Value {
        match e {
            Expr::Lit(v) => { v },
            Expr::BinOp(op, lhs, rhs) => {
                let x = self.compute_expr(*lhs);
                let y = self.compute_expr(*rhs);
                self.compute_op(op, x, y)
            },
        }
    }

    /// saw this trick on JTs onehour language - match types first!
    fn compute_op(&self, op: Op, lhs: Value, rhs: Value) -> Value {
        match (lhs, rhs) {
            (Value::Int(x), Value::Int(y)) => {
                match op {
                    Op::Add => Value::Int(x + y),
                    Op::Sub => Value::Int(x - y),
                    Op::Mul => Value::Int(x * y),
                    Op::Div => Value::Int(x / y),
                    Op::Exp => Value::Int(x.pow(y as u32)),
                }
            },
        }
    }
}