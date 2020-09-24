use crate::compiler::{Bytecode, Function, Instruction};
use crate::error::*;
use crate::parser::Parser;
use crate::tokenizer::Token;
use crate::util::*;

use std::collections::VecDeque;
use std::io::{BufRead, Write};

pub const MAX_RECURSION_DEPTH: usize = 10000;
pub const MAX_UNWIND_LIMIT: usize = 8;

#[derive(Debug)]
pub struct Interpreter {
    bytecode: Bytecode,
    scope: Scope,                           // global scope -> scope1... -> current scope
    function_stack: Vec<(Function, usize)>, // function call stack
    context_stack: Vec<Vec<Context>>,       // context for function calls
    arg_queue: VecDeque<RickrollObject>,    // function argument queue
}

impl Interpreter {
    pub fn new(bytecode: Bytecode) -> Interpreter {
        Interpreter {
            bytecode,
            scope: Scope::new(), // global scope is constructed inside Scope::new
            function_stack: Vec::new(),
            context_stack: Vec::new(),
            arg_queue: VecDeque::new(),
        }
    }

    // displays debug info
    fn unwind_stack(&mut self, err: Error) -> Error {
        let mut unwind_count = 0;
        let mut err = err;
        while !self.function_stack.is_empty() {
            let (func, line) = self.function_stack.pop().unwrap();
            if line >= func.len() || func.debug_line(line) == 0 {
                err = Error::traceback(err, None);
            } else {
                err = Error::traceback(err, Some(func.debug_line(line)));
            }
            unwind_count += 1;
            if unwind_count > MAX_UNWIND_LIMIT {
                break;
            }
        }
        return err;
    }

    // evaluates an expression using the parser and error-wraps its result
    fn eval(&self, tokens: Vec<Token>) -> Result<RickrollObject, Error> {
        let parser = Parser::new(tokens, self.scope.clone());
        return parser.eval();
    }

    // executes a function
    pub fn run<W, R>(
        &mut self,
        func: String,
        buffer: &mut W,
        reader: &mut R,
    ) -> Result<RickrollObject, Error>
    where
        W: Write,
        R: BufRead,
    {
        let mut function = self.bytecode.get_func(func.clone()); // get function bytecode
        self.function_stack.push((function.clone(), 0)); // push current function to stack
        let mut ptr = 0;
        loop {
            // stack overflow
            if self.function_stack.len() >= MAX_RECURSION_DEPTH {
                return Err(Error::new(
                    ErrorType::StackOverflowError,
                    &(format!(
                        "Too many recursive calls for function {}",
                        function.get_name()
                    ))[..],
                    Some(self.bytecode.get_func(func.clone()).debug_line(0)),
                ));
            }
            let opcode = &function[ptr];
            use Instruction::*;
            match opcode {
                Put(tokens) => {
                    writeln!(buffer, "{}", self.eval(tokens.clone())?)
                        .expect("Error when writing to buffer");
                }
                Let(varname) => {
                    self.scope.add_var(varname.clone());
                }
                Glb(varname) => {
                    self.scope
                        .get_global()
                        .set_var(varname.clone(), RickrollObject::Undefined);
                }
                Set(varname, tokens) => {
                    let val = self.eval(tokens.clone())?;
                    self.scope.set_var(varname.clone(), val);
                }
                Jmp(dest) => {
                    ptr = *dest;
                    continue; // do not advance()
                }
                Jmpif(tokens, dest) => {
                    // jump??
                    let val = self.eval(tokens.clone())?;
                    let jump = match val {
                        RickrollObject::Bool(x) => x,
                        _ => {
                            return Err(Error::new(
                                ErrorType::IllegalArgumentError,
                                "Unexpected non-boolean argument",
                                Some(function.debug_line(ptr)),
                            ));
                        }
                    };
                    if jump {
                        ptr = *dest;
                        continue; // do not advance()
                    }
                }
                Pctx() => {
                    self.scope.push(Context::new());
                }
                Dctx() => {
                    self.scope.pop();
                }
                Call(func) => {
                    self.function_stack.last_mut().unwrap().1 = ptr;
                    self.context_stack.push(self.scope.behead()); // store current state
                    function = self.bytecode.get_func(func.clone()); // replace function
                    ptr = 0;
                    self.function_stack.push((function.clone(), 0));
                    continue;
                }
                Scall(_, func) => {
                    self.function_stack.last_mut().unwrap().1 = ptr;
                    self.context_stack.push(self.scope.behead()); // store current state
                    function = self.bytecode.get_func(func.clone());
                    ptr = 0;
                    self.function_stack.push((function.clone(), 0));
                    continue;
                }
                Ret(tokens) => {
                    let result = self.eval(tokens.clone())?; // eval result
                    self.scope.behead(); // remove all contexts except for global
                    self.function_stack.pop(); // pop current function
                                               // no more functions
                    if self.function_stack.is_empty() {
                        return Ok(result);
                    }
                    self.scope.push_all(self.context_stack.pop().unwrap()); // repush contexts
                                                                            // replace function and ptr with last value
                    let last = self.function_stack.last().unwrap();
                    function = last.0.clone();
                    ptr = last.1;
                    // check for scall
                    if let Scall(varname, _) = function[ptr].clone() {
                        // set variable
                        self.scope.set_var(varname, result);
                    }
                }
                Pushq(var) => {
                    self.arg_queue
                        .push_back(self.scope.get_var(var.clone()).unwrap());
                }
                Exp(var) => {
                    self.scope.add_var(var.clone());
                    self.scope
                        .set_var(var.clone(), self.arg_queue.pop_front().unwrap());
                }
            }
            /* debug
            let mut test = Vec::new();
            for x in self.function_stack.iter() {
                let (func, line) = x.clone();
                test.push((func.get_name().clone(), line));
            }
            test.last_mut().unwrap().1 = ptr;
            println!("{:?}", test);
            println!("{:?}\n", self.scope);
            */
            ptr += 1; // increment pointer
        }
    }

    // takes in a mutable buffer and reader rather than
    // writing to stdout and reading from stdin
    pub fn execute<W, R>(mut self, mut buffer: W, mut reader: R) -> Result<RickrollObject, Error>
    where
        W: Write,
        R: BufRead,
    {
        // must have a main to execute
        if !self.bytecode.has_main() {
            return Err(Error::new(
                ErrorType::RuntimeError,
                "Could not find a [Chorus] to execute",
                None,
            ));
        }
        // global is optional
        if self.bytecode.has_global() {
            // run global
            match self.run(String::from("[Global]"), &mut buffer, &mut reader) {
                Err(error) => {
                    return Err(self.unwind_stack(error));
                }
                Ok(_) => (),
            }
        }
        // run main
        return match self.run(String::from("[Main]"), &mut buffer, &mut reader) {
            Err(error) => {
                return Err(self.unwind_stack(error));
            }
            Ok(obj) => Ok(obj),
        };
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
