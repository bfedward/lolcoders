use std::fmt::{self};

use crate::{
    lexer::{Token, Tokens},
    types::{identifier::Identifier, primitive::Yarn},
};

pub enum AppError {
    HaiMustBeFirstLine,
    KThxByeMustBeLastLine,
    VariableDoesNotExist(Identifier),
    FunctionDoesNotExist(Identifier),
    NotEnoughArgsForFunction,
    InvalidIdentifier(String),
    TokenCannotBeExpression(Token),
    MissingExpression,
    CouldNotGetCurrentVariableScope,
    CannotReturnFromFunctionOutsideFunction,
    FunctionMustEndIfUSaySo,
    UnknownVariableType,
    IncorrectFunctionArguments(Identifier, Tokens),
    LineParseError(Tokens),
    FunctionArgumentsMustEndWithMkay,
    FunctionParseError,
    InvalidExpression(Tokens),
    YarnIsNotANumbar(Yarn),
    YarnIsNotANumbr(Yarn),
    YarnIsNotANumber(Yarn),
    CannotPerformMathsOnTroof,
    CannotPerformMathsOnNoob,
    DivisionByZero,
    NumberOverflow,
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
            AppError::TokenCannotBeExpression(token) => {
                write!(f, "Token {token} cannot be expression")
            }
            AppError::MissingExpression => {
                write!(f, "Missing expression")
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
            AppError::IncorrectFunctionArguments(called_func, tokens) => {
                write!(
                    f,
                    "Incorrect function arguments for {called_func}: {tokens}"
                )
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
            AppError::InvalidExpression(tokens) => {
                write!(f, "Invalid expression: {tokens}")
            }
            AppError::YarnIsNotANumbar(yarn) => {
                write!(f, "YARN \"{yarn}\" is not a NUMBAR")
            }
            AppError::YarnIsNotANumbr(yarn) => {
                write!(f, "YARN \"{yarn}\" is not a NUMBR")
            }
            AppError::YarnIsNotANumber(yarn) => {
                write!(f, "YARN \"{yarn}\" is not a number")
            }
            AppError::CannotPerformMathsOnTroof => {
                write!(f, "Cannot perform maths expressions with TROOF")
            }
            AppError::CannotPerformMathsOnNoob => {
                write!(f, "Cannot perform maths expressions with NOOB")
            }
            AppError::DivisionByZero => {
                write!(f, "Division by zero")
            }
            AppError::NumberOverflow => {
                write!(f, "Number overflow")
            }
        }
    }
}
