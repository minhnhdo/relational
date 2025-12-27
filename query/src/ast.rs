pub(crate) mod string;

#[derive(Debug, PartialEq, Eq)]
pub enum Literal {
    String(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Literal(Literal),
}
