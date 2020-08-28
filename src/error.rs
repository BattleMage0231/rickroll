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
}

// get name from enum member
impl ErrorType {
    fn as_string(&self) -> String {
        match self {
            ErrorType::IllegalCharError => "Illegal Character",
            ErrorType::RuntimeError => "Runtime Error",
            ErrorType::IllegalArgumentError => "Illegal Argument",
            ErrorType::SyntaxError => "Syntax Error",
            ErrorType::IllegalCastError => "Illegal Cast",
            ErrorType::IndexOutOfBoundsError => "Index Out of Bounds",
            ErrorType::FileError => "File Error",
            ErrorType::NameError => "Name Error",
            ErrorType::Traceback => "Traceback",
        }
        .to_string()
    }
}

#[derive(Debug)]
pub struct Error {
    err: ErrorType,
    desc: String,
    line: Option<usize>, // line could not exist
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
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        // error name
        res += &self.err.as_string()[..];
        // error line if exists
        if Option::is_some(&self.line) {
            res = format!("{} on line {}", res, self.line.unwrap());
        }
        // error description
        res = format!("{}: {}", res, self.desc);
        write!(f, "{}", res)
    }
}
