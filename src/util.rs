use std::collections::HashMap;
use std::ops::{Index, IndexMut};

use strum_macros::EnumIter;

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
    Variable(String),
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

#[derive(Debug, EnumIter, Clone)]
pub enum Statement {
    Say(Vec<Token>),
    Let(String),
    Assign(String, Vec<Token>),
    Check(Vec<Token>),
    WhileEnd(),
    IfEnd(),
}

// intermediate representation of lexed statements
#[derive(Debug)]
pub struct Intermediate {
    statements: Vec<Statement>,
    debug_lines: Vec<usize>,
}

impl Intermediate {
    pub fn new() -> Intermediate {
        Intermediate {
            statements: Vec::new(),
            debug_lines: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.statements.len()
    }

    pub fn push(&mut self, instruction: Statement, orig_line: usize) {
        self.statements.push(instruction);
        self.debug_lines.push(orig_line);
    }

    pub fn debug_line(&self, index: usize) -> usize {
        self.debug_lines[index]
    }
}

impl Index<usize> for Intermediate {
    type Output = Statement;

    fn index<'a>(&'a self, index: usize) -> &'a Statement {
        &self.statements[index]
    }
}

impl IndexMut<usize> for Intermediate {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Statement {
        &mut self.statements[index]
    }
}

// bytecode instruction
#[derive(Debug, Clone)]
pub enum Instruction {
    // print
    Put(Vec<Token>),
    // end program
    End(),
    // let and set variables
    Let(String),
    Set(String, Vec<Token>),
    // jump and conditionally jump
    Jmp(usize),
    Jmpif(Vec<Token>, usize),
    // push and pop context
    Pctx(),
    Dctx(),
    // temporary instruction used to allocate
    // instructions before their existence
    Tmp(),
}

// bytecode definition
#[derive(Debug)]
pub struct Bytecode {
    instructions: Vec<Instruction>,
    debug_lines: Vec<usize>,
    alloc_stack: Vec<usize>,
    /*
     * function: HashMap<String, usize>,
     * file: String,
     * imports: HashMap<String, Bytecode>,
     */
}

impl Bytecode {
    pub fn new() -> Bytecode {
        Bytecode {
            instructions: Vec::new(),
            debug_lines: Vec::new(),
            alloc_stack: Vec::new(),
        }
    }

    pub fn from(vec: Vec<(usize, Instruction)>) -> Bytecode {
        let mut bytecode = Bytecode::new();
        for (line, instruction) in vec {
            bytecode.push(instruction, line);
        }
        return bytecode;
    }

    pub fn to_vec(&self) -> Vec<(usize, Instruction)> {
        let mut res: Vec<(usize, Instruction)> = Vec::new();
        for i in 0..self.len() {
            res.push((self.debug_lines[i], self.instructions[i].clone()));
        }
        return res;
    }

    pub fn debug_line(&self, index: usize) -> usize {
        self.debug_lines[index]
    }

    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    pub fn push(&mut self, instruction: Instruction, orig_line: usize) {
        self.instructions.push(instruction);
        self.debug_lines.push(orig_line);
    }

    pub fn has_tmp(&self) -> bool {
        !self.alloc_stack.is_empty()
    }

    // allocates a Tmp() instruction in the next line
    // returns the allocated index
    pub fn alloc_tmp(&mut self, orig_line: usize) -> usize {
        self.alloc_stack.push(self.len());
        self.instructions.push(Instruction::Tmp());
        self.debug_lines.push(orig_line);
        return *self.alloc_stack.last().unwrap();
    }

    // replaces the last Tmp() with a valid instruction
    // panics if there are no allocated Tmp() instructions
    pub fn free_top(&mut self, new: Instruction) {
        self.instructions[self.alloc_stack.pop().unwrap()] = new;
    }
}

// index into bytecode using [] operator
impl Index<usize> for Bytecode {
    type Output = Instruction;

    fn index<'a>(&'a self, index: usize) -> &'a Instruction {
        &self.instructions[index]
    }
}

impl IndexMut<usize> for Bytecode {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Instruction {
        &mut self.instructions[index]
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
