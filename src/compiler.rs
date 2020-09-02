use lazy_static::lazy_static;
use regex::Regex;
use strum::IntoEnumIterator;
use strum_macros::EnumIter; 

use crate::error::*;
use crate::util::*;
use crate::lexer::*;

#[derive(Debug)]
pub enum Instruction {
    Put(Vec<Token>),
    End(),
}

#[derive(Debug, EnumIter)]
pub enum Statement {
    Say,
}

impl Statement {
    pub fn matches(&self, raw: &String) -> bool {
        lazy_static! {
            static ref SAY: Regex = Regex::new("^Never gonna say .+$").unwrap();
        }
        use Statement::*;
        return match self {
            Say => &SAY,
        }.is_match(raw);
    }

    pub fn match_statement(raw: &String) -> Option<Statement> {
        for statement in Statement::iter() {
            if statement.matches(raw) {
                return Some(statement);
            }
        } 
        return None;
    }
}

#[derive(Debug)]
pub struct Compiler {
    ptr: usize,
    raw: Vec<String>,
}

impl Compiler {
    pub fn new(raw_txt: String) -> Compiler {
        Compiler {
            ptr: 0,
            raw: {
                let mut res = Vec::new();
                let mut cur = String::new();
                for chr in raw_txt.chars() {
                    if chr == '\r' || chr == '\n' {
                        res.push(cur);
                        cur = String::new();
                    } else {
                        cur.push(chr);
                    }
                }
                res.push(cur);
                res
            }
        }
    }

    fn advance(&mut self) {
        self.ptr += 1;
    }

    // wraps a traceback around a possible error
    fn wrap_check<T>(&self, res: Result<T, Error>) -> Result<T, Error> {
        if let Err(error) = res {
            return Err(Error::traceback(error, Some(self.ptr)));
        }
        return res;
    }

    pub fn compile(mut self) -> Result<Vec<Instruction>, Error> {
        let mut compiled: Vec<Instruction> = Vec::new();
        while self.ptr < self.raw.len() {
            // try to match a statement
            let curln = self.raw[self.ptr].trim();
            if curln != "" {
                let res = Statement::match_statement(&String::from(curln));
                // no statement matched
                if res.is_none() {
                    return Err(Error::new(ErrorType::SyntaxError, "Illegal statement", Some(self.ptr)));
                }
                // compile statement to bytecode
                use Statement::*;
                use Instruction::*;
                match res.unwrap() {
                    Say => {
                        // ^Never gonna say .+$
                        let expr = String::from(&curln[16..]);
                        let tokens = self.wrap_check(Lexer::new(expr).make_tokens())?;
                        compiled.push(Put(tokens));
                    },
                }   
            }
            // advance
            self.advance();
        }
        compiled.push(Instruction::End());
        return Ok(compiled);
    }
}
