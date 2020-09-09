use crate::error::*;
use crate::util::*;

/*
 * This expression parser utilizes Dijkstra's Shunting-yard algorithm
 * for parsing and evaluating infix notation expressions.
 * https://en.wikipedia.org/wiki/Shunting-yard_algorithm
 */

// get precedence of operator
pub fn precedence_of(op: &Operator) -> usize {
    use Operator::*;
    // higher precedence is evaluated before lower
    return match op {
        LParen => 0, // no operator can pop left parenthesis
        Or => 1,
        And => 2,
        Greater | Less | GreaterEquals | LessEquals | Equals | NotEquals => 3,
        Add | Subtract => 4,
        Multiply | Divide | Modulo => 5,
        ArrayAccess => 6,
        Not => 7,
        UnaryMinus => 8,
        RParen => 9,
    };
}

// helper functions to make an operator not found/defined error
fn err_unary(op: &str, first: &RickrollObject) -> Error {
    Error::new(
        ErrorType::IllegalArgumentError,
        &format!("{} is not defined for {:?}", op, first)[..],
        None,
    )
}

fn err_binary(op: &str, first: &RickrollObject, second: &RickrollObject) -> Error {
    Error::new(
        ErrorType::IllegalArgumentError,
        &format!("{} is not defined for {:?} and {:?}", op, first, second)[..],
        None,
    )
}

// evaluates an unary expression
pub fn eval_unary(op: &Operator, arg: &RickrollObject) -> Result<RickrollObject, Error> {
    use Operator::*;
    use RickrollObject::*;
    return match op {
        UnaryMinus => match arg {
            Int(x) => Ok(Int(-x)),
            Float(x) => Ok(Float(-x)),
            _ => Err(err_unary("Unary minus", arg)),
        },
        Not => match arg {
            Bool(x) => Ok(Bool(!x)),
            _ => Err(err_unary("Unary not", arg)),
        },
        _ => panic!("Operator is not unary!"),
    };
}

