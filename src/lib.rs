#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Int(i32),
    Float(f32),
}
#[derive(Debug, PartialEq, Clone)]
pub enum Types {
    Number(Number),
    String,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Arithmatic {
    Add,
    Subtract,
    Multiply,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Logical {
    OR,
    AND,
    NOT,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    ArithmaticOperator(Arithmatic),
    LogicalOperator(Logical),
    Assignment,
}
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Operator(Operator),
    Identifier,
    Comma,
    Constant,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Data {
    Char(char),
    String(Box<String>),
}
