use std::fmt::{Debug, Formatter};

pub(crate) mod string;

pub enum Value {
    Literal(Literal),
}

pub enum Literal {
    String(String),
}

impl Debug for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::String(s) => write!(f, "'{}'", s),
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Literal(l) => write!(f, "{:?}", l),
        }
    }
}