// evaluates a binary expression
pub fn eval_binary(
    op: &Operator,
    first: &RickrollObject,
    second: &RickrollObject,
) -> Result<RickrollObject, Error> {
    use Operator::*;
    use RickrollObject::*;
    return match op {
        ArrayAccess => match (first, second) {
            // array access makes a deep copy of the array
            // this doesn't matter since expressions can't mutate objects in Rickroll
            (Array(arr), Int(x)) => Ok(arr[*x as usize].clone()),
            _ => Err(err_binary("Array access", first, second)),
        },
        Add => match (first, second) {
            (Int(x), Int(y)) => Ok(Int(x + y)),
            (Float(x), Float(y)) => Ok(Float(x + y)),
            (Int(x), Float(y)) => Ok(Float(*x as f32 + y)),
            (Float(x), Int(y)) => Ok(Float(x + *y as f32)),
            _ => Err(err_binary("Add", first, second)),
        },
        Subtract => match (first, second) {
            (Int(x), Int(y)) => Ok(Int(x - y)),
            (Float(x), Float(y)) => Ok(Float(x - y)),
            (Int(x), Float(y)) => Ok(Float(*x as f32 - y)),
            (Float(x), Int(y)) => Ok(Float(x - *y as f32)),
            _ => Err(err_binary("Subtract", first, second)),
        },
        Multiply => match (first, second) {
            (Int(x), Int(y)) => Ok(Int(x * y)),
            (Float(x), Float(y)) => Ok(Float(x * y)),
            (Int(x), Float(y)) => Ok(Float(*x as f32 * y)),
            (Float(x), Int(y)) => Ok(Float(x * *y as f32)),
            _ => Err(err_binary("Multiply", first, second)),
        },
        Divide => match (first, second) {
            (Int(x), Int(y)) => Ok(Int(x / y)),
            (Float(x), Float(y)) => Ok(Float(x / y)),
            (Int(x), Float(y)) => Ok(Float(*x as f32 / y)),
            (Float(x), Int(y)) => Ok(Float(x / *y as f32)),
            _ => Err(err_binary("Divide", first, second)),
        },
        Modulo => match (first, second) {
            (Int(x), Int(y)) => Ok(Int(x % y)),
            (Float(x), Float(y)) => Ok(Float(x % y)),
            (Int(x), Float(y)) => Ok(Float(*x as f32 % y)),
            (Float(x), Int(y)) => Ok(Float(x % *y as f32)),
            _ => Err(err_binary("Modulo", first, second)),
        },
        And => match (first, second) {
            (Bool(x), Bool(y)) => Ok(Bool(*x && *y)),
            _ => Err(err_binary("And", first, second)),
        },
        Or => match (first, second) {
            (Bool(x), Bool(y)) => Ok(Bool(*x || *y)),
            _ => Err(err_binary("Or", first, second)),
        },
        Greater => match (first, second) {
            (Int(x), Int(y)) => Ok(Bool(x > y)),
            (Float(x), Float(y)) => Ok(Bool(x > y)),
            (Int(x), Float(y)) => Ok(Bool(*x as f32 > *y)),
            (Float(x), Int(y)) => Ok(Bool(*x > *y as f32)),
            _ => Err(err_binary("Greater", first, second)),
        },
        Less => match (first, second) {
            (Int(x), Int(y)) => Ok(Bool(x < y)),
            (Float(x), Float(y)) => Ok(Bool(x < y)),
            (Int(x), Float(y)) => Ok(Bool((*x as f32) < *y)),
            (Float(x), Int(y)) => Ok(Bool(*x < *y as f32)),
            _ => Err(err_binary("Less", first, second)),
        },
        GreaterEquals => match (first, second) {
            (Int(x), Int(y)) => Ok(Bool(x >= y)),
            (Float(x), Float(y)) => Ok(Bool(x >= y)),
            (Int(x), Float(y)) => Ok(Bool(*x as f32 >= *y)),
            (Float(x), Int(y)) => Ok(Bool(*x >= *y as f32)),
            _ => Err(err_binary("Greater equals", first, second)),
        },
        LessEquals => match (first, second) {
            (Int(x), Int(y)) => Ok(Bool(x <= y)),
            (Float(x), Float(y)) => Ok(Bool(x <= y)),
            (Int(x), Float(y)) => Ok(Bool(*x as f32 <= *y)),
            (Float(x), Int(y)) => Ok(Bool(*x <= *y as f32)),
            _ => Err(err_binary("Less equals", first, second)),
        },
        Equals => match (first, second) {
            (Int(x), Int(y)) => Ok(Bool(x == y)),
            (Float(x), Float(y)) => Ok(Bool(x == y)),
            (Int(x), Float(y)) => Ok(Bool(*x as f32 == *y)),
            (Float(x), Int(y)) => Ok(Bool(*x == *y as f32)),
            _ => Err(err_binary("Equals", first, second)),
        },
        NotEquals => match (first, second) {
            (Int(x), Int(y)) => Ok(Bool(x != y)),
            (Float(x), Float(y)) => Ok(Bool(x != y)),
            (Int(x), Float(y)) => Ok(Bool(*x as f32 != *y)),
            (Float(x), Int(y)) => Ok(Bool(*x != *y as f32)),
            _ => Err(err_binary("Not equals", first, second)),
        },
        _ => panic!(format!("Operator {:?} is not binary!", op)),
    };
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    ptr: usize,
    value_stack: Vec<RickrollObject>, // stack of values
    op_stack: Vec<Operator>,          // stack of operators
    scope: Scope,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, scope: Scope) -> Parser {
        Parser {
            tokens,
            ptr: 0,
            value_stack: Vec::new(),
            op_stack: Vec::new(),
            scope,
        }
    }

    fn advance(&mut self) {
        self.ptr += 1;
    }

    fn has_more(&self) -> bool {
        self.ptr < self.tokens.len()
    }

    // evaluates all possible operations given that the last operator is op
    // all operators are left-associative
    fn pop(&mut self, op: &Operator) -> Result<(), Error> {
        while !self.op_stack.is_empty()
            && precedence_of(self.op_stack.last().unwrap()) >= precedence_of(op)
        {
            let top = self.op_stack.last().unwrap();
            if top.is_unary() {
                if self.value_stack.is_empty() {
                    return Err(Error::new(
                        ErrorType::IllegalArgumentError,
                        "Not enough arguments",
                        None,
                    ));
                }
                let arg = self.value_stack.pop().unwrap();
                self.value_stack.push(eval_unary(top, &arg)?);
            } else {
                if self.value_stack.len() < 2 {
                    return Err(Error::new(
                        ErrorType::IllegalArgumentError,
                        "Not enough arguments",
                        None,
                    ));
                }
                let first = self.value_stack.pop().unwrap();
                let second = self.value_stack.pop().unwrap();
                self.value_stack.push(eval_binary(top, &second, &first)?);
            }
            self.op_stack.pop();
        }
        Ok(())
    }

    // evaluates all operations until there are no operators or a left parenthesis is reached
    fn pop_all(&mut self) -> Result<(), Error> {
        while !self.op_stack.is_empty() {
            let top = self.op_stack.pop().unwrap();
            match top {
                Operator::LParen => break,
                _ => {
                    if top.is_unary() {
                        if self.value_stack.is_empty() {
                            return Err(Error::new(
                                ErrorType::IllegalArgumentError,
                                "Not enough arguments",
                                None,
                            ));
                        }
                        let arg = self.value_stack.pop().unwrap();
                        self.value_stack.push(eval_unary(&top, &arg)?);
                    } else {
                        if self.value_stack.len() < 2 {
                            return Err(Error::new(
                                ErrorType::IllegalArgumentError,
                                "Not enough arguments",
                                None,
                            ));
                        }
                        let first = self.value_stack.pop().unwrap();
                        let second = self.value_stack.pop().unwrap();
                        self.value_stack.push(eval_binary(&top, &second, &first)?);
                    }
                }
            }
        }
        Ok(())
    }

    // evaluates the expression
    pub fn eval(mut self) -> Result<RickrollObject, Error> {
        while self.has_more() {
            let token = (&self.tokens)[self.ptr].clone(); // reference to a Token object
            match token {
                Token::Value(obj) => self.value_stack.push(obj.clone()), // push to stack if value
                Token::Operator(op) => {
                    if let Operator::RParen = op {
                        self.pop_all()?;
                    } else if let Operator::LParen = op {
                        self.op_stack.push(op.clone());
                    } else {
                        // unary operator => wait for binary operator to pop it
                        if !op.is_unary() {
                            self.pop(&op)?; // pop all possible if operator
                        }
                        self.op_stack.push(op.clone());
                    }
                }
                Token::Variable(name) => {
                    let value = self.scope.get_var(name.clone());
                    if value.is_none() {
                        return Err(Error::new(
                            ErrorType::NameError,
                            &(format!("No such variable {}", name))[..],
                            None,
                        ));
                    } else {
                        self.value_stack.push(value.unwrap().clone());
                    }
                }
            }
            self.advance();
        }
        // try to pop all operations at the end
        self.pop_all()?;
        // if something went wrong while evaluating the expression...
        if self.value_stack.len() != 1 || !self.op_stack.is_empty() {
            return Err(Error::new(
                ErrorType::SyntaxError,
                "Illegal expression syntax",
                None,
            ));
        }
        // return a clone of the only value left in value_stack
        return Ok(self.value_stack.last().unwrap().clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::Operator::*;
    use crate::util::RickrollObject::*;
    use Token::*;

    // helper function to return string form of evaluated
    fn get(tokens: Vec<Token>) -> String {
        match Parser::new(tokens, Scope::new()).eval() {
            Ok(val) => format!("{}", val),
            Err(err) => format!("{:?}", err),
        }
    }

    // helper function to test whether the first expression evaluates to the second
    fn assert_eqv(first: Vec<Token>, second: &str) {
        assert_eq!(&get(first)[..], second);
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
