use std::fmt::{Debug, Formatter};

pub(crate) mod string;

pub enum Expression {
    Literal(Literal),
}

pub enum Literal {
    String(String),
}

impl Debug for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::String(s) => write!(f, "Literal::String({})", s),
        }
    }
}

impl Debug for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Literal(l) => write!(f, "{:?}", l),
        }
    }
}
