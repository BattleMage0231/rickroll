use crate::error::*;
use crate::util::*;

const SPECIAL: &str = "!&|<>=";

#[derive(Debug)]
pub enum Token {
    Value(RickrollObject),
    Operator(Operator),
}

#[derive(Debug)]
pub struct Lexer {
    raw: Vec<char>,
    cur_chr: Option<char>,
    ptr: usize,
}

impl Lexer {
    // makes a new lexer from the raw string
    pub fn new(string: String) -> Lexer {
        let raw: Vec<char> = String::from(string.trim()).chars().collect();
        Lexer {
            raw: raw.clone(),
            cur_chr: if raw.len() > 0 { Some(raw[0]) } else { None },
            ptr: 0,
        }
    }

    // advances the ptr
    fn advance(&mut self) {
        self.ptr += 1;
        self.cur_chr = if self.ptr < self.raw.len() {
            Some(self.raw[self.ptr])
        } else {
            None
        };
    }

    // consume self after making tokens
    pub fn make_tokens(mut self) -> Result<Vec<Token>, Error> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut paren_balance = 0;
        if self.raw.is_empty() {
            return Err(Error::new(
                ErrorType::SyntaxError,
                "Unexpected end of statement",
                None,
            ));
        }
        while self.cur_chr.is_some() {
            let mut chr = self.cur_chr.unwrap();
            if chr.is_ascii_digit() {
                let num = self.make_number()?;
                tokens.push(num);
                continue;
            }
            if chr.is_ascii_alphabetic() {
                let var = self.make_variable()?;
                tokens.push(var);
                continue;
            }
            if SPECIAL.contains(chr) {
                let operator = self.make_operator()?;
                tokens.push(operator);
                continue;
            }
            // character literal
            if chr == '\'' {
                self.advance();
                if self.cur_chr.is_none() {
                    return Err(Error::new(
                        ErrorType::IllegalCharError,
                        "Trailing character literal",
                        None,
                    ));
                }
                let mut chrlit = self.cur_chr.unwrap();
                if chrlit == '\'' {
                    return Err(Error::new(
                        ErrorType::IllegalCharError,
                        "Empty literal",
                        None,
                    ));
                }
                if chrlit == '\\' {
                    self.advance();
                    if self.cur_chr.is_none() {
                        return Err(Error::new(
                            ErrorType::IllegalCharError,
                            "Trailing character literal",
                            None,
                        ));
                    }
                    chr = self.cur_chr.unwrap();
                    chrlit = match chr {
                        'n' => '\n',
                        _ => chr,
                    };
                }
                self.advance();
                if self.cur_chr.is_none() {
                    return Err(Error::new(
                        ErrorType::IllegalCharError,
                        "Trailing character literal",
                        None,
                    ));
                }
                chr = self.cur_chr.unwrap();
                if chr != '\'' {
                    return Err(Error::new(
                        ErrorType::IllegalCharError,
                        "Too many characters in literal",
                        None,
                    ));
                }
                tokens.push(Token::Value(RickrollObject::Char(chrlit)));
                self.advance();
                continue;
            }
            match chr {
                chr if chr.is_whitespace() => (),
                '+' => tokens.push(Token::Operator(Operator::Add)),
                '-' => {
                    let mut token = Token::Operator(Operator::UnaryMinus);
                    if !tokens.is_empty() {
                        match tokens.last().unwrap() {
                            Token::Value(obj) => match obj {
                                RickrollObject::Int(_) | RickrollObject::Float(_) => {
                                    token = Token::Operator(Operator::Subtract);
                                }
                                _ => (),
                            },
                            _ => (),
                        }
                    }
                    tokens.push(token);
                }
                '*' => tokens.push(Token::Operator(Operator::Multiply)),
                '/' => tokens.push(Token::Operator(Operator::Divide)),
                '%' => tokens.push(Token::Operator(Operator::Modulo)),
                '(' => {
                    tokens.push(Token::Operator(Operator::LParen));
                    paren_balance += 1;
                }
                ')' => {
                    tokens.push(Token::Operator(Operator::RParen));
                    paren_balance -= 1;
                    if paren_balance < 0 {
                        return Err(Error::new(
                            ErrorType::SyntaxError,
                            "Unbalanced parenthesis",
                            None,
                        ));
                    }
                }
                ':' => tokens.push(Token::Operator(Operator::ArrayAccess)),
                _ => {
                    return Err(Error::new(
                        ErrorType::IllegalCharError,
                        "Illegal character in expression",
                        None,
                    ));
                }
            }
            self.advance();
        }
        if paren_balance != 0 {
            return Err(Error::new(
                ErrorType::SyntaxError,
                "Unbalanced parenthesis",
                None,
            ));
        }
        return Ok(tokens);
    }

    fn make_number(&mut self) -> Result<Token, Error> {
        let mut inum = 0_i32;
        let mut fnum = 0_f32;
        let mut float = false;
        let mut dig = 0_i32;
        let mut chr = self.cur_chr.unwrap();
        loop {
            if chr == '.' {
                if float {
                    return Err(Error::new(
                        ErrorType::IllegalCharError,
                        "Unknown character '.'",
                        None,
                    ));
                }
                float = true;
                dig = 1;
                fnum = inum as f32;
            } else {
                if float {
                    fnum += (chr.to_digit(10).unwrap() as f32) / ((10.0_f32).powi(dig));
                    dig += 1;
                } else {
                    inum *= 10;
                    inum += chr.to_digit(10).unwrap() as i32;
                }
            }
            self.advance();
            match self.cur_chr {
                Some(x) if (x.is_ascii_digit() || x == '.') => chr = x,
                _ => break,
            }
        }
        return Ok(Token::Value(if float {
            RickrollObject::Float(fnum)
        } else {
            RickrollObject::Int(inum)
        }));
    }

    fn make_variable(&mut self) -> Result<Token, Error> {
        let mut varname = String::new();
        let mut chr = self.cur_chr.unwrap();
        loop {
            varname.push(chr);
            self.advance();
            match self.cur_chr {
                Some(x) if (x.is_ascii_alphabetic() || x == '_') => chr = x,
                _ => break,
            }
        }
        let res = from_constant(&varname);
        if res.is_some() {
            return Ok(Token::Value(res.unwrap()));
        }
        return Err(Error::new(
            ErrorType::NameError,
            &format!("Variable {} not found", varname).to_string(),
            None,
        ));
    }

    fn make_operator(&mut self) -> Result<Token, Error> {
        let mut opname = String::new();
        let mut chr = self.cur_chr.unwrap();
        loop {
            opname.push(chr);
            self.advance();
            match self.cur_chr {
                Some(x) if SPECIAL.contains(x) => chr = x,
                _ => break,
            }
        }
        return match &opname[..] {
            "&&" => Ok(Token::Operator(Operator::And)),
            "||" => Ok(Token::Operator(Operator::Or)),
            ">" => Ok(Token::Operator(Operator::Greater)),
            "<" => Ok(Token::Operator(Operator::Less)),
            ">=" => Ok(Token::Operator(Operator::GreaterEquals)),
            "<=" => Ok(Token::Operator(Operator::LessEquals)),
            "==" => Ok(Token::Operator(Operator::Equals)),
            "!" => Ok(Token::Operator(Operator::Not)),
            "!=" => Ok(Token::Operator(Operator::NotEquals)),
            _ => Err(Error::new(
                ErrorType::RuntimeError,
                &format!("Operator {} not found", opname).to_string(),
                None,
            )),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // helper function to return string form of parsed
    fn get(expr: &str) -> String {
        match Lexer::new(String::from(expr)).make_tokens() {
            Ok(tokens) => format!("{:?}", tokens),
            Err(err) => format!("{:?}", err),
        }
    }

    // helper function to test whether the first expression parses to the second
    fn assert_eqv(first: &str, second: &str) {
        assert_eq!(&get(first)[..], second);
    }

    // simple expressions without brackets
    #[test]
    fn simple() {
        assert_eqv("1 + 2", "[Value(Int(1)), Operator(Add), Value(Int(2))]");
        assert_eqv("1  + 2- 3 *45 ", "[Value(Int(1)), Operator(Add), Value(Int(2)), Operator(Subtract), Value(Int(3)), Operator(Multiply), Value(Int(45))]");
        assert_eqv("72 * 4.0 + -1.0", "[Value(Int(72)), Operator(Multiply), Value(Float(4.0)), Operator(Add), Operator(UnaryMinus), Value(Float(1.0))]");
    }

    // valid parenthesis expressions
    #[test]
    fn paren() {
        assert_eqv("(3 * 4)", "[Operator(LParen), Value(Int(3)), Operator(Multiply), Value(Int(4)), Operator(RParen)]");
        assert_eqv("2 % (1 + 2 * 3 ) + 5", "[Value(Int(2)), Operator(Modulo), Operator(LParen), Value(Int(1)), Operator(Add), Value(Int(2)), Operator(Multiply), Value(Int(3)), Operator(RParen), Operator(Add), Value(Int(5))]");
        assert_eqv("4 + (( 4+ 5 ) * (3) * 1)", "[Value(Int(4)), Operator(Add), Operator(LParen), Operator(LParen), Value(Int(4)), Operator(Add), Value(Int(5)), Operator(RParen), Operator(Multiply), Operator(LParen), Value(Int(3)), Operator(RParen), Operator(Multiply), Value(Int(1)), Operator(RParen)]");
    }

    // valid character expressions
    #[test]
    fn char() {
        assert_eqv("'x'", "[Value(Char('x'))]");
        assert_eqv("'\\n'", "[Value(Char('\\n'))]");
    }

    // valid boolean expressiobs
    #[test]
    fn bool() {
        assert_eqv(
            " 3 > 4",
            "[Value(Int(3)), Operator(Greater), Value(Int(4))]",
        );
        assert_eqv("4 <= 5 ||5 > 6", "[Value(Int(4)), Operator(LessEquals), Value(Int(5)), Operator(Or), Value(Int(5)), Operator(Greater), Value(Int(6))]");
        assert_eqv("!(1 == 1) && 2 != 2 || 3 + 1 > 4", "[Operator(Not), Operator(LParen), Value(Int(1)), Operator(Equals), Value(Int(1)), Operator(RParen), Operator(And), Value(Int(2)), Operator(NotEquals), Value(Int(2)), Operator(Or), Value(Int(3)), Operator(Add), Value(Int(1)), Operator(Greater), Value(Int(4))]");
    }

    // valid language constants
    #[test]
    fn constants() {
        assert_eqv(
            "TRUE || FALSE",
            "[Value(Bool(true)), Operator(Or), Value(Bool(false))]",
        );
        assert_eqv(
            " ARRAY:3",
            "[Value(Array([])), Operator(ArrayAccess), Value(Int(3))]",
        );
        assert_eqv("UNDEFINED", "[Value(Undefined)]");
    }

    // should output error
    #[test]
    fn error() {
        assert_eqv(
            "    ",
            "Error { err: SyntaxError, desc: \"Unexpected end of statement\", line: None }",
        );
        assert_eqv(
            "'a",
            "Error { err: IllegalCharError, desc: \"Trailing character literal\", line: None }",
        );
        assert_eqv(
            "3 + (()()",
            "Error { err: SyntaxError, desc: \"Unbalanced parenthesis\", line: None }",
        );
        assert_eqv(
            "a += b ** cD",
            "Error { err: NameError, desc: \"Variable a not found\", line: None }",
        );
        assert_eqv(
            "'asdasdasdasdasd'",
            "Error { err: IllegalCharError, desc: \"Too many characters in literal\", line: None }",
        );
    }
}
