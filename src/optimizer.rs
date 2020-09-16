use crate::error::*;
use crate::lexer::Intermediate;

#[derive(Debug)]
pub struct OptimizerOptions {
    optimize: bool,
}

// optimize intermediate representation
#[derive(Debug)]
pub struct Optimizer {
    ptr: usize,
    code: Intermediate,
    curfunc: Option<&'static str>, // current functionality of optimizer
    options: OptimizerOptions,
}

impl Optimizer {
    pub fn new(code: Intermediate, options: OptimizerOptions) -> Optimizer {
        Optimizer {
            ptr: 0,
            code,
            curfunc: None,
            options,
        }
    }

    fn advance(&mut self) {
        self.ptr += 1;
    }

    fn has_more(&self) -> bool {
        self.ptr < self.code.len()
    }

    // wraps a traceback around a possible error
    fn wrap_check<T>(&self, res: Result<T, Error>) -> Result<T, Error> {
        if let Err(error) = res {
            return Err(Error::traceback(
                error,
                Some(self.code.debug_line(self.ptr)),
            ));
        }
        return res;
    }

    pub fn optimize(mut self) -> Result<Intermediate, Error> {
        if !self.options.optimize {
            return Ok(self.code);
        }
        panic!("Optimization is not yet implemented.");
    }
}

#[cfg(test)]
mod tests {}
