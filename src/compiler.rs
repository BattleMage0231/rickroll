use crate::error::*;
use crate::lexer::{Intermediate, Statement};
use crate::tokenizer::Token;

use std::collections::HashMap;
use std::ops::{Index, IndexMut};

// bytecode instruction
#[derive(Debug, Clone)]
pub enum Instruction {
    // print
    Put(Vec<Token>),
    // let, globally let, and set variables
    Let(String),
    Glb(String),
    Set(String, Vec<Token>),
    // jump and conditionally jump
    Jmp(usize),
    Jmpif(Vec<Token>, usize),
    // push and pop context
    Pctx(),
    Dctx(),
    // function instructions
    Call(String),
    Scall(String, String),
    Ret(Vec<Token>),
    // argument queue instructions
    Pushq(String),
    Exp(String),
}

// bytecode function
#[derive(Debug, Clone)]
pub struct Function {
    name: String,
    instructions: Vec<Instruction>,
    args: Vec<String>,
    debug_lines: Vec<usize>,
    /*
     * file: String,
     */
}

impl Function {
    pub fn new(name: String, args: Vec<String>) -> Function {
        Function {
            name,
            instructions: Vec::new(),
            args,
            debug_lines: Vec::new(),
        }
    }

    pub fn set_args(&mut self, args: Vec<String>) {
        self.args = args;
    }

    pub fn get_args(&self) -> &Vec<String> {
        &self.args
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    pub fn from(name: String, args: Vec<String>, vec: Vec<(usize, Instruction)>) -> Function {
        let mut func = Function::new(name, args);
        for (line, instruction) in vec {
            func.push(instruction, line);
        }
        return func;
    }

    pub fn to_vec(&self) -> Vec<(usize, Instruction)> {
        let mut res: Vec<(usize, Instruction)> = Vec::new();
        for i in 0..self.len() {
            res.push((self.debug_lines[i], self.instructions[i].clone()));
        }
        return res;
    }

    pub fn debug_line(&self, index: usize) -> usize {
        self.debug_lines[index]
    }

    pub fn push(&mut self, instruction: Instruction, orig_line: usize) {
        self.instructions.push(instruction);
        self.debug_lines.push(orig_line);
    }
}

// index into function using [] operator
impl Index<usize> for Function {
    type Output = Instruction;

    fn index<'a>(&'a self, index: usize) -> &'a Instruction {
        &self.instructions[index]
    }
}

impl IndexMut<usize> for Function {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Instruction {
        &mut self.instructions[index]
    }
}

// bytecode definition
#[derive(Debug)]
pub struct Bytecode {
    functions: HashMap<String, Function>,
    has_main: bool,
    has_global: bool,
    /*
     * main_file: String,
     */
}

impl Bytecode {
    pub fn new() -> Bytecode {
        Bytecode {
            functions: HashMap::new(),
            has_main: false,
            has_global: false,
        }
    }

    pub fn has_main(&self) -> bool {
        self.has_main
    }

    pub fn has_global(&self) -> bool {
        self.has_global
    }

    pub fn has_func(&self, func: &String) -> bool {
        self.functions.contains_key(func)
    }

    pub fn debug_line(&self, func: String, index: usize) -> usize {
        self.functions.get(&func).unwrap().debug_line(index)
    }

    // pushes a function into the bytecode
    // replaces old function if same name
    pub fn push(&mut self, func: Function) {
        self.functions.insert(func.get_name().clone(), func);
    }

    // main function
    pub fn set_main(&mut self, mut func: Function) {
        func.set_name(String::from("[Main]"));
        self.functions.insert(String::from("[Main]"), func);
        self.has_main = true;
    }

    // global function (i.e. Intro block)
    pub fn set_global(&mut self, mut func: Function) {
        func.set_name(String::from("[Global]"));
        self.functions.insert(String::from("[Global]"), func);
        self.has_global = true;
    }

    pub fn get_func(&self, name: String) -> Function {
        self.functions.get(&name).unwrap().clone()
    }
}

#[derive(Debug)]
pub struct Compiler {
    ptr: usize,
    func_ptr: usize,
    raw: Intermediate,
    check_stack: Vec<usize>, // remember lines following those that have check statements
    compiled: Bytecode,
}

