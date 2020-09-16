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
mod tests {}
