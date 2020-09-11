use crate::error::*;
use crate::parser::*;
use crate::util::*;

use std::io::{BufRead, Write};

#[derive(Debug)]
pub struct Interpreter {
    ptr: usize,
    bytecode: Vec<(usize, Instruction)>, // Vec<(original line, Instruction)>
    scope: Scope,                        // global scope -> scope1... -> current scope
}

impl Interpreter {
    pub fn new(bytecode: Vec<(usize, Instruction)>) -> Interpreter {
        Interpreter {
            ptr: 0,
            bytecode,
            scope: Scope::new(),
        }
    }

    pub fn advance(&mut self) {
        self.ptr += 1;
    }

    pub fn has_more(&self) -> bool {
        self.ptr < self.bytecode.len()
    }

    // wraps a traceback around a possible error
    fn wrap_check<T>(&self, res: Result<T, Error>) -> Result<T, Error> {
        if let Err(error) = res {
            return Err(Error::traceback(error, Some(self.bytecode[self.ptr].0)));
        }
        return res;
    }

    // evaluates an expression using the parser and error-wraps its result
    fn eval(&self, tokens: Vec<Token>) -> Result<RickrollObject, Error> {
        let parser = Parser::new(tokens, self.scope.clone());
        return self.wrap_check(parser.eval());
    }

    // takes in a mutable buffer and reader rather than
    // writing to stdout and reading from stdin
    pub fn run<W, R>(mut self, buffer: &mut W, reader: &mut R) -> Result<RickrollObject, Error>
    where
        W: Write,
        R: BufRead,
    {
        while self.has_more() {
            let (line, opcode) = &self.bytecode[self.ptr];
            use Instruction::*;
            match opcode {
                Put(tokens) => {
                    writeln!(buffer, "{}", self.eval(tokens.clone())?)
                        .expect("Error when writing to buffer");
                }
                Let(varname) => {
                    self.scope.add_var(varname.clone());
                }
                Set(varname, tokens) => {
                    let val = self.eval(tokens.clone())?;
                    self.scope.set_var(varname.clone(), val);
                }
                End() => {
                    return Ok(RickrollObject::Undefined);
                }
                Jmp(dest) => {
                    self.ptr = *dest;
                    continue; // do not advance()
                }
                Jmpif(tokens, dest) => {
                    // jump??
                    let val = self.eval(tokens.clone())?;
                    let jump = match val {
                        RickrollObject::Bool(x) => x,
                        _ => {
                            return Err(Error::new(
                                ErrorType::IllegalArgumentError,
                                "Unexpected non-boolean argument",
                                Some(*line),
                            ))
                        }
                    };
                    if jump {
                        self.ptr = *dest;
                        continue; // do not advance()
                    }
                }
                Pctx() => {
                    self.scope.push(Context::new());
                }
                Dctx() => {
                    self.scope.pop();
                }
                Tmp() => {
                    panic!("Unexpected Tmp() on line {}", self.ptr);
                }
            }
            self.advance();
        }
        return Ok(RickrollObject::Undefined);
    }
}
