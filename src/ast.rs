use std::fmt;

#[derive(Debug)]
pub enum Expression {
    Num(i32),
    Increment(Box<Expression>),
    ExpressionOp {op: String, left: Box<Expression>, right: Box<Expression> }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Expression::Num(x) => { write!(f, "{}", x.to_string()) },
            &Expression::Increment(ref expr) => { write!(f, "(++{})", expr) },
            &Expression::ExpressionOp {ref op, ref left, ref right } => {
                write!(f, "({} {} {})", left, op.to_string(), right)
            }
        }
    }
}
