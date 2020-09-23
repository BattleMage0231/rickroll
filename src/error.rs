// all native error types
#[derive(Debug)]
pub enum ErrorType {
    IllegalCharError,
    RuntimeError,
    IllegalArgumentError,
    SyntaxError,
    IllegalCastError,
    IndexOutOfBoundsError,
    FileError,
    NameError,
    Traceback,
    StackOverflowError,
}

// get name from enum member
impl ErrorType {
    fn as_string(&self) -> String {
        use ErrorType::*;
        match self {
            IllegalCharError => "Illegal Character",
            RuntimeError => "Runtime Error",
            IllegalArgumentError => "Illegal Argument",
            SyntaxError => "Syntax Error",
            IllegalCastError => "Illegal Cast",
            IndexOutOfBoundsError => "Index Out of Bounds",
            FileError => "File Error",
            NameError => "Name Error",
            Traceback => "Traceback",
            StackOverflowError => "Stack Overflow",
        }
        .to_string()
    }
}

#[derive(Debug)]
pub struct Error {
    err: ErrorType,
    desc: String,
    // line could not exist
    line: Option<usize>,
    // child could not exist
    child: Box<Option<Error>>,
}

impl Error {
    // make a new error
    pub fn new(err: ErrorType, desc: &str, line: Option<usize>) -> Error {
        // error can't be traceback
        assert_ne!(
            err.as_string(),
            String::from("Traceback"),
            "Error::new() can't make tracebacks"
        );
        Error {
            err,
            desc: String::from(desc),
            line,
            child: Box::new(None),
        }
    }

    pub fn traceback(child: Error, line: Option<usize>) -> Error {
        Error {
            err: ErrorType::Traceback,
            desc: String::from(""),
            line,
            child: Box::new(Some(child)),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        // child if traceback
        if self.child.is_some() {
            // get previous error message from Box<Option<Error>>
            res += &(self.child.as_ref().as_ref().unwrap().to_string())[..];
            res += "\n";
        }
        // error name
        res += &self.err.as_string()[..];
        // error line if exists
        if self.line.is_some() {
            res = format!("{} on line {}", res, self.line.unwrap());
        }
        // error description if not traceback
        if self.child.is_none() {
            res = format!("{}: {}", res, self.desc);
        }
        write!(f, "{}", res)
    }
}
