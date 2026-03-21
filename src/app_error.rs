use std::fmt::{self};

pub enum AppError {
    HaiMustBeFirstLine,
    KThxByeMustBeLastLine,
    ParseError,
    FunctionDoesNotExist(String),
    NotEnoughArgsForFunction,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::HaiMustBeFirstLine => write!(f, "Must start with HAI"),
            AppError::ParseError => write!(f, "Parse error!"),
            AppError::KThxByeMustBeLastLine => write!(f, "Must end with KTHXBYE"),
            AppError::FunctionDoesNotExist(func) => write!(f, "Function {func} does not exist"),
            AppError::NotEnoughArgsForFunction => {
                write!(f, "Not enough arguments to call function")
            }
        }
    }
}
