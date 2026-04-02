use std::fmt::{self};

use crate::{lexer::Tokens, types::identifier::Identifier};

pub enum AppError {
    HaiMustBeFirstLine,
    KThxByeMustBeLastLine,
    VariableDoesNotExist(Identifier),
    FunctionDoesNotExist(Identifier),
    NotEnoughArgsForFunction,
    InvalidIdentifier(String),
    TokenCannotBeExpression,
    MissingExpression,
    UnexpectedTokensInExpression,
    CouldNotGetCurrentVariableScope,
    CannotReturnFromFunctionOutsideFunction,
    FunctionMustEndIfUSaySo,
    UnknownVariableType,
    IncorrectFunctionArguments(Identifier),
    LineParseError(Tokens),
    FunctionArgumentsMustEndWithMkay,
    FunctionParseError,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::HaiMustBeFirstLine => write!(f, "Must start with HAI"),
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
            AppError::FunctionMustEndIfUSaySo => {
                write!(f, "Function must end IF U SAY SO")
            }
            AppError::UnknownVariableType => {
                write!(f, "Unknown variable type")
            }
            AppError::IncorrectFunctionArguments(called_func) => {
                write!(f, "Incorrect function arguments for {called_func}")
            }
            AppError::LineParseError(tokens) => {
                write!(f, "Line parse error:\t{tokens}")
            }
            AppError::FunctionArgumentsMustEndWithMkay => {
                write!(f, "Function arguments must end with MKAY")
            }
            AppError::FunctionParseError => {
                write!(f, "Could not parse function")
            }
        }
    }
}
