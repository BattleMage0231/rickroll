use std::collections::HashMap;

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

// variable cache for a single block
#[derive(Debug, Clone)]
pub struct Context {
    vars: HashMap<String, RickrollObject>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            vars: HashMap::new(),
        }
    }

    pub fn set_var(&mut self, name: String, value: RickrollObject) {
        self.vars.insert(name, value);
    }

    pub fn get_var(&mut self, name: String) -> Option<RickrollObject> {
        if self.vars.contains_key(&name) {
            return Some(self.vars.get(&name).unwrap().clone());
        } else {
            return None;
        }
    }

    pub fn has_var(&self, name: String) -> bool {
        self.vars.contains_key(&name)
    }
}

#[derive(Debug, Clone)]
pub struct Scope {
    contexts: Vec<Context>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            contexts: vec![Context::new()],
        }
    }

    pub fn len(&self) -> usize {
        self.contexts.len()
    }

    pub fn push(&mut self, context: Context) {
        self.contexts.push(context);
    }

    pub fn push_all(&mut self, mut contexts: Vec<Context>) {
        self.contexts.append(&mut contexts);
    }

    pub fn behead(&mut self) -> Vec<Context> {
        if self.contexts.is_empty() {
            panic!("Empty scope cannot be beheaded");
        }
        let tail = Vec::from(&self.contexts[1..]);
        self.contexts.truncate(1);
        return tail;
    }

    pub fn get_global(&mut self) -> &mut Context {
        self.contexts.first_mut().unwrap()
    }

    pub fn pop(&mut self) -> Context {
        self.contexts
            .pop()
            .expect("Cannot pop context from empty scope")
    }

    // sets a variable in the scope
    // does nothing if variable doesn't exist
    pub fn set_var(&mut self, name: String, value: RickrollObject) {
        for context in self.contexts.iter_mut().rev() {
            if context.has_var(name.clone()) {
                context.set_var(name, value);
                return;
            }
        }
    }

    // gets the value of a variable in the scope
    // returns None if variable doesn't exist
    pub fn get_var(&mut self, name: String) -> Option<RickrollObject> {
        for context in self.contexts.iter_mut().rev() {
            if context.has_var(name.clone()) {
                return Some(context.get_var(name).unwrap());
            }
        }
        return None;
    }

    pub fn has_var(&self, name: String) -> bool {
        for context in self.contexts.iter() {
            if context.has_var(name.clone()) {
                return true;
            }
        }
        return false;
    }

    pub fn add_var(&mut self, name: String) {
        self.contexts
            .last_mut()
            .unwrap()
            .set_var(name, RickrollObject::Undefined);
    }
}
