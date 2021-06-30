use crate::error::*;
use crate::lexer::Token;
use crate::util::*;

// special operator characters
const OP_CHARS: &str = "!&|<>=~";

#[derive(Debug, Clone)]
pub enum Expr {
    Value(RickrollObject),
    Name(String),
    Operation(Operator, Vec<Expr>),
}

#[derive(Debug)]
pub struct ExprLexer {
    raw: Vec<char>, // raw expression string
    ptr: usize,
    tokens: Vec<Token>,
    line: usize,
}

impl ExprLexer {
    // makes a new tokenizer from the raw string
    pub fn new(string: String, line: usize) -> ExprLexer {
        ExprLexer {
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
        let mut float = false;
        let mut raw = String::new();
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
                float = true;
            }
            raw.push(chr);
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
        if float {
            let res = raw.parse::<f32>();
            match res {
                Ok(val) => return Ok(Token::Value(self.line, RickrollObject::Float(val))),
                Err(_) => return Err(Error::new(
                    ErrorType::IllegalArgumentError,
                    "Improper floating point literal",
                    None,
                )),
            }
        } else {
            let res = raw.parse::<i32>();
            match res {
                Ok(val) => return Ok(Token::Value(self.line, RickrollObject::Int(val))),
                Err(_) => return Err(Error::new(
                    ErrorType::IllegalArgumentError,
                    "Improper integer literal",
                    None,
                )),
            }
        }
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
            "&&" | "||" | ">" | "<" | ">=" | "<=" | "==" | "!=" | "!" | "~" => {
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

// get operator from string
pub fn get_operator(str: &String) -> Result<Operator, Error> {
    use Operator::*;
    return match &str[..] {
        "||" => Ok(Or),
        "&&" => Ok(And),
        ">" => Ok(Greater),
        "<" => Ok(Less),
        ">=" => Ok(GreaterEquals),
        "<=" => Ok(LessEquals),
        "==" => Ok(Equals),
        "!=" => Ok(NotEquals),
        "+" => Ok(Add),
        "-" => Ok(Subtract),
        "*" => Ok(Multiply),
        "/" => Ok(Divide),
        "%" => Ok(Modulo),
        ":" => Ok(ArrayAccess),
        "!" => Ok(Not),
        "~" => Ok(UnaryMinus),
        _ => Err(Error::new(
            ErrorType::SyntaxError,
            &format!("Operator {} not found", str)[..],
            None,
        )),
    };
}

// get precedence of operator
pub fn precedence_of(op: &Operator) -> usize {
    use Operator::*;
    // higher precedence is evaluated before lower
    return match op {
        Or => 1,
        And => 2,
        Greater | Less | GreaterEquals | LessEquals | Equals | NotEquals => 3,
        Add | Subtract => 4,
        Multiply | Divide | Modulo => 5,
        ArrayAccess => 6,
        Not => 7,
        UnaryMinus => 8,
    };
}

/*
 * This expression parser utilizes Dijkstra's Shunting-yard algorithm
 * for parsing infix expressions and converting them to ASTs.
 * https://en.wikipedia.org/wiki/Shunting-yard_algorithm
 */

#[derive(Debug)]
pub struct ExprParser {
    tokens: Vec<Token>,
    ptr: usize,
    scope: Scope,
    output_stack: Vec<Token>, // output stack
    op_stack: Vec<Token>,     // stack of operators and parenthesis
}

impl ExprParser {
    pub fn new(tokens: Vec<Token>, scope: Scope) -> ExprParser {
        ExprParser {
            tokens,
            ptr: 0,
            output_stack: Vec::new(),
            op_stack: Vec::new(),
            scope,
        }
    }

    fn has_more(&self) -> bool {
        self.ptr < self.tokens.len()
    }

    // resolves as many operations as possible given the last operator
    // all operators are left-associative
    fn pop(&mut self, op: &Operator) -> Result<(), Error> {
        while !self.op_stack.is_empty() {
            let top = self.op_stack.last().unwrap();
            if let Token::Punc(_, _) = top {
                break; // will never be ")", only "("
            }
            match top {
                Token::Operator(_, top_chr) => {
                    // break if precedence is lower
                    if precedence_of(&get_operator(top_chr)?) < precedence_of(op) {
                        break;
                    }
                    self.output_stack.push(self.op_stack.pop().unwrap());
                }
                _ => panic!("ExprParser::pop called with non punctuation or operator"),
            }
        }
        return Ok(());
    }

    // resolves all operations until there are no more operators
    // or a left parenthesis is reached
    fn pop_all(&mut self) -> Result<(), Error> {
        while !self.op_stack.is_empty() {
            let top = self.op_stack.pop().unwrap(); // pop top
            if let Token::Punc(_, _) = top {
                break; // will never be ")", only "("
            }
            match &top {
                Token::Operator(_, op) => {
                    get_operator(&op)?; // ensure operator is valid
                    self.output_stack.push(top);
                }
                _ => panic!("ExprParser::pop_all called with non punctuation or operator"),
            }
        }
        return Ok(());
    }

    // parses the tokens into RPN stored in output_stacl
    fn to_rpn(&mut self) -> Result<(), Error> {
        while self.has_more() {
            let token = ((&self.tokens)[self.ptr]).clone(); // reference to a Token object
            match &token {
                Token::Value(_, _) => self.output_stack.push(token),
                Token::Operator(_, op) => {
                    let valid = get_operator(&op)?;
                    if !valid.is_unary() {
                        self.pop(&valid)?;
                    }
                    self.op_stack.push(token);
                }
                Token::Punc(_, punc) => {
                    // "(" or ")"
                    match &punc[..] {
                        "(" => {
                            self.op_stack.push(token);
                        }
                        ")" => {
                            self.pop_all()?;
                        }
                        _ => panic!("Unexpected symbol found in ExprParser::to_rpn"),
                    }
                }
                Token::Name(_, name) => {
                    if self.scope.has_var(name.clone()) {
                        self.output_stack.push(token);
                    } else {
                        return Err(Error::new(
                            ErrorType::NameError,
                            &(format!("No such variable {}", name))[..],
                            None,
                        ));
                    }
                }
                _ => panic!("Unexpected enum variant found in ExprParser::to_rpn"),
            }
            self.ptr += 1;
        }
        // try to pop all operations at the end
        self.pop_all()?;
        return Ok(());
    }

    pub fn parse(mut self) -> Result<Expr, Error> {
        self.to_rpn()?;
        let mut stack: Vec<Expr> = Vec::new();
        if self.output_stack.len() == 1 {
            let tok = self.output_stack.pop().unwrap();
            if let Token::Name(_, name) = tok {
                return Ok(Expr::Name(name));
            } else if let Token::Value(_, val) = tok {
                return Ok(Expr::Value(val));
            } else {
                return Err(Error::new(
                    ErrorType::SyntaxError,
                    "Illegal expression",
                    None,
                ));
            }
        }
        while !self.output_stack.is_empty() {
            let token = self.output_stack.pop().unwrap();
            match token {
                Token::Value(_, obj) => {
                    let last = stack.last_mut().unwrap();
                    match last {
                        Expr::Operation(_, args) => {
                            args.push(Expr::Value(obj.clone()));
                        }
                        _ => panic!("ExprParser::parse: Found non-operation in return stack"),
                    }
                }
                Token::Operator(_, op) => {
                    stack.push(Expr::Operation(get_operator(&op)?, Vec::new()));
                }
                Token::Name(_, name) => {
                    let last = stack.last_mut().unwrap();
                    match last {
                        Expr::Operation(_, args) => {
                            args.push(Expr::Name(name.clone()));
                        }
                        _ => panic!("ExprParser::parse: Found non-operation in return stack"),
                    }
                }
                _ => panic!("Unexpected enum variant found in ExprParser::parse"),
            }
            while stack.len() > 1 {
                let top_expr = stack.last().unwrap();
                if let Expr::Operation(op, args) = top_expr {
                    let enough_args;
                    if op.is_unary() {
                        enough_args = args.len() == 1;
                    } else {
                        enough_args = args.len() == 2;
                    }
                    if enough_args {
                        let top_expr = stack.pop().unwrap();
                        if let Expr::Operation(_, args) = stack.last_mut().unwrap() {
                            args.push(top_expr);
                        } else {
                            panic!("ExprParser::parse: Found non-operation in return stack");
                        }
                    }
                } else {
                    panic!("ExprParser::parse: Found non-operation in return stack");
                }
            }
        }
        if stack.len() != 1 {
            return Err(Error::new(
                ErrorType::SyntaxError,
                "Illegal expression",
                None,
            ));
        } else {
            return Ok(stack.pop().unwrap());
        }
    }
}
