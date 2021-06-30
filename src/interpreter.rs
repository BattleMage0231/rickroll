use crate::error::*;
use crate::expr::*;
use crate::parser::*;
use crate::util::*;
use crate::stdlib::BUILTIN_FUNCTIONS;

use std::collections::HashMap;
use std::io::{BufRead, Write};

pub const MAX_RECURSION_DEPTH: usize = 10000;
pub const MAX_UNWIND_LIMIT: usize = 8;

#[derive(Debug)]
pub struct Interpreter {
    functions: HashMap<String, ASTNode>,
}

fn eval_err(op: &Operator) -> Error {
    Error::new(
        ErrorType::IllegalArgumentError,
        &format!("Illegal types for operation {:?}", op)[..],
        None,
    )
}

impl Interpreter {
    pub fn new(functions: HashMap<String, ASTNode>) -> Interpreter {
        Interpreter {
            functions,
        }
    }

    // wraps a traceback around a possible error
    fn wrap_check<T>(&self, res: Result<T, Error>, ln: usize) -> Result<T, Error> {
        if let Err(error) = res {
            return Err(Error::traceback(error, Some(ln)));
        }
        return res;
    }

    fn eval(&self, expr: &Expr, scope: &Scope) -> Result<RickrollObject, Error> {
        match expr {
            Expr::Value(obj) => Ok(obj.clone()),
            Expr::Name(name) => {
                if scope.has_var(name.clone()) {
                    return Ok(scope.get_var(name.clone()).unwrap());
                } else {
                    return Err(Error::new(
                        ErrorType::NameError,
                        &format!("Variable {} doesn't exist", name)[..],
                        None,
                    ));
                }
            }
            Expr::Operation(op, args) => {
                use Operator::*;
                use RickrollObject::*;
                if op.is_unary() && args.len() == 1 {
                    let operand = self.eval(&args[0], scope)?;
                    return match op {
                        UnaryMinus => match operand {
                            Int(x) => Ok(Int(-x)),
                            Float(x) => Ok(Float(-x)),
                            _ => Err(eval_err(op)),
                        },
                        Not => match operand {
                            Bool(x) => Ok(Bool(!x)),
                            _ => Err(eval_err(op)),
                        },
                        _ => panic!("Unary operator is not unary!"),
                    };
                } else if !op.is_unary() && args.len() == 2 {
                    // expressions operands start from the top
                    let first = self.eval(&args[1], scope)?;
                    let second = self.eval(&args[0], scope)?;
                    let ans = match op {
                        ArrayAccess => match (first, second) {
                            (Array(arr), Int(x)) => Ok(arr[x as usize].clone()),
                            _ => Err(eval_err(op)),
                        },
                        Add => match (first, second) {
                            (Int(x), Int(y)) => Ok(Int(x.wrapping_add(y))),
                            (Float(x), Float(y)) => Ok(Float(x + y)),
                            _ => Err(eval_err(op)),
                        },
                        Subtract => match (first, second) {
                            (Int(x), Int(y)) => Ok(Int(x.wrapping_sub(y))),
                            (Float(x), Float(y)) => Ok(Float(x - y)),
                            _ => Err(eval_err(op)),
                        },
                        Multiply => match (first, second) {
                            (Int(x), Int(y)) => Ok(Int(x.wrapping_mul(y))),
                            (Float(x), Float(y)) => Ok(Float(x * y)),
                            _ => Err(eval_err(op)),
                        },
                        Divide => match (first, second) {
                            (Int(x), Int(y)) => Ok(Int(x.wrapping_div(y))),
                            (Float(x), Float(y)) => Ok(Float(x / y)),
                            _ => Err(eval_err(op)),
                        },
                        Modulo => match (first, second) {
                            (Int(x), Int(y)) => Ok(Int(x.wrapping_rem(y))),
                            (Float(x), Float(y)) => Ok(Float(x % y)),
                            _ => Err(eval_err(op)),
                        },
                        And => match (first, second) {
                            (Bool(x), Bool(y)) => Ok(Bool(x && y)),
                            _ => Err(eval_err(op)),
                        },
                        Or => match (first, second) {
                            (Bool(x), Bool(y)) => Ok(Bool(x || y)),
                            _ => Err(eval_err(op)),
                        },
                        Greater => match (first, second) {
                            (Int(x), Int(y)) => Ok(Bool(x > y)),
                            (Float(x), Float(y)) => Ok(Bool(x > y)),
                            _ => Err(eval_err(op)),
                        },
                        Less => match (first, second) {
                            (Int(x), Int(y)) => Ok(Bool(x < y)),
                            (Float(x), Float(y)) => Ok(Bool(x < y)),
                            _ => Err(eval_err(op)),
                        },
                        GreaterEquals => match (first, second) {
                            (Int(x), Int(y)) => Ok(Bool(x >= y)),
                            (Float(x), Float(y)) => Ok(Bool(x >= y)),
                            _ => Err(eval_err(op)),
                        },
                        LessEquals => match (first, second) {
                            (Int(x), Int(y)) => Ok(Bool(x <= y)),
                            (Float(x), Float(y)) => Ok(Bool(x <= y)),
                            _ => Err(eval_err(op)),
                        },
                        Equals => match (first, second) {
                            (Int(x), Int(y)) => Ok(Bool(x == y)),
                            (Float(x), Float(y)) => Ok(Bool(x == y)),
                            (Bool(x), Bool(y)) => Ok(Bool(x == y)),
                            (Char(x), Char(y)) => Ok(Bool(x == y)),
                            _ => Ok(Bool(false)), // default false
                        },
                        NotEquals => match (first, second) {
                            (Int(x), Int(y)) => Ok(Bool(x != y)),
                            (Float(x), Float(y)) => Ok(Bool(x != y)),
                            (Bool(x), Bool(y)) => Ok(Bool(x != y)),
                            (Char(x), Char(y)) => Ok(Bool(x != y)),
                            _ => Ok(Bool(true)), // default true
                        },
                        _ => panic!("Binary operator is not binary!"),
                    };
                    return ans;
                } else {
                    return Err(Error::new(ErrorType::NameError, "Illegal operation", None));
                }
            }
        }
    }

