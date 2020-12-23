use crate::error::*;
use crate::lexer::Token;
use crate::util::*;

// special operator characters
const OP_CHARS: &str = "!&|<>=";

#[derive(Debug)]
pub struct Tokenizer {
    raw: Vec<char>, // raw expression string
    ptr: usize,
    tokens: Vec<Token>,
    line: usize,
}

impl Tokenizer {
    // makes a new tokenizer from the raw string
    pub fn new(string: String, line: usize) -> Tokenizer {
        Tokenizer {
            raw: string.trim().chars().collect(),
            ptr: 0,
            tokens: Vec::new(),
            line,
        }
    }

    // whether tokenizer has more characters to parse
    fn has_more(&self) -> bool {
        self.ptr < self.raw.len()
    }

    // consume self after making tokens
    pub fn make_tokens(mut self) -> Result<Vec<Token>, Error> {
        // empty expression cannot be parsed
        if self.raw.is_empty() {
            return Err(Error::new(
                ErrorType::SyntaxError,
                "Unexpected end of statement",
                None,
            ));
        }
        while self.ptr < self.raw.len() {
            let mut chr = self.raw[self.ptr]; // cur char
                                              // make number
            if chr.is_ascii_digit() {
                let num = self.make_number()?;
                self.tokens.push(num);
                continue;
            }
            // make variable/constant
            if chr.is_ascii_alphabetic() {
                let var = self.make_variable()?;
                self.tokens.push(var);
                continue;
            }
            // make operator
            if OP_CHARS.contains(chr) {
                let operator = self.make_operator()?;
                self.tokens.push(operator);
                continue;
            }
            // character literal
            if chr == '\'' {
                self.ptr += 1;
                // expected more characters in expression
                if !self.has_more() {
                    return Err(Error::new(
                        ErrorType::IllegalCharError,
                        "Trailing character literal",
                        None,
                    ));
                }
                let mut chrlit = self.raw[self.ptr]; // value of char literal
                                                     // empty char literal ('')
                if chrlit == '\'' {
                    return Err(Error::new(
                        ErrorType::IllegalCharError,
                        "Empty literal",
                        None,
                    ));
                }
                // possible escape sequence
                if chrlit == '\\' {
                    self.ptr += 1;
                    if !self.has_more() {
                        return Err(Error::new(
                            ErrorType::IllegalCharError,
                            "Trailing character literal",
                            None,
                        ));
                    }
                    chr = self.raw[self.ptr]; // cur char
                    chrlit = match chr {
                        'n' => '\n', // new line
                        _ => chr,    // otherwise no escape sequence found, regular char
                    };
                }
                self.ptr += 1;
                if !self.has_more() {
                    return Err(Error::new(
                        ErrorType::IllegalCharError,
                        "Trailing character literal",
                        None,
                    ));
                }
                // make sure last character closes off the literal
                chr = self.raw[self.ptr];
                if chr != '\'' {
                    return Err(Error::new(
                        ErrorType::IllegalCharError,
                        "More than one character in literal",
                        None,
                    ));
                }
                // push char value
                self.tokens
                    .push(Token::Value(self.line, RickrollObject::Char(chrlit)));
                self.ptr += 1;
                continue;
            }
            match chr {
                // whitespace can be ignored
                chr if chr.is_whitespace() => (),
                '+' | '-' | '*' | '/' | '%' | ':' => self
                    .tokens
                    .push(Token::Operator(self.line, String::from(chr))),
                '(' => self.tokens.push(Token::Punc(self.line, String::from("("))),
                ')' => self.tokens.push(Token::Punc(self.line, String::from(")"))),
                _ => {
                    return Err(Error::new(
                        ErrorType::IllegalCharError,
                        "Illegal character in expression",
                        None,
                    ));
                }
            }
            self.ptr += 1;
        }
        return Ok(self.tokens);
    }

    // parses a number starting at self.ptr
    fn make_number(&mut self) -> Result<Token, Error> {
        // number can either be integer or float
        let mut inum = 0_i32;
        let mut fnum = 0_f32;
        // whether number is float
        let mut float = false;
        // number of decimal digits
        let mut dig = 0_i32;
        let mut chr = self.raw[self.ptr]; // cur char
        loop {
            // '.' means number is floating point
            if chr == '.' {
                // only one '.' can exist in a number
                if float {
                    return Err(Error::new(
                        ErrorType::IllegalCharError,
                        "Unknown character '.'",
                        None,
                    ));
                }
                // replace inum with fnum
                float = true;
                dig = 1;
                fnum = inum as f32;
            } else {
                // if float, must be in decimal digits
                if float {
                    fnum += (chr.to_digit(10).unwrap() as f32) / ((10.0_f32).powi(dig));
                    dig += 1;
                } else {
                    // if int, must be units digit
                    match inum.checked_mul(10) {
                        None => {
                            return Err(Error::new(
                                ErrorType::IllegalArgumentError,
                                "Integer or float literal too large",
                                None,
                            ))
                        }
                        Some(x) => inum = x,
                    }
                    let curdig = chr.to_digit(10).unwrap() as i32;
                    match inum.checked_add(curdig) {
                        None => {
                            return Err(Error::new(
                                ErrorType::IllegalArgumentError,
                                "Integer or float literal too large",
                                None,
                            ))
                        }
                        Some(x) => inum = x,
                    }
                }
            }
            self.ptr += 1;
            // check if still part of number
            if self.has_more() {
                let cur = self.raw[self.ptr];
                if cur.is_ascii_digit() || cur == '.' {
                    chr = cur;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        // return float/int depending on value
        return Ok(Token::Value(
            self.line,
            if float {
                RickrollObject::Float(fnum)
            } else {
                RickrollObject::Int(inum)
            },
        ));
    }

    // makes a variable/constant starting at ptr
    fn make_variable(&mut self) -> Result<Token, Error> {
        let mut varname = String::new();
        let mut chr = self.raw[self.ptr];
        loop {
            varname.push(chr);
            self.ptr += 1;
            if self.has_more() {
                let cur = self.raw[self.ptr];
                // can only be alphabetic or _
                if cur.is_ascii_alphabetic() || cur == '_' {
                    chr = cur;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        // check if var is a constant
        let res = from_constant(&varname);
        if res.is_some() {
            return Ok(Token::Value(self.line, res.unwrap()));
        } else {
            return Ok(Token::Name(self.line, varname));
        }
    }

    // makes a complex operator starting at ptr
    fn make_operator(&mut self) -> Result<Token, Error> {
        let mut opname = String::new();
        let mut chr = self.raw[self.ptr];
        loop {
            opname.push(chr);
            self.ptr += 1;
            if self.has_more() {
                let cur = self.raw[self.ptr];
                if OP_CHARS.contains(cur) {
                    chr = cur;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        return match &opname[..] {
            // support only one "!" before an argument
            // multiple "!" can be formatted as "! !"
            "&&" | "||" | ">" | "<" | ">=" | "<=" | "==" | "!=" | "!" => {
                Ok(Token::Operator(self.line, opname))
            }
            _ => Err(Error::new(
                ErrorType::RuntimeError,
                &format!("Operator {} not found", opname).to_string(),
                None,
            )),
        };
    }
}
