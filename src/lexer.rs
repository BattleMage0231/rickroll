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
        // iterate over raw
        while self.has_more() {
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
                        "Mismatched while end",
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
                        "Mismatched if end",
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
                "Mismatched while or if start",
                Some(self.ptr),
            ));
        }
        return Ok(self.lexed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get(s: &str) -> String {
        String::from(format!("{:?}", Lexer::new(String::from(s)).parse()))
    }
    fn assert_eqv(raw: &str, res: &str) {
        assert_eq!(&get(raw)[..], res);
    }

    #[test]
    fn simple() {
        assert_eqv(
            "\
            Never gonna say 1 + 2
            Never gonna say 3 > 4
            ",
            "Ok(Intermediate { statements: [Say([Value(Int(1)), Operator(Add), Value(Int(2))]), Say([Value(Int(3)), Operator(Greater), Value(Int(4))])], debug_lines: [1, 2] })",
        );
        assert_eqv(
            "\
            Never gonna let a down
            Never gonna give a 3
            Never gonna let b down
            Never gonna say a
            Never gonna say b
            Never gonna say a + 3
            ",
            "Ok(Intermediate { statements: [Let(\"a\"), Assign(\"a\", [Value(Int(3))]), Let(\"b\"), Say([Variable(\"a\")]), Say([Variable(\"b\")]), Say([Variable(\"a\"), Operator(Add), Value(Int(3))])], debug_lines: [1, 2, 3, 4, 5, 6] })",
        );
    }

    // while loops and if statements
    #[test]
    fn check() {
        assert_eqv(
            "\
            Never gonna let n down
            Never gonna give n 10
            Never gonna let first down
            Never gonna let second down
            Never gonna give first 0
            Never gonna give second 1
            Never gonna say second
            Inside we both know n != 0
                Never gonna let sum down
                Never gonna give sum first + second
                Never gonna say sum
                Never gonna give first second
                Never gonna give second sum
                Never gonna give n n - 1
            We know the game and we're gonna play it
            ",
            "Ok(Intermediate { statements: [Let(\"n\"), Assign(\"n\", [Value(Int(10))]), Let(\"first\"), Let(\"second\"), Assign(\"first\", [Value(Int(0))]), Assign(\"second\", [Value(Int(1))]), Say([Variable(\"second\")]), Check([Variable(\"n\"), Operator(NotEquals), Value(Int(0))]), Let(\"sum\"), Assign(\"sum\", [Variable(\"first\"), Operator(Add), Variable(\"second\")]), Say([Variable(\"sum\")]), Assign(\"first\", [Variable(\"second\")]), Assign(\"second\", [Variable(\"sum\")]), Assign(\"n\", [Variable(\"n\"), Operator(Subtract), Value(Int(1))]), WhileEnd], debug_lines: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15] })",
        );
        assert_eqv(
            "\
            Never gonna let a down
            Never gonna give a 5
            Inside we both know a == 5
                Never gonna say TRUE
            Your heart's been aching but you're too shy to say it
            ",
            "Ok(Intermediate { statements: [Let(\"a\"), Assign(\"a\", [Value(Int(5))]), Check([Variable(\"a\"), Operator(Equals), Value(Int(5))]), Say([Value(Bool(true))]), IfEnd], debug_lines: [1, 2, 3, 4, 5] })",
        );
    }

    // should output Result::Error
    #[test]
    fn error() {
        assert_eqv(
            "\
            asdasdasdasd
            Never gonna say a
            ",
            "Err(Error { err: SyntaxError, desc: \"Illegal statement\", line: Some(1), child: None })",
        );
        assert_eqv(
            "\
            Inside we both know TRUE
                Inside we both know TRUE
                Your heart's been aching but you're too shy to say it
            ",
            "Err(Error { err: SyntaxError, desc: \"Mismatched while or if start\", line: Some(4), child: None })",
        );
    }
}
