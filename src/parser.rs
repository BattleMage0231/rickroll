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
        Not => 6,
        ArrayAccess => 7,
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
        }
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
    op_stack: Vec<Operator>, // stack of operators
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            ptr: 0,
            value_stack: Vec::new(),
            op_stack: Vec::new(),
        }
    }

    fn advance(&mut self) {
        self.ptr += 1;
    }

    fn has_more(&self) -> bool {
        self.ptr < self.tokens.len()
    }

    // evaluates all possible operations given that the last operator is op
    fn pop(&self, op: &Operator) {
        /* to be implemented */
    }

    // evaluates all operations until there are no operators or a left parenthesis is reached
    fn pop_all(&self) {
        /* to be implemented */
    }

    // evaluates the expression
    pub fn eval(mut self) -> Result<RickrollObject, Error> {
        while self.has_more() {
            let token = &self.tokens[self.ptr]; // reference to a Token object
            match token {
                Token::Value(obj) => self.value_stack.push(obj.clone()), // push to stack if value
                Token::Operator(op) => self.pop(&op), // pop all possible if operator
            }
            self.advance();
        }
        self.pop_all(); // pop all at the end
        // if something went wrong while evaluating the expression...
        if self.value_stack.len() != 1 || !self.op_stack.is_empty() {
            return Err(Error::new(ErrorType::SyntaxError, "Illegal expression syntax", None));
        }
        // return a clone of the only value left in value_stack
        return Ok(self.value_stack.last().unwrap().clone());
    }
}