impl Compiler {
    pub fn new(raw: Intermediate) -> Compiler {
        Compiler {
            ptr: 0,
            func_ptr: 0,
            raw,
            check_stack: Vec::new(),
            compiled: Bytecode::new(),
        }
    }

    fn advance(&mut self) {
        self.ptr += 1;
    }

    fn has_more(&self) -> bool {
        self.ptr < self.raw.len()
    }

    // wraps a traceback around a possible error
    fn wrap_check<T>(&self, res: Result<T, Error>) -> Result<T, Error> {
        if let Err(error) = res {
            return Err(Error::traceback(error, Some(self.raw.debug_line(self.ptr))));
        }
        return res;
    }

    fn parse_program(&mut self) -> Result<(), Error> {
        while self.has_more() {
            let block = self.raw[self.ptr].clone();
            use Statement::*;
            match block {
                Chorus() => {
                    self.parse_main()?;
                }
                Intro() => {
                    self.parse_global()?;
                }
                Verse(_, _) => {
                    self.parse_function()?;
                }
                _ => {
                    return Err(Error::new(
                        ErrorType::SyntaxError,
                        "Unblocked statement",
                        Some(self.raw.debug_line(self.ptr)),
                    ));
                }
            }
            assert!(self.check_stack.is_empty());
            self.func_ptr = 0;
        }
        Ok(())
    }

    // parses the global function ([Intro])
    fn parse_global(&mut self) -> Result<(), Error> {
        // pointer at [Intro]
        self.advance();
        // make function
        let mut global: Function = Function::new(String::from("[Global]"), Vec::new());
        global.push(Instruction::Pctx(), 0); // new context
        self.func_ptr += 1;
        while self.has_more() {
            let statement = self.raw[self.ptr].clone();
            match &statement {
                Statement::Chorus() | Statement::Verse(_, _) => break,
                // in the global function, all variables are global
                Statement::Let(varname) => {
                    global.push(
                        Instruction::Glb(varname.clone()),
                        self.raw.debug_line(self.ptr),
                    );
                    self.func_ptr += 1;
                }
                _ => self.parse_common(&mut global, statement)?,
            }
            self.advance();
        }
        global.push(Instruction::Dctx(), 0); // pop context
        self.func_ptr += 1;
        self.compiled.set_global(global);
        return Ok(());
    }

    // parses the main function
    fn parse_main(&mut self) -> Result<(), Error> {
        // pointer at [Chorus]
        self.advance();
        // make function
        let mut main: Function = Function::new(String::from("[Main]"), Vec::new());
        main.push(Instruction::Pctx(), 0); // new context
        self.func_ptr += 1;
        while self.has_more() {
            let statement = self.raw[self.ptr].clone();
            match statement {
                Statement::Intro() | Statement::Verse(_, _) => break,
                _ => self.parse_common(&mut main, statement)?,
            }
            self.advance();
        }
        main.push(Instruction::Dctx(), 0); // pop context
        self.func_ptr += 1;
        self.compiled.set_main(main);
        return Ok(());
    }

    fn parse_function(&mut self) -> Result<(), Error> {
        // pointer at [Verse X]
        let func_sig = match self.raw[self.ptr].clone() {
            Statement::Verse(name, args) => (name, args),
            _ => panic!("parse_function() called without being at [Verse]"),
        };
        let (func_name, func_args) = func_sig;
        self.advance();
        // make function
        let mut func: Function = Function::new(func_name.clone(), func_args.clone());
        func.push(Instruction::Pctx(), 0); // new context
        self.func_ptr += 1;
        for var in func_args {
            // expect arguments
            func.push(Instruction::Exp(var), self.raw.debug_line(self.ptr));
            self.func_ptr += 1;
        }
        while self.has_more() {
            let statement = self.raw[self.ptr].clone();
            match statement {
                Statement::Intro() | Statement::Chorus() => break,
                _ => self.parse_common(&mut func, statement)?,
            }
            self.advance();
        }
        func.push(Instruction::Dctx(), 0); // pop context
        self.func_ptr += 1;
        self.compiled.push(func);
        return Ok(());
    }

