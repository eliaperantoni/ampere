use crate::scanner::Token;

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Num(f64),
    Bool(bool),
    String(String),
}

impl Value {
    pub fn stringify(&self) -> String {
        use Value::*;
        match self {
            String(string) => string.clone(),
            Num(num) => num.to_string(),
            Bool(bool) => bool.to_string(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Literal(Value),
    Interp { segments: Vec<String>, exprs: Vec<Expr> },
    Unary { op: Token, rhs: Box<Expr> },
    Binary { lhs: Box<Expr>, rhs: Box<Expr>, op: Token },
}

impl From<Value> for Expr {
    fn from(value: Value) -> Self {
        Expr::Literal(value)
    }
}