    // execute a statement
    // returns Ok(obj) if the function should return
    pub fn execute(
        &mut self,
        statement: &ASTNode,
        scope: &mut Scope,
        buffer: &mut dyn Write,
        reader: &mut dyn BufRead,
    ) -> Result<Option<RickrollObject>, Error> {
        match statement {
            ASTNode::Say(ln, expr) => {
                let res = self.wrap_check(self.eval(expr, scope), *ln)?;
                writeln!(buffer, "{}", res).expect("Error when writing to buffer");
            }
            ASTNode::Let(_, name) => {
                scope.add_var(name.clone());
            }
            ASTNode::Assign(ln, name, expr) => {
                let res = self.wrap_check(self.eval(expr, scope), *ln)?;
                scope.set_var(name.clone(), res);
            }
            ASTNode::While(ln, cond, body) => loop {
                let res = self.wrap_check(self.eval(cond, scope), *ln)?;
                match res {
                    RickrollObject::Bool(x) => {
                        if !x {
                            break;
                        }
                    }
                    _ => {
                        return Err(Error::new(
                            ErrorType::RuntimeError,
                            "While condition is not boolean",
                            Some(*ln),
                        ))
                    }
                }
                scope.push(Context::new());
                for node in body {
                    let res = self.execute(node, scope, buffer, reader)?;
                    match res {
                        Some(obj) => return Ok(Some(obj)),
                        None => (),
                    }
                }
                scope.pop();
            },
            ASTNode::If(ln, cond, body) => {
                let res = self.wrap_check(self.eval(cond, scope), *ln)?;
                match res {
                    RickrollObject::Bool(x) => {
                        if x {
                            scope.push(Context::new());
                            for node in body {
                                let res = self.execute(node, scope, buffer, reader)?;
                                match res {
                                    Some(obj) => return Ok(Some(obj)),
                                    None => (),
                                }
                            }
                            scope.pop();
                        }
                    }
                    _ => {
                        return Err(Error::new(
                            ErrorType::RuntimeError,
                            "While condition is not boolean",
                            Some(*ln),
                        ))
                    }
                }
            }
            ASTNode::Run(ln, func, args) => {
                let mut passed: Vec<RickrollObject> = Vec::new();
                for arg in args {
                    passed.push(scope.get_var(arg.clone()).unwrap());
                }
                let tail = scope.behead();
                scope.push(Context::new());
                let res = self.run_function(func.clone(), passed, scope, buffer, reader);
                self.wrap_check(res, *ln)?;
                scope.behead();
                scope.push_all(tail);
            }
            ASTNode::RunAssign(ln, var, func, args) => {
                let mut passed: Vec<RickrollObject> = Vec::new();
                for arg in args {
                    passed.push(scope.get_var(arg.clone()).unwrap());
                }
                let tail = scope.behead();
                scope.push(Context::new());
                let res = self.run_function(func.clone(), passed, scope, buffer, reader);
                let res = self.wrap_check(res, *ln)?;
                scope.behead();
                scope.push_all(tail);
                scope.set_var(var.clone(), res);
            },
            ASTNode::Return(ln, expr) => {
                let res = self.wrap_check(self.eval(expr, scope), *ln)?;
                return Ok(Some(res));
            },
            _ => {
                panic!("Interpreter::execute called with Function");
            },
        }
        return Ok(None);
    }

