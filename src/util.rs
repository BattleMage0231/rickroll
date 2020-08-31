// collection of data types
#[derive(Debug, Clone)]
pub enum RickrollObject {
    Int(i32),
    Float(f32),
    Bool(bool),
    Array(Vec<RickrollObject>),
    Char(char),
    Undefined,
}

// operators
#[derive(Debug)]
pub enum Operator {
    ArrayAccess,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    UnaryMinus,
    And,
    Or,
    Not,
    Greater,
    Less,
    GreaterEquals,
    LessEquals,
    Equals,
    NotEquals,
    LParen,
    RParen,
}

impl Operator {
    // checks if operator is unary
    pub fn is_unary(&self) -> bool {
        use Operator::*;
        match self {
            UnaryMinus | Not => true,
            _ => false,
        }
    }
}

// rickroll token
#[derive(Debug)]
pub enum Token {
    Value(RickrollObject),
    Operator(Operator),
}

// language constants
pub fn from_constant(constant: &String) -> Option<RickrollObject> {
    match &constant[..] {
        "TRUE" => Some(RickrollObject::Bool(true)),
        "FALSE" => Some(RickrollObject::Bool(false)),
        "UNDEFINED" => Some(RickrollObject::Undefined),
        "ARRAY" => Some(RickrollObject::Array(Vec::new())),
        _ => None,
    }
}
