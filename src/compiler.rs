use crate::error::*;
use crate::util::*;

#[derive(Debug)]
pub struct Compiler {
    ptr: usize,
    raw: Intermediate,
    check_stack: Vec<usize>, // remember lines following those that have check statements
    compiled: Bytecode,
}

impl Compiler {
    pub fn new(raw: Intermediate) -> Compiler {
        Compiler {
            ptr: 0,
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

    pub fn compile(mut self) -> Result<Bytecode, Error> {
        while self.has_more() {
            let statement = self.raw[self.ptr].clone();
            // compile statement to bytecode
            use Instruction::*;
            use Statement::*;
            match statement {
                Say(tokens) => {
                    self.compiled
                        .push(Put(tokens), self.raw.debug_line(self.ptr));
                }
                Statement::Let(varname) => {
                    self.compiled
                        .push(Instruction::Let(varname), self.raw.debug_line(self.ptr));
                }
                Assign(varname, tokens) => {
                    self.compiled
                        .push(Set(varname, tokens), self.raw.debug_line(self.ptr));
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
                Check(tokens) => {
                    // skip the next line if tokens evaluates to true
                    self.compiled.push(
                        Jmpif(tokens, self.compiled.len() + 2),
                        self.raw.debug_line(self.ptr),
                    );
                    // jump to end of loop/if
                    // we don't know where that is, so put a temporary value for now
                    self.compiled
                        .push(Jmp(usize::MAX), self.raw.debug_line(self.ptr));
                    self.check_stack.push(self.compiled.len() - 1);
                    // add new context
                    self.compiled.push(Pctx(), self.raw.debug_line(self.ptr));
                }
                WhileEnd() => {
                    self.compiled.push(Dctx(), self.raw.debug_line(self.ptr)); // pop context
                    let top = self.check_stack.pop().unwrap(); // pop last check index
                                                               // jump back to condition checking
                    self.compiled
                        .push(Jmp(top - 1), self.raw.debug_line(self.ptr));
                    // if condition untrue, jump outside of loop
                    self.compiled[top] = Jmp(self.compiled.len());
                }
                IfEnd() => {
                    self.compiled.push(Dctx(), self.raw.debug_line(self.ptr)); // pop context
                    let top = self.check_stack.pop().unwrap(); // pop last check index
                                                               // if condition untrue, jump to end of if statement
                    self.compiled[top] = Jmp(self.compiled.len());
                }
            }
            self.advance();
        }
        self.compiled.push(Instruction::End(), 0);
        return Ok(self.compiled);
    }
}

#[cfg(test)]
mod tests {
    use super::Operator::*;
    use super::*;
    use RickrollObject::*;
    use Statement::*;
    use Token::*;

    // helper functions
    fn get(s: Vec<(usize, Statement)>) -> String {
        String::from(format!("{:?}", Compiler::new(Intermediate::from(s)).compile()))
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
    }
    
}
