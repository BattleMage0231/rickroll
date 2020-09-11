use lazy_static::lazy_static;
use regex::Regex;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::error::*;
use crate::lexer::*;
use crate::util::*;

#[derive(Debug, EnumIter)]
pub enum Statement {
    Say,
    Let,
    Assign,
    Check,
    WhileEnd,
    IfEnd,
}

impl Statement {
    pub fn matches(&self, raw: &String) -> bool {
        lazy_static! {
            // print
            static ref SAY: Regex = Regex::new("^Never gonna say .+$").unwrap();
            // let + assign to var
            static ref LET: Regex = Regex::new("^Never gonna let \\w+ down$").unwrap();
            static ref ASSIGN: Regex = Regex::new("^Never gonna give \\w+ .+$").unwrap();
            // check, if, and while
            static ref CHECK: Regex = Regex::new("^Inside we both know .+$").unwrap();
            static ref WHILE_END: Regex = Regex::new("^We know the game and we\'re gonna play it$").unwrap();
            static ref IF_END: Regex = Regex::new("^Your heart\'s been aching but you\'re too shy to say it$").unwrap();
        }
        use Statement::*;
        return match self {
            Say => &(*SAY),
            Let => &(*LET),
            Assign => &(*ASSIGN),
            Check => &(*CHECK),
            WhileEnd => &(*WHILE_END),
            IfEnd => &(*IF_END),
        }
        .is_match(raw);
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
    scope: Scope,
    check_stack: Vec<usize>, // remember lines following those that have check statements
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
            },
            scope: Scope::new(),
            check_stack: Vec::new(),
        }
    }

    fn advance(&mut self) {
        self.ptr += 1;
    }

    // wraps a traceback around a possible error
    fn wrap_check<T>(&self, res: Result<T, Error>) -> Result<T, Error> {
        if let Err(error) = res {
            return Err(Error::traceback(error, Some(self.ptr + 1)));
        }
        return res;
    }

    // Vec<(original line number, instruction)>
    // instructions with no original line or expected error should have a line number of 0
    pub fn compile(mut self) -> Result<Vec<(usize, Instruction)>, Error> {
        let mut compiled: Vec<(usize, Instruction)> = Vec::new();
        while self.ptr < self.raw.len() {
            // try to match a statement
            let curln = self.raw[self.ptr].trim();
            if curln != "" {
                let res = Statement::match_statement(&String::from(curln));
                // no statement matched
                if res.is_none() {
                    return Err(Error::new(
                        ErrorType::SyntaxError,
                        "Illegal statement",
                        Some(self.ptr + 1),
                    ));
                }
                // compile statement to bytecode
                use Instruction::*;
                use Statement::*;
                match res.unwrap() {
                    Say => {
                        // ^Never gonna say .+$
                        let expr = String::from(&curln[16..]);
                        let tokens =
                            self.wrap_check(Lexer::new(expr, self.scope.clone()).make_tokens())?;
                        // push Put instruction
                        compiled.push((self.ptr + 1, Put(tokens)));
                    }
                    Statement::Let => {
                        // ^Never gonna let \\w+ down$
                        let varname = String::from(&curln[16..(curln.len() - 5)]);
                        if self.scope.has_var(varname.clone()) {
                            return Err(Error::new(
                                ErrorType::NameError,
                                &(format!(
                                    "Variable {} already exists in the current scope",
                                    varname
                                ))[..],
                                Some(self.ptr + 1),
                            ));
                        }
                        self.scope.add_var(varname.clone());
                        // push Let instruction
                        compiled.push((self.ptr + 1, Instruction::Let(varname)));
                    }
                    Assign => {
                        // ^Never gonna give \\w+ .+$
                        let slice = String::from(&curln[17..]); // \\w .+
                        match slice.find(' ') {
                            Some(index) => {
                                let varname = String::from(String::from(&slice[..index]).trim());
                                if !self.scope.has_var(varname.clone()) {
                                    return Err(Error::new(
                                        ErrorType::NameError,
                                        &(format!("Variable {} doesn't exist", varname))[..],
                                        Some(self.ptr + 1),
                                    ));
                                }
                                let expr = String::from(&slice[(index + 1)..]);
                                let tokens = self.wrap_check(
                                    Lexer::new(expr, self.scope.clone()).make_tokens(),
                                )?;
                                // push Set instruction
                                compiled.push((self.ptr + 1, Set(varname, tokens)));
                            }
                            None => {
                                return Err(Error::new(
                                    ErrorType::SyntaxError,
                                    "Illegal statement",
                                    Some(self.ptr + 1),
                                ))
                            }
                        }
                    }
                    /*
                     * If compiles to
                     * 0 jmpif [TRUE] 2 ; if true, jump to start of code
                     * 1 jmp 5          ; if not true, jump to end of if
                     * 2 pctx           ; push context
                     * 3 put 0
                     * 4 dctx           ; delete context
                     * 5 end
                     * 
                     * While compiles to
                     * 0 jmpif [TRUE] 2 ; if true, jump to start of code
                     * 1 jmp 6          ; if not true, jump to end of loop
                     * 2 pctx           ; push context
                     * 3 put 0
                     * 4 dctx           ; delete context
                     * 5 jmp 0          ; jump back to loop start
                     * 6 end
                     */
                    Check => {
                        // ^Inside we both know .+$
                        let expr = String::from(&curln[20..]);
                        let tokens =
                            self.wrap_check(Lexer::new(expr, self.scope.clone()).make_tokens())?;
                        // skip the next line if tokens evaluates to true
                        compiled.push((self.ptr + 1, Jmpif(tokens, compiled.len() + 2)));
                        // jmp to end of loop/if
                        // since we don't know where it is yet, put a temp and remember its index
                        self.check_stack.push(compiled.len());
                        compiled.push((self.ptr + 1, Tmp()));
                        // push new context
                        compiled.push((self.ptr + 1, Pctx()));
                    }
                    WhileEnd => {
                        // ^We know the game and we\'re gonna play it$
                        if self.check_stack.is_empty() {
                            return Err(Error::new(
                                ErrorType::SyntaxError,
                                "Mismatched while or if end",
                                Some(self.ptr + 1),
                            ));
                        }
                        compiled.push((self.ptr + 1, Dctx())); // pop top context
                        let check_top = self.check_stack.pop().unwrap();
                        compiled.push((self.ptr + 1, Jmp(check_top - 1))); // back to the jmpif instruction
                        // push jmp to instruction at top of check stack
                        compiled[check_top].1 = Jmp(compiled.len());
                    }
                    IfEnd => {
                        // ^Your heart\'s been aching but you\'re too shy to say it$
                        if self.check_stack.is_empty() {
                            return Err(Error::new(
                                ErrorType::SyntaxError,
                                "Mismatched while or if end",
                                Some(self.ptr + 1),
                            ));
                        }
                        compiled.push((self.ptr + 1, Dctx())); // pop top context
                        compiled[self.check_stack.pop().unwrap()].1 = Jmp(compiled.len()); // fill jmp
                    }
                }
            }
            // advance
            self.advance();
        }
        // if check stack isn't empty
        if !self.check_stack.is_empty() {
            return Err(Error::new(
                ErrorType::SyntaxError,
                "Mismatched while or if start",
                Some(compiled[self.check_stack.pop().unwrap()].0),
            ));
        }
        compiled.push((0, Instruction::End()));
        return Ok(compiled);
    }
}
