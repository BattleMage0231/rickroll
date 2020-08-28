// collection of data types
#[derive(Debug)]
pub enum RickrollObject {
    Int(i32),
    Float(f32),
    Bool(bool),
    Array(Vec<RickrollObject>),
    Char(char),
    Undefined,
}

impl std::string::ToString for RickrollObject {
    fn to_string(&self) -> String {
        match self {
            RickrollObject::Undefined => String::from("Undefined"),
            RickrollObject::Int(x) => x.to_string(),
            RickrollObject::Float(x) => x.to_string(),
            RickrollObject::Bool(x) => x.to_string(),
            RickrollObject::Array(x) => {
                // array to string recursively
                let mut res = String::from("[");
                for i in 0..x.len() {
                    res += &x[i].to_string()[..];
                    if i != x.len() - 1 {
                        res += ", ";
                    }
                }
                res += "]";
                res
            }
            RickrollObject::Char(x) => x.to_string(),
        }
    }
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

impl std::string::ToString for Operator {
    fn to_string(&self) -> String {
        match self {
            Operator::ArrayAccess => "ARRAY_ACCESS",
            Operator::Add => "ADD",
            Operator::Subtract => "SUBTRACT",
            Operator::Multiply => "MULTIPLY",
            Operator::Divide => "DIVIDE",
            Operator::Modulo => "MODULO",
            Operator::UnaryMinus => "UNARY_MINUS",
            Operator::And => "AND",
            Operator::Or => "OR",
            Operator::Not => "NOT",
            Operator::Greater => "GREATER",
            Operator::Less => "LESS",
            Operator::GreaterEquals => "GREATER_EQUALS",
            Operator::LessEquals => "LESS_EQUALS",
            Operator::Equals => "EQUALS",
            Operator::NotEquals => "NOT_EQUALS",
            Operator::LParen => "LPAREN",
            Operator::RParen => "RParen",
        }
        .to_string()
    }
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
