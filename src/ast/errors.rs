use std::error::Error;
use std::fmt::{Display, Formatter, self};
use std::convert::From;

#[derive(Debug, Clone)]
pub struct SyntaxError(pub String);

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "SyntaxError: {}", self.0)
    }
}

impl From<&str> for SyntaxError {
    fn from(string: &str) -> Self {
        Self(String::from(string))
    }
}

impl From<String> for SyntaxError {
    fn from(string: String) -> Self {
        Self(string)
    }
}

impl Error for SyntaxError { }

#[derive(Debug, Clone)]
pub enum RuntimeError {
    NameError(String),
    OperatorError(String),  // e.g. a + b is legal syntactically but not if a and b are not add-able
    TypeError(String),  // e.g. condition part of piecewise block did not return a Boolean
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use RuntimeError::*;
        match self {
            NameError(msg) => write!(f, "NameError: {}", msg),
            OperatorError(msg) => write!(f, "OperatorError: {}", msg),
            TypeError(msg) => write!(f, "TypeError: {}", msg),
        }
    }
}

impl Error for RuntimeError { }
