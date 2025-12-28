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
pub struct SelectCore {
    pub result_columns: Vec<ResultColumn>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CompoundOperator {
    Union,
    UnionAll,
    Intersect,
    Except,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SelectStatementCore {
    Simple(Box<SelectCore>),
    Compounded(Box<CompoundedSelectCores>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct CompoundedSelectCores {
    pub lhs: SelectStatementCore,
    pub operator: CompoundOperator,
    pub rhs: SelectCore,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SelectStatement {
    pub core: SelectStatementCore,
}
