use crate::compiler::{Bytecode, Instruction};
use crate::error::*;
use crate::parser::Parser;
use crate::tokenizer::Token;
use crate::util::*;

use std::collections::VecDeque;
use std::io::{BufRead, Write};

pub const MAX_RECURSION_DEPTH: usize = 8000;

#[derive(Debug)]
pub struct Interpreter {
    bytecode: Bytecode,
    scope: Scope,                        // global scope -> scope1... -> current scope
    function_stack: Vec<String>,         // function call stack
    context_stack: Vec<Vec<Context>>,    // context stack for function calls
    arg_queue: VecDeque<RickrollObject>, // function argument queue
}

impl Interpreter {
    pub fn new(bytecode: Bytecode) -> Interpreter {
        Interpreter {
            bytecode,
            scope: Scope::new(),
            function_stack: Vec::new(),
            context_stack: Vec::new(),
            arg_queue: VecDeque::new(),
        }
    }

    // wraps a traceback around a possible error
    fn wrap_check<T>(&self, res: Result<T, Error>, ptr: usize) -> Result<T, Error> {
        if let Err(error) = res {
            return Err(Error::traceback(
                error,
                Some(
                    self.bytecode
                        .get_func(self.function_stack.last().unwrap().clone())
                        .debug_line(ptr),
                ),
            ));
        }
        return res;
    }

    // evaluates an expression using the parser and error-wraps its result
    fn eval(&self, tokens: Vec<Token>, ptr: usize) -> Result<RickrollObject, Error> {
        let parser = Parser::new(tokens, self.scope.clone());
        return self.wrap_check(parser.eval(), ptr);
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
        if self.function_stack.len() >= MAX_RECURSION_DEPTH {
            return Err(Error::new(
                ErrorType::StackOverflowError,
                &(format!("Too many recursive calls for function {}", func))[..],
                Some(self.bytecode.get_func(func.clone()).debug_line(0)),
            ));
        }
        // push current function to stack
        self.function_stack.push(func.clone());
        let mut ptr = 0; // function ptr
        let function = self.bytecode.get_func(func.clone()); // get function bytecode
        while ptr < function.len() {
            let opcode = &function[ptr];
            use Instruction::*;
            match opcode {
                Put(tokens) => {
                    writeln!(buffer, "{}", self.eval(tokens.clone(), ptr)?)
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
                    let val = self.eval(tokens.clone(), ptr)?;
                    self.scope.set_var(varname.clone(), val);
                }
                Jmp(dest) => {
                    ptr = *dest;
                    continue; // do not advance()
                }
                Jmpif(tokens, dest) => {
                    // jump??
                    let val = self.eval(tokens.clone(), ptr)?;
                    let jump = match val {
                        RickrollObject::Bool(x) => x,
                        _ => {
                            return Err(Error::new(
                                ErrorType::IllegalArgumentError,
                                "Unexpected non-boolean argument",
                                Some(function.debug_line(ptr)),
                            ))
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
                    self.context_stack.push(self.scope.behead()); // store current state
                    let res = self.run(func.clone(), buffer, reader); // call function
                                                                      // recursive calls should automatically clean up scope
                    assert!(
                        self.scope.len() == 1,
                        "Scope has more than one context after function call"
                    );
                    self.scope.push_all(self.context_stack.pop().unwrap()); // return state to scope
                    match res {
                        // recursively run function
                        Err(err) => {
                            return Err(Error::traceback(err, Some(function.debug_line(ptr))))
                        }
                        Ok(_) => (),
                    }
                }
                Scall(varname, func) => {
                    self.context_stack.push(self.scope.behead()); // store current state
                    let res = self.run(func.clone(), buffer, reader); // call function
                                                                      // recursive calls should automatically clean up scope
                    assert!(
                        self.scope.len() == 1,
                        "Scope has more than one context after function call"
                    );
                    self.scope.push_all(self.context_stack.pop().unwrap()); // return state to scope
                    match res {
                        // recursively run function
                        Err(err) => {
                            return Err(Error::traceback(err, Some(function.debug_line(ptr))))
                        }
                        Ok(obj) => {
                            self.scope.set_var(varname.clone(), obj); // set variable to return value
                        }
                    }
                }
                Ret(tokens) => {
                    let result = self.eval(tokens.clone(), ptr); // eval result
                                                                 // traceback error
                    if let Err(err) = result {
                        return Err(Error::traceback(err, Some(function.debug_line(ptr))));
                    }
                    self.scope.behead(); // remove all contexts except for global
                    return result;
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
            ptr += 1; // increment pointer
        }
        self.scope.behead(); // remove all contexts except for global
        return Ok(RickrollObject::Undefined);
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
            self.run(String::from("[Global]"), &mut buffer, &mut reader)?; // execute global
        }
        return self.run(String::from("[Main]"), &mut buffer, &mut reader); // execute main
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