    // parse common statement that doesn't vary between functions
    // advances pointer to next statement
    fn parse_common(&mut self, function: &mut Function, statement: Statement) -> Result<(), Error> {
        use Instruction::*;
        use Statement::*;
        match statement {
            Chorus() | Intro() | Verse(_, _) => panic!("Blocks cannot be commonly parsed"),
            // print
            Say(tokens) => {
                function.push(Put(tokens), self.raw.debug_line(self.ptr));
                self.func_ptr += 1;
            }
            // let and assign variables
            Statement::Let(varname) => {
                function.push(Instruction::Let(varname), self.raw.debug_line(self.ptr));
                self.func_ptr += 1;
            }
            Assign(varname, tokens) => {
                function.push(Set(varname, tokens), self.raw.debug_line(self.ptr));
                self.func_ptr += 1;
            }
            /*
             * If compiles to
             * 0 jmpif [TRUE] 2 ; if true, jump to start of code
             * 1 jmp 5          ; if not true, jump to end of if
             * 2 pctx           ; push context
             * 3 put 0
             * 4 dctx           ; delete context
             * 5 end
             *
             * While compiles to
             * 0 jmpif [TRUE] 2 ; if true, jump to start of code
             * 1 jmp 6          ; if not true, jump to end of loop
             * 2 pctx           ; push context
             * 3 put 0
             * 4 dctx           ; delete context
             * 5 jmp 0          ; jump back to loop start
             * 6 end
             */
            // while loops and if statements
            Check(tokens) => {
                let debug_line = self.raw.debug_line(self.ptr);
                // skip the next line if tokens evaluates to true
                function.push(Jmpif(tokens, self.func_ptr + 2), debug_line);
                self.func_ptr += 1;
                // jump to end of loop/if
                // we don't know where that is, so put a temporary value for now
                function.push(Jmp(usize::MAX), debug_line);
                self.check_stack.push(self.func_ptr); // store index of jmp(umax)
                self.func_ptr += 1;
                // add new context
                function.push(Pctx(), debug_line);
                self.func_ptr += 1;
            }
            WhileEnd() => {
                let debug_line = self.raw.debug_line(self.ptr);
                // delete context
                function.push(Dctx(), debug_line);
                self.func_ptr += 1;
                // jump back to condition checking
                let top = self.check_stack.pop().unwrap(); // pop last check index
                function.push(Jmp(top - 1), debug_line);
                self.func_ptr += 1;
                // replace temporary index from before
                function[top] = Jmp(self.func_ptr);
            }
            IfEnd() => {
                let debug_line = self.raw.debug_line(self.ptr);
                // delete context
                function.push(Dctx(), debug_line);
                self.func_ptr += 1;
                // replace temporary index from before
                let top = self.check_stack.pop().unwrap(); // pop last check index
                function[top] = Jmp(self.func_ptr);
            }
            Return(tokens) => {
                function.push(Ret(tokens), self.raw.debug_line(self.ptr));
                self.func_ptr += 1;
            }
            Run(func, args) => {
                let debug_line = self.raw.debug_line(self.ptr);
                for var in args {
                    function.push(Pushq(var), debug_line);
                    self.func_ptr += 1;
                }
                function.push(Call(func), debug_line);
                self.func_ptr += 1;
            }
            RunAssign(func, var, args) => {
                let debug_line = self.raw.debug_line(self.ptr);
                for var in args {
                    function.push(Pushq(var), debug_line);
                    self.func_ptr += 1;
                }
                function.push(Scall(func, var), debug_line);
                self.func_ptr += 1;
            }
        }
        return Ok(());
    }