    // executes a function
    pub fn run_function(
        &mut self,
        func: String,
        passed: Vec<RickrollObject>,
        scope: &mut Scope,
        buffer: &mut dyn Write,
        reader: &mut dyn BufRead,
    ) -> Result<RickrollObject, Error> {
        if !self.functions.contains_key(&func) && BUILTIN_FUNCTIONS.contains_key(&func) {
            let mut arg_vals = Vec::new();
            for arg in passed {
                arg_vals.push(arg.clone());
            }
            return BUILTIN_FUNCTIONS[&func](arg_vals, buffer, reader);
        }
        let function = self.functions.get(&func).unwrap().clone();
        match function {
            ASTNode::Function(_, _, args, body) => {
                // function arguments
                for (arg, val) in args.iter().zip(passed.iter()) {
                    scope.add_var(arg.clone());
                    scope.set_var(arg.clone(), val.clone());
                }
                for node in body {
                    let res = self.execute(&node, scope, buffer, reader)?;
                    match res {
                        Some(obj) => { 
                            return Ok(obj);
                        },
                        None => (),
                    }
                }
                return Ok(RickrollObject::Undefined);
            }
            _ => panic!("Interpreter::run_function called with non-function"),
        };
    }

    // execute the program
    pub fn run(
        &mut self,
        buffer: &mut dyn Write,
        reader: &mut dyn BufRead,
    ) -> Result<RickrollObject, Error> {
        let mut global_scope = Scope::new();
        if self.functions.contains_key(&String::from("[INTRO]")) {
            self.run_function(String::from("[INTRO]"), Vec::new(), &mut global_scope, buffer, reader)?;
        }
        if self.functions.contains_key(&String::from("[CHORUS]")) {
            global_scope.push(Context::new());
            let val = self.run_function(
                String::from("[CHORUS]"),
                Vec::new(),
                &mut global_scope,
                buffer,
                reader,
            );
            global_scope.pop();
            return val;
        } else {
            return Err(Error::new(
                ErrorType::RuntimeError,
                "No main function found",
                None,
            ));
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use super::Operator::*;
    use super::*;
    use Instruction::*;
    use RickrollObject::*;
    use Token::*;

    // helper functions
    fn get(s: Vec<(usize, Instruction)>, stdout: Vec<u8>, stdin: String) -> String {
        let mut stdout = stdout;
        use std::io::BufReader;
        let res = Interpreter::new(Bytecode::from(s))
            .run(&mut stdout, &mut BufReader::new(stdin.as_bytes()));
        return match res {
            Ok(_) => String::from_utf8(stdout).unwrap(),
            Err(err) => String::from(format!("{:?}", err)),
        };
    }

    fn assert_eqv(raw: Vec<(usize, Instruction)>, stdin: &str, res: &str) {
        assert_eq!(&get(raw, Vec::new(), String::from(stdin))[..], res);
    }

    #[test]
    fn simple() {
        assert_eqv(
            vec![
                (1, Put(vec![Value(Int(1)), Operator(Add), Value(Int(2))])),
                (
                    2,
                    Put(vec![Value(Int(3)), Operator(Greater), Value(Int(4))]),
                ),
                (0, End()),
            ],
            "",
            "3\nFALSE\n",
        );
        assert_eqv(
            vec![
                (1, Let("a".to_string())),
                (2, Set("a".to_string(), vec![Value(Int(3))])),
                (3, Let("b".to_string())),
                (4, Put(vec![Variable("a".to_string())])),
                (5, Put(vec![Variable("b".to_string())])),
                (
                    6,
                    Put(vec![
                        Variable("a".to_string()),
                        Operator(Add),
                        Value(Int(3)),
                    ]),
                ),
                (0, End()),
            ],
            "",
            "3\nUNDEFINED\n6\n",
        );
    }

    #[test]
    fn check() {
        assert_eqv(
            vec![
                (1, Let(String::from("n"))),
                (2, Set(String::from("n"), vec![Value(Int(10))])),
                (3, Let(String::from("first"))),
                (4, Let(String::from("second"))),
                (5, Set(String::from("first"), vec![Value(Int(0))])),
                (6, Set(String::from("second"), vec![Value(Int(1))])),
                (7, Put(vec![Variable(String::from("second"))])),
                (
                    8,
                    Jmpif(
                        vec![
                            Variable(String::from("n")),
                            Operator(NotEquals),
                            Value(Int(0)),
                        ],
                        9,
                    ),
                ),
                (8, Jmp(18)),
                (8, Pctx()),
                (9, Let(String::from("sum"))),
                (
                    10,
                    Set(
                        String::from("sum"),
                        vec![
                            Variable(String::from("first")),
                            Operator(Add),
                            Variable(String::from("second")),
                        ],
                    ),
                ),
                (11, Put(vec![Variable(String::from("sum"))])),
                (
                    12,
                    Set(
                        String::from("first"),
                        vec![Variable(String::from("second"))],
                    ),
                ),
                (
                    13,
                    Set(String::from("second"), vec![Variable(String::from("sum"))]),
                ),
                (
                    14,
                    Set(
                        String::from("n"),
                        vec![
                            Variable(String::from("n")),
                            Operator(Subtract),
                            Value(Int(1)),
                        ],
                    ),
                ),
                (15, Dctx()),
                (15, Jmp(7)),
                (0, End()),
            ],
            "",
            "1\n1\n2\n3\n5\n8\n13\n21\n34\n55\n89\n",
        );
        assert_eqv(
            vec![
                (1, Let(String::from("a"))),
                (2, Set(String::from("a"), vec![Value(Int(5))])),
                (
                    3,
                    Jmpif(
                        vec![Variable(String::from("a")), Operator(Equals), Value(Int(5))],
                        4,
                    ),
                ),
                (3, Jmp(7)),
                (3, Pctx()),
                (4, Put(vec![Value(Bool(true))])),
                (5, Dctx()),
                (0, End()),
            ],
            "",
            "TRUE\n",
        );
    }

    #[test]
    fn error() {
        assert_eqv(
            vec![
                (1, Put(vec![Value(Int(3)), Operator(And)]))
            ],
            "",
            "Error { err: Traceback, desc: \"\", line: Some(1), child: Some(Error { err: IllegalArgumentError, desc: \"Not enough arguments\", line: None, child: None }) }",
        );
        assert_eqv(
            vec![
                (1, Jmpif(vec![Value(Int(5))], 2)),
                (2, End()),
            ],
            "",
            "Error { err: IllegalArgumentError, desc: \"Unexpected non-boolean argument\", line: Some(1), child: None }",
        )
    }
}
*/
