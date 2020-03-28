use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct ConfigError;

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid configuration")
    }
}

impl error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct InvalidCommandError {
    msg: String,
}

impl InvalidCommandError {
    pub fn new(msg: &str) -> InvalidCommandError {
        InvalidCommandError {
            msg: String::from(msg),
        }
    }
}

impl fmt::Display for InvalidCommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid command: {}", self.msg)
    }
}

impl error::Error for InvalidCommandError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct CompilationFailed;

impl fmt::Display for CompilationFailed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Compilation failed")
    }
}

impl error::Error for CompilationFailed {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
