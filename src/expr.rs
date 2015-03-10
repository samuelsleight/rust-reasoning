use std::fmt::{Display, Formatter, Result};

pub enum Expr {
    Atom(String),
    Not(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    IfThen(Box<Expr>, Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        match self {
            &Expr::Atom(ref str) => write!(fmt, "{}", str),
            &Expr::Not(ref e) => write!(fmt, "¬({})", e),
            &Expr::And(ref e1, ref e2) => write!(fmt, "({}) ∧ ({})", e1, e2),
            &Expr::Or(ref e1, ref e2) => write!(fmt, "({}) ∨ ({})", e1, e2),
            &Expr::IfThen(ref e1, ref e2) => write!(fmt, "({}) → ({})", e1, e2),
        }
    }
}
