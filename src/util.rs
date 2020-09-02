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

impl std::fmt::Display for RickrollObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use RickrollObject::*;
        let formatted = match self {
            Int(x) => x.to_string(),
            Float(x) => x.to_string(),
            Bool(x) => String::from(if *x { "TRUE" } else { "FALSE" }),
            Array(x) => {
                let mut res = String::from("[");
                for ind in 0..x.len() {
                    res += &x[ind].to_string()[..];
                    if ind != x.len() - 1 {
                        res += ", "
                    }
                }
                res += "]";
                res
            }
            Char(x) => x.to_string(),
            Undefined => String::from("UNDEFINED"),
        };
        write!(f, "{}", formatted)
    }
}

// operators
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
