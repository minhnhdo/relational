use std::fmt::{Debug, Formatter};

pub enum Value {
    String(String),
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(f, "'{}'", s),
        }
    }
}
