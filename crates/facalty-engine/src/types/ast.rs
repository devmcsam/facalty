use super::glue::functions::Function;
pub struct Node {
    left_leaf: Option<Box<Self>>,
    right_leaf: Option<Box<Self>>,
}

/// An operation that can be performed between two nodes
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow, // ^
    Rem, // %
}

pub enum Symbol {
    LParen,
    RParen,
    Number,
    Variable(char),
    Operator(Operator),
    BuiltInFunction(Function),
    UserFunction,
}
