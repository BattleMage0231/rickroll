use lazy_static::lazy_static;
use regex::Regex;

use crate::error::*;
use crate::tokenizer::*;
use crate::util::*;

// lex source code into IR
// does not do any complex syntax checking
#[derive(Debug)]
pub struct Lexer {
    ptr: usize,
    raw: Vec<String>,
    lexed: Intermediate,
    scope: Scope,
    check_counter: usize,
}

impl Lexer {
    pub fn new(raw_txt: String) -> Lexer {
        Lexer {
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
            lexed: Intermediate::new(),
            scope: Scope::new(),
            check_counter: 0,
        }
    }

    fn advance(&mut self) {
        self.ptr += 1;
    }

    fn has_more(&self) -> bool {
        self.ptr < self.raw.len()
    }

    // wraps a traceback around a possible error
    fn wrap_check<T>(&self, res: Result<T, Error>) -> Result<T, Error> {
        if let Err(error) = res {
            return Err(Error::traceback(error, Some(self.ptr + 1)));
        }
        return res;
    }

    pub fn parse(mut self) -> Result<Intermediate, Error> {
        println!("A");
        // regexes for matching statements
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
        println!("B");
        // iterate over raw
        while self.has_more() {
            println!("C");
            // try to match a statement
            let curln = self.raw[self.ptr].trim();
            if curln == "" {
                self.advance();
                continue;
            } else if SAY.is_match(curln) {
                // ^Never gonna say .+$
                let expr = String::from(&curln[16..]);
                let tokens =
                    self.wrap_check(Tokenizer::new(expr, self.scope.clone()).make_tokens())?;
                self.lexed.push(Statement::Say(tokens), self.ptr + 1);
            } else if LET.is_match(curln) {
                // ^Never gonna let \\w+ down$
                let varname = String::from(&curln[16..(curln.len() - 5)]);
                // variable names already exists
                if self.scope.has_var(varname.clone()) {
                    return Err(Error::new(
                        ErrorType::NameError,
                        &(format!("Variable {} already exists in the current scope", varname))[..],
                        Some(self.ptr + 1),
                    ));
                }
                self.scope.add_var(varname.clone());
                self.lexed.push(Statement::Let(varname), self.ptr + 1);
            } else if ASSIGN.is_match(curln) {
                // ^Never gonna give \\w+ .+$
                let slice = String::from(&curln[17..]); // \\w .+
                match slice.find(' ') {
                    Some(index) => {
                        let varname = String::from(String::from(&slice[..index]).trim());
                        // variable doesn't exist
                        if !self.scope.has_var(varname.clone()) {
                            return Err(Error::new(
                                ErrorType::NameError,
                                &(format!("Variable {} doesn't exist", varname))[..],
                                Some(self.ptr + 1),
                            ));
                        }
                        let expr = String::from(&slice[(index + 1)..]);
                        let tokens = self
                            .wrap_check(Tokenizer::new(expr, self.scope.clone()).make_tokens())?;
                        self.lexed
                            .push(Statement::Assign(varname, tokens), self.ptr + 1);
                    }
                    None => {
                        return Err(Error::new(
                            ErrorType::SyntaxError,
                            "Illegal statement",
                            Some(self.ptr + 1),
                        ));
                    }
                }
            } else if CHECK.is_match(curln) {
                // ^Inside we both know .+$
                let expr = String::from(&curln[20..]);
                let tokens =
                    self.wrap_check(Tokenizer::new(expr, self.scope.clone()).make_tokens())?;
                self.lexed.push(Statement::Check(tokens), self.ptr + 1);
                // increase check counter
                self.check_counter += 1;
            } else if WHILE_END.is_match(curln) {
                // ^We know the game and we\'re gonna play it$
                self.lexed.push(Statement::WhileEnd(), self.ptr + 1);
                // mismatched check counter
                if self.check_counter == 0 {
                    return Err(Error::new(
                        ErrorType::SyntaxError,
                        "Mismatched while or if end",
                        Some(self.ptr + 1),
                    ));
                }
                self.check_counter -= 1;
            } else if IF_END.is_match(curln) {
                // ^Your heart\'s been aching but you\'re too shy to say it$
                self.lexed.push(Statement::IfEnd(), self.ptr + 1);
                // mismatched check counter
                if self.check_counter == 0 {
                    return Err(Error::new(
                        ErrorType::SyntaxError,
                        "Mismatched while or if end",
                        Some(self.ptr + 1),
                    ));
                }
                self.check_counter -= 1;
            } else {
                // unknown statement
                return Err(Error::new(
                    ErrorType::SyntaxError,
                    "Illegal statement",
                    Some(self.ptr + 1),
                ));
            }
            self.advance();
        }
        if self.check_counter != 0 {
            return Err(Error::new(
                ErrorType::SyntaxError,
                "Mismatched while or if end",
                Some(self.ptr + 1),
            ));
        }
        return Ok(self.lexed);
    }
}

#[cfg(test)]
mod tests {}