    pub fn compile(mut self) -> Result<Bytecode, Error> {
        match self.parse_program() {
            Ok(()) => Ok(self.compiled),
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    /*
    use super::*;
    use crate::tokenizer::Token::*;
    use crate::util::{Operator::*, RickrollObject::*};
    use Statement::*;

    // helper functions
    fn get(s: Vec<(usize, Statement)>) -> String {
        String::from(format!(
            "{:?}",
            Compiler::new(Intermediate::from(s)).compile()
        ))
    }

    fn assert_eqv(raw: Vec<(usize, Statement)>, res: &str) {
        assert_eq!(&get(raw)[..], res);
    }

    #[test]
    fn simple() {
        assert_eqv(
            vec![(1, Say(vec![Value(Int(1)), Operator(Add), Value(Int(2))])), (2, Say(vec![Value(Int(3)), Operator(Greater), Value(Int(4))]))],
            "Ok(Bytecode { instructions: [Put([Value(Int(1)), Operator(Add), Value(Int(2))]), Put([Value(Int(3)), Operator(Greater), Value(Int(4))]), End], debug_lines: [1, 2, 0] })",
        );
        assert_eqv(
            vec![(1, Let(String::from("a"))), (2, Assign(String::from("a"), vec![Value(Int(3))])), (3, Let(String::from("b"))), (4, Say(vec![Variable(String::from("a"))])), (5, Say(vec![Variable(String::from("b"))])), (6, Say(vec![Variable(String::from("a")), Operator(Add), Value(Int(3))]))],
            "Ok(Bytecode { instructions: [Let(\"a\"), Set(\"a\", [Value(Int(3))]), Let(\"b\"), Put([Variable(\"a\")]), Put([Variable(\"b\")]), Put([Variable(\"a\"), Operator(Add), Value(Int(3))]), End], debug_lines: [1, 2, 3, 4, 5, 6, 0] })",
        );
    }

    // while loops and if statements
    #[test]
    fn check() {
        assert_eqv(
            vec![(1, Let(String::from("n"))), (2, Assign(String::from("n"), vec![Value(Int(10))])), (3, Let(String::from("first"))), (4, Let(String::from("second"))), (5, Assign(String::from("first"), vec![Value(Int(0))])), (6, Assign(String::from("second"), vec![Value(Int(1))])), (7, Say(vec![Variable(String::from("second"))])), (8, Check(vec![Variable(String::from("n")), Operator(NotEquals), Value(Int(0))])), (9, Let(String::from("sum"))), (10, Assign(String::from("sum"), vec![Variable(String::from("first")), Operator(Add), Variable(String::from("second"))])), (11, Say(vec![Variable(String::from("sum"))])), (12, Assign(String::from("first"), vec![Variable(String::from("second"))])), (13, Assign(String::from("second"), vec![Variable(String::from("sum"))])), (14, Assign(String::from("n"), vec![Variable(String::from("n")), Operator(Subtract), Value(Int(1))])), (15, WhileEnd())],
            "Ok(Bytecode { instructions: [Let(\"n\"), Set(\"n\", [Value(Int(10))]), Let(\"first\"), Let(\"second\"), Set(\"first\", [Value(Int(0))]), Set(\"second\", [Value(Int(1))]), Put([Variable(\"second\")]), Jmpif([Variable(\"n\"), Operator(NotEquals), Value(Int(0))], 9), Jmp(18), Pctx, Let(\"sum\"), Set(\"sum\", [Variable(\"first\"), Operator(Add), Variable(\"second\")]), Put([Variable(\"sum\")]), Set(\"first\", [Variable(\"second\")]), Set(\"second\", [Variable(\"sum\")]), Set(\"n\", [Variable(\"n\"), Operator(Subtract), Value(Int(1))]), Dctx, Jmp(7), End], debug_lines: [1, 2, 3, 4, 5, 6, 7, 8, 8, 8, 9, 10, 11, 12, 13, 14, 15, 15, 0] })",
        );
        assert_eqv(
            vec![(1, Let(String::from("a"))), (2, Assign(String::from("a"), vec![Value(Int(5))])), (3, Check(vec![Variable(String::from("a")), Operator(Equals), Value(Int(5))])), (4, Say(vec![Value(Bool(true))])), (5, IfEnd())],
            "Ok(Bytecode { instructions: [Let(\"a\"), Set(\"a\", [Value(Int(5))]), Jmpif([Variable(\"a\"), Operator(Equals), Value(Int(5))], 4), Jmp(7), Pctx, Put([Value(Bool(true))]), Dctx, End], debug_lines: [1, 2, 3, 3, 3, 4, 5, 0] })",
        );
    }*/
}
