use lazy_static::lazy_static;
use regex::Regex;

use crate::error::*;
use crate::tokenizer::Tokenizer;
use crate::util::*;

#[derive(Debug)]
pub enum Token {
    Punc(usize, String),
    Name(usize, String),
    Value(usize, RickrollObject),
    Operator(usize, String),
    Statement(usize, String),
}

#[derive(Debug)]
pub struct Lexer {
    ptr: usize,
    raw: Vec<String>,
    lexed: Vec<Token>,
}

impl Lexer {
    pub fn new(raw_text: String) -> Lexer {
        Lexer {
            ptr: 0,
            raw: {
                let mut res = Vec::new();
                let mut cur = String::new();
                for chr in raw_text.chars() {
                    if chr == '\n' {
                        res.push(cur);
                        cur = String::new();
                    } else {
                        cur.push(chr);
                    }
                }
                res.push(cur);
                res
            },
            lexed: Vec::new(),
        }
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

    // helper function splitting a string of the form "A, BCD, EEE" into ["A", "BCD", "EEE"]
    fn split_vars(&self, raw: String, empty: String) -> Result<Vec<String>, Error> {
        let mut args: Vec<String> = Vec::new();
        let mut cur: String = String::new();
        for chr in String::from(raw.trim()).chars() {
            // valid character
            if chr.is_ascii_alphabetic() || chr == '_' {
                cur.push(chr);
            } else if chr == ',' {
                // variable break
                if cur.is_empty() {
                    return Err(Error::new(
                        ErrorType::NameError,
                        "Blank variable name",
                        Some(self.ptr + 1),
                    ));
                }
                args.push(cur.to_owned());
                cur.clear();
            } else if !chr.is_ascii_whitespace() {
                // illegal character
                return Err(Error::new(
                    ErrorType::IllegalArgumentError,
                    &(format!("Illegal character \"{}\" in variable", chr))[..],
                    Some(self.ptr + 1),
                ));
            }
        }
        // empty returns no arguments
        if cur == empty {
            return Ok(Vec::new());
        }
        if !cur.is_empty() {
            args.push(cur.to_owned());
        }
        return Ok(args);
    }

    pub fn parse(mut self) -> Result<Vec<Token>, Error> {
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
            // blocks (functions)
            static ref CHORUS: Regex = Regex::new("^\\[Chorus\\]$").unwrap();
            static ref INTRO: Regex = Regex::new("^\\[Intro\\]$").unwrap();
            static ref VERSE: Regex = Regex::new("^\\[Verse \\w+\\]$").unwrap();
            // function statements
            static ref RUN: Regex = Regex::new("^Never gonna run \\w+ and desert .+$").unwrap();
            static ref RUN_ASSIGN: Regex = Regex::new("^\\(Ooh give you \\w+\\) Never gonna run \\w+ and desert .+$").unwrap();
            static ref RETURN: Regex = Regex::new("^\\(Ooh\\) Never gonna give, never gonna give \\(give you .+\\)$").unwrap();
            // function parameters
            static ref ARGS: Regex = Regex::new("\\(Ooh give you .+\\)").unwrap();
        }
        // iterate over raw
        while self.has_more() {
            // try to match a statement
            let curln = self.raw[self.ptr].trim();
            if curln == "" {
                self.ptr += 1;
                continue;
            } else if SAY.is_match(curln) {
                // ^Never gonna say .+$
                let expr = String::from(&curln[16..]);
                let tokens = self.wrap_check(Tokenizer::new(expr, self.ptr + 1).make_tokens())?;
                self.lexed
                    .push(Token::Statement(self.ptr + 1, String::from("SAY")));
                for token in tokens {
                    self.lexed.push(token);
                }
            } else if LET.is_match(curln) {
                // ^Never gonna let \\w+ down$
                let varname = String::from(&curln[16..(curln.len() - 5)]);
                self.lexed
                    .push(Token::Statement(self.ptr + 1, String::from("LET")));
                self.lexed.push(Token::Name(self.ptr + 1, varname));
            } else if ASSIGN.is_match(curln) {
                // ^Never gonna give \\w+ .+$
                let slice = String::from(&curln[17..]); // \\w .+
                match slice.find(' ') {
                    Some(index) => {
                        let varname = String::from(String::from(&slice[..index]).trim());
                        let expr = String::from(&slice[(index + 1)..]);
                        let tokens =
                            self.wrap_check(Tokenizer::new(expr, self.ptr + 1).make_tokens())?;
                        self.lexed
                            .push(Token::Statement(self.ptr + 1, String::from("ASSIGN")));
                        self.lexed.push(Token::Name(self.ptr + 1, varname));
                        for token in tokens {
                            self.lexed.push(token);
                        }
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
                let tokens = self.wrap_check(Tokenizer::new(expr, self.ptr + 1).make_tokens())?;
                self.lexed
                    .push(Token::Statement(self.ptr + 1, String::from("CHECK")));
                for token in tokens {
                    self.lexed.push(token);
                }
            } else if WHILE_END.is_match(curln) {
                // ^We know the game and we\'re gonna play it$
                self.lexed
                    .push(Token::Statement(self.ptr + 1, String::from("WHILE_END")));
            } else if IF_END.is_match(curln) {
                // ^Your heart\'s been aching but you\'re too shy to say it$
                self.lexed
                    .push(Token::Statement(self.ptr + 1, String::from("IF_END")));
            } else if CHORUS.is_match(curln) {
                self.lexed
                    .push(Token::Statement(self.ptr + 1, String::from("VERSE")));
                self.lexed
                    .push(Token::Name(self.ptr + 1, String::from("[CHORUS]")));
            } else if INTRO.is_match(curln) {
                self.lexed
                    .push(Token::Statement(self.ptr + 1, String::from("VERSE")));
                self.lexed
                    .push(Token::Name(self.ptr + 1, String::from("[INTRO]")));
            } else if VERSE.is_match(curln) {
                // ^\\[Verse \\w+\\]$
                let func_name = String::from(&curln[7..(curln.len() - 1)]);
                self.ptr += 1;
                let curln = self.raw[self.ptr].trim();
                if !self.has_more() || !ARGS.is_match(curln) {
                    return Err(Error::new(
                        ErrorType::SyntaxError,
                        &(format!("No argument specification for function {}", func_name))[..],
                        Some(self.ptr + 1),
                    ));
                }
                // "\\(Ooh give you .+\\)"
                let func_args = self.split_vars(
                    String::from(&curln[14..(curln.len() - 1)]),
                    String::from("up"),
                )?;
                self.lexed
                    .push(Token::Statement(self.ptr, String::from("VERSE")));
                self.lexed.push(Token::Name(self.ptr, func_name));
                for arg in func_args {
                    self.lexed.push(Token::Name(self.ptr + 1, arg));
                }
            } else if RUN.is_match(curln) {
                // ^Never gonna run \\w+ and desert .+$
                let substring = String::from(&curln[16..]); // \\w+ and desert .+$
                let ind = substring.find(' ').unwrap();
                // get function info
                let func_name = String::from(&substring[..ind]);
                let func_args =
                    self.split_vars(String::from(&substring[(ind + 12)..]), String::from("you"))?;
                // push function call
                self.lexed
                    .push(Token::Statement(self.ptr + 1, String::from("RUN")));
                self.lexed.push(Token::Name(self.ptr + 1, func_name));
                for arg in func_args {
                    self.lexed.push(Token::Name(self.ptr + 1, arg));
                }
            } else if RUN_ASSIGN.is_match(curln) {
                // ^\\(Ooh give you \\w+\\) Never gonna run \\w+ and desert .+$
                let substring = String::from(&curln[14..]); // \\w+\\) Never gonna run \\w+ and desert .+$
                let ind = substring.find(')').unwrap();
                // get variable info
                let varname = String::from(&substring[..ind]);
                let substring = String::from(&substring[(ind + 18)..]); // \\w+ and desert .+$
                let ind = substring.find(' ').unwrap();
                // get function info
                let func_name = String::from(&substring[..ind]);
                let func_args =
                    self.split_vars(String::from(&substring[(ind + 12)..]), String::from("you"))?;
                // push function call
                self.lexed
                    .push(Token::Statement(self.ptr + 1, String::from("RUN_ASSIGN")));
                self.lexed.push(Token::Name(self.ptr + 1, varname));
                self.lexed.push(Token::Name(self.ptr + 1, func_name));
                for arg in func_args {
                    self.lexed.push(Token::Name(self.ptr + 1, arg));
                }
            } else if RETURN.is_match(curln) {
                // ^\\(Ooh\\) Never gonna give, never gonna give \\(give you .+\\)$
                let expr = String::from(&curln[51..(curln.len() - 1)]);
                let tokens = self.wrap_check(Tokenizer::new(expr, self.ptr + 1).make_tokens())?;
                self.lexed
                    .push(Token::Statement(self.ptr + 1, String::from("RETURN")));
                for token in tokens {
                    self.lexed.push(token);
                }
            } else {
                // unknown statement
                return Err(Error::new(
                    ErrorType::SyntaxError,
                    "Illegal statement",
                    Some(self.ptr + 1),
                ));
            }
            self.ptr += 1;
        }
        return Ok(self.lexed);
    }
}

/*
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
*/

/*
#[derive(Debug, EnumIter, Clone)]
pub enum Statement {
    Say(usize, Vec<String>),
    Let(usize, String),
    Assign(usize, String, Vec<String>),
    Check(usize, Vec<String>),
    WhileEnd(usize),
    IfEnd(usize),
    // functions
    Chorus(usize),
    Intro(usize),
    Verse(usize, String, Vec<String>),
    // function utilities
    Return(usize, Vec<String>),
    Run(usize, String, Vec<String>),
    RunAssign(usize, String, String, Vec<String>),
}

// lex source code into IR
// does not do any complex syntax checking
#[derive(Debug)]
pub struct Lexer {
    ptr: usize,
    raw: Vec<String>,
    lexed: Vec<Statement>,
    scope: Scope,
    check_counter: usize,
    function_cache: HashMap<String, usize>, // <Name, Arg count>
}
*/
