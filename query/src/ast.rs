pub(crate) mod string;

#[derive(Debug, PartialEq, Eq)]
pub enum Literal {
    String(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Literal(Literal),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ResultColumn {
    Expression(Expression),
}

#[derive(Debug, PartialEq, Eq)]
pub struct SelectStatement {
    pub result_columns: Vec<ResultColumn>,
}
