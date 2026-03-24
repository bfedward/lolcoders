use std::fmt::{self};

use crate::types::Identifier;

pub enum AppError {
    HaiMustBeFirstLine,
    KThxByeMustBeLastLine,
    ParseError,
    FunctionDoesNotExist(Identifier),
    NotEnoughArgsForFunction,
    InvalidIdentifier(String),
    TokenCannotBeExpression,
    MissingExpression,
    UnexpectedTokensInExpression,
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
            AppError::InvalidIdentifier(name) => {
                write!(f, "Invalid variable identifier: {name}")
            }
            AppError::TokenCannotBeExpression => {
                write!(f, "Token cannot be expression")
            }
            AppError::MissingExpression => {
                write!(f, "Missing expression")
            }
            AppError::UnexpectedTokensInExpression => {
                write!(f, "Unexpected tokens in expression")
            }
        }
    }
}
