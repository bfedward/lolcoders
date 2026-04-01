use std::fmt::{self};

use crate::types::identifier::Identifier;

pub enum AppError {
    HaiMustBeFirstLine,
    KThxByeMustBeLastLine,
    ParseError,
    VariableDoesNotExist(Identifier),
    FunctionDoesNotExist(Identifier),
    NotEnoughArgsForFunction,
    InvalidIdentifier(String),
    TokenCannotBeExpression,
    MissingExpression,
    UnexpectedTokensInExpression,
    CouldNotGetCurrentVariableScope,
    CannotReturnFromFunctionOutsideFunction,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::HaiMustBeFirstLine => write!(f, "Must start with HAI"),
            AppError::ParseError => write!(f, "Parse error!"),
            AppError::KThxByeMustBeLastLine => write!(f, "Must end with KTHXBYE"),
            AppError::VariableDoesNotExist(var) => {
                write!(f, "Variable {var} does not exist in current scope")
            }
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
            AppError::CouldNotGetCurrentVariableScope => {
                write!(f, "Could not get current variable scope")
            }
            AppError::CannotReturnFromFunctionOutsideFunction => {
                write!(f, "Cannot return from function outside of a function")
            }
        }
    }
}
