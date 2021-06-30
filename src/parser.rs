use crate::error::*;
use crate::expr::*;
use crate::lexer::Token;
use crate::util::*;
use crate::stdlib::BUILTIN_FUNCTIONS;

use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
pub enum ASTNode {
    Say(usize, Expr),
    Let(usize, String),
    Assign(usize, String, Expr),
    If(usize, Expr, Vec<ASTNode>),
    While(usize, Expr, Vec<ASTNode>),
    Function(usize, String, Vec<String>, Vec<ASTNode>),
    Return(usize, Expr),
    Run(usize, String, Vec<String>),
    RunAssign(usize, String, String, Vec<String>),
}

impl ASTNode {
    pub fn get_line(&self) -> usize {
        use ASTNode::*;
        match self {
            Say(ln, _) => *ln,
            Let(ln, _) => *ln,
            Assign(ln, _, _) => *ln,
            If(ln, _, _) => *ln,
            While(ln, _, _) => *ln,
            Function(ln, _, _, _) => *ln,
            Return(ln, _) => *ln,
            Run(ln, _, _) => *ln,
            RunAssign(ln, _, _, _) => *ln,
        }
    }
}

#[derive(Debug)]
pub struct Parser {
    tokens: VecDeque<Token>,
    output: HashMap<String, ASTNode>,
    func_cache: HashSet<String>,
    scope: Scope,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: VecDeque::from(tokens),
            output: HashMap::new(),
            func_cache: HashSet::new(),
            scope: Scope::new(),
        }
    }

    fn get_name(&mut self) -> String {
        let name = self.tokens.pop_front().unwrap();
        match name {
            Token::Name(_, name) => return name,
            _ => panic!("Parser::get_name called with invalid name"),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, Error> {
        let mut expr_tokens: Vec<Token> = Vec::new();
        while !self.tokens.is_empty() {
            if let Token::Statement(_, _) = self.tokens.front().unwrap() {
                break;
            }
            expr_tokens.push(self.tokens.pop_front().unwrap());
        }
        let parser = ExprParser::new(expr_tokens, self.scope.clone());
        return parser.parse();
    }

    fn parse_loop(&mut self, line: usize) -> Result<ASTNode, Error> {
        self.scope.push(Context::new());
        let condition = self.parse_expr()?;
        let mut body: Vec<ASTNode> = Vec::new();
        while !self.tokens.is_empty() {
            let top = self.tokens.front().unwrap();
            if let Token::Statement(ln, kw) = top {
                match &kw[..] {
                    "WHILE_END" => {
                        self.scope.pop();
                        self.tokens.pop_front();
                        return Ok(ASTNode::While(line, condition, body));
                    }
                    "IF_END" => {
                        self.scope.pop();
                        self.tokens.pop_front();
                        return Ok(ASTNode::If(line, condition, body));
                    }
                    "VERSE" => {
                        return Err(Error::new(
                            ErrorType::SyntaxError,
                            "Unbalanced statements",
                            Some(*ln),
                        ));
                    }
                    _ => {
                        body.push(self.parse_statement()?);
                    }
                }
            } else {
                panic!("Parser::parse_loop called with invalid statement");
            }
        }
        return Err(Error::new(
            ErrorType::SyntaxError,
            "Unbalanced statements",
            None,
        ));
    }

    fn parse_statement(&mut self) -> Result<ASTNode, Error> {
        let token = self.tokens.pop_front().unwrap();
        if let Token::Statement(line, kw) = token {
            match &kw[..] {
                "SAY" => {
                    return Ok(ASTNode::Say(line, self.parse_expr()?));
                }
                "LET" => {
                    let name = self.get_name();
                    if self.scope.has_var(name.clone()) {
                        return Err(Error::new(
                            ErrorType::NameError,
                            &format!("Variable name {} already exists", name)[..],
                            Some(line),
                        ));
                    }
                    self.scope.add_var(name.clone());
                    return Ok(ASTNode::Let(line, name));
                }
                "ASSIGN" => {
                    let name = self.get_name();
                    if !self.scope.has_var(name.clone()) {
                        return Err(Error::new(
                            ErrorType::NameError,
                            &format!("Variable name {} doesn't exist", name)[..],
                            Some(line),
                        ));
                    }
                    return Ok(ASTNode::Assign(line, name, self.parse_expr()?));
                }
                "CHECK" => {
                    return self.parse_loop(line);
                }
                "WHILE_END" | "IF_END" => {
                    return Err(Error::new(
                        ErrorType::SyntaxError,
                        "Unbalanced statements",
                        Some(line),
                    ));
                }
                "RUN" => {
                    let name = self.get_name();
                    if !self.func_cache.contains(&name) && !BUILTIN_FUNCTIONS.contains_key(&name) {
                        return Err(Error::new(
                            ErrorType::NameError,
                            &format!("Function name {} doesn't exist", name)[..],
                            Some(line),
                        ));
                    }
                    let mut args: Vec<String> = Vec::new();
                    while !self.tokens.is_empty() {
                        match self.tokens.front().unwrap() {
                            Token::Name(_, name) => {
                                args.push(name.clone());
                                self.tokens.pop_front();
                            }
                            _ => break,
                        }
                    }
                    return Ok(ASTNode::Run(line, name, args));
                }
                "RUN_ASSIGN" => {
                    let var_name = self.get_name();
                    let name = self.get_name();
                    if !self.func_cache.contains(&name) && !BUILTIN_FUNCTIONS.contains_key(&name) {
                        return Err(Error::new(
                            ErrorType::NameError,
                            &format!("Function name {} doesn't exist", name)[..],
                            Some(line),
                        ));
                    }
                    let mut args: Vec<String> = Vec::new();
                    while !self.tokens.is_empty() {
                        match self.tokens.front().unwrap() {
                            Token::Name(_, name) => {
                                args.push(name.clone());
                                self.tokens.pop_front();
                            }
                            _ => break,
                        }
                    }
                    return Ok(ASTNode::RunAssign(line, var_name, name, args));
                }
                "RETURN" => {
                    return Ok(ASTNode::Return(line, self.parse_expr()?));
                }
                _ => panic!("Parser::parse_statement called with invalid keyword {}", kw),
            }
        } else {
            return Err(Error::new(
                ErrorType::SyntaxError,
                "Illegal statement",
                Some(token.get_line()),
            ));
        }
    }

    fn parse_function(&mut self) -> Result<ASTNode, Error> {
        let token = self.tokens.pop_front().unwrap();
        let mut body: Vec<ASTNode> = Vec::new();
        if let Token::Statement(ln, kw) = &token {
            if kw.clone() == String::from("VERSE") {
                // add scope
                self.scope.push(Context::new());
                // extract name
                let name_token = self.tokens.pop_front().unwrap();
                let name = match name_token {
                    Token::Name(_, name) => name,
                    _ => panic!("Parser::parse_function called with malformed verse token"),
                };
                // insert into func_cache
                if self.func_cache.contains(&name) {
                    return Err(Error::new(
                        ErrorType::NameError,
                        &format!("Function named {} already exists", name)[..],
                        Some(*ln),
                    ));
                }
                self.func_cache.insert(name.clone());
                // extract arguments
                let mut args: Vec<String> = Vec::new();
                while !self.tokens.is_empty() {
                    let front = self.tokens.front().unwrap();
                    match front {
                        Token::Name(_, name) => {
                            args.push(name.clone());
                            self.scope.add_var(name.clone());
                            self.tokens.pop_front();
                        }
                        _ => break,
                    }
                }
                // extract body
                while !self.tokens.is_empty() {
                    let front = self.tokens.front().unwrap();
                    if let Token::Statement(_, kw) = front {
                        if String::from(kw) != String::from("VERSE") {
                            body.push(self.parse_statement()?);
                        } else {
                            break;
                        }
                    } else {
                        return Err(Error::new(
                            ErrorType::SyntaxError,
                            "Illegal statement",
                            Some(token.get_line()),
                        ));
                    }
                }
                self.scope.pop();
                return Ok(ASTNode::Function(*ln, name, args, body));
            } else {
                return Err(Error::new(
                    ErrorType::SyntaxError,
                    "Invalid start of function",
                    Some(*ln),
                ));
            }
        } else {
            return Err(Error::new(
                ErrorType::SyntaxError,
                "Invalid start of function",
                Some(token.get_line()),
            ));
        }
    }

    pub fn parse(mut self) -> Result<HashMap<String, ASTNode>, Error> {
        while !self.tokens.is_empty() {
            // parse function
            let fnc = self.parse_function()?;
            if let ASTNode::Function(_, name, _, _) = &fnc {
                self.output.insert(name.clone(), fnc);
            } else {
                return Err(Error::new(
                    ErrorType::SyntaxError,
                    "Statement not in function",
                    Some(fnc.get_line()),
                ));
            }
        }
        return Ok(self.output);
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::Operator::*;
    use crate::util::RickrollObject::*;
    use Token::*;

    // helper function to return string form of evaluated
    fn get(tokens: Vec<Token>, scope: Scope) -> String {
        match Parser::new(tokens, scope).eval() {
            Ok(val) => format!("{}", val),
            Err(err) => format!("{:?}", err),
        }
    }

    // helper function to test whether the first expression evaluates to the second
    fn assert_eqv(first: Vec<Token>, second: &str) {
        assert_eq!(&get(first, Scope::new())[..], second);
    }

    fn assert_eqv_scope(first: Vec<Token>, second: &str, scope: Scope) {
        assert_eq!(&get(first, scope)[..], second);
    }

    // simple test cases
    #[test]
    fn simple() {
        assert_eqv(vec![Value(Int(1)), Operator(Add), Value(Int(2))], "3");
        assert_eqv(
            vec![
                Value(Int(1)),
                Operator(Multiply),
                Value(Int(2)),
                Operator(Divide),
                Value(Int(3)),
                Operator(Add),
                Value(Int(4)),
            ],
            "4",
        );
        assert_eqv(
            vec![
                Value(Int(1)),
                Operator(Add),
                Value(Int(2)),
                Operator(GreaterEquals),
                Value(Int(3)),
                Operator(Or),
                Value(Bool(false)),
            ],
            "TRUE",
        );
        // variables test
        assert_eqv_scope(
            vec![
                Variable(String::from("a")),
                Operator(Add),
                Value(Int(3)),
                Operator(Multiply),
                Variable(String::from("xxx")),
            ],
            "15",
            {
                let mut s = Scope::new();
                s.add_var(String::from("a"));
                s.add_var(String::from("xxx"));
                s.set_var(String::from("a"), Int(3));
                s.set_var(String::from("xxx"), Float(4.0));
                s
            },
        );
    }

    // operator precedence and parenthesis
    #[test]
    fn precedence() {
        assert_eqv(
            vec![
                Value(Int(3)),
                Operator(Add),
                Value(Int(2)),
                Operator(Multiply),
                Value(Int(5)),
            ],
            "13",
        );
        assert_eqv(
            vec![
                Operator(LParen),
                Operator(LParen),
                Value(Int(7)),
                Operator(RParen),
                Operator(RParen),
                Operator(Multiply),
                Operator(LParen),
                Value(Int(4)),
                Operator(Add),
                Value(Int(2)),
                Operator(RParen),
            ],
            "42",
        );
        assert_eqv(
            vec![
                Operator(LParen),
                Value(Int(3)),
                Operator(Add),
                Value(Int(2)),
                Operator(RParen),
                Operator(Multiply),
                Value(Int(5)),
                Operator(Greater),
                Value(Int(4)),
                Operator(Or),
                Value(Bool(true)),
                Operator(And),
                Value(Bool(false)),
            ],
            "TRUE",
        );
    }

    // unary operators
    #[test]
    fn unary() {
        assert_eqv(
            vec![
                Operator(Not),
                Operator(Not),
                Value(Bool(true)),
                Operator(And),
                Operator(Not),
                Value(Bool(false)),
            ],
            "TRUE",
        );
        assert_eqv(
            vec![
                Value(Int(3)),
                Operator(Subtract),
                Operator(UnaryMinus),
                Operator(LParen),
                Operator(UnaryMinus),
                Value(Int(4)),
                Operator(RParen),
            ],
            "-1",
        );
    }

    // things that should be errors
    #[test]
    fn error() {
        assert_eqv(
            vec![
                Value(Int(1)),
                Operator(Add),
                Operator(Not),
                Operator(LParen),
                Value(Bool(true)),
                Operator(Or),
                Value(Bool(false)),
                Operator(RParen)
            ],
            "Error { err: IllegalArgumentError, desc: \"Add is not defined for Int(1) and Bool(false)\", line: None, child: None }",
        );
        assert_eqv(
            vec![
                Operator(Add),
                Value(Int(1)),
                Operator(Multiply),
                Value(Int(2)),
            ],
            "Error { err: IllegalArgumentError, desc: \"Not enough arguments\", line: None, child: None }",
        );
    }
}
*/
