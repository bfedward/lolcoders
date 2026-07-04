use std::fmt::{self};

use crate::{
    lexer::{Token, Tokens},
    types::{identifier::Identifier, primitive::Yarn},
};

pub enum AppError {
    HaiMustBeFirstLine,
    MustGiveVersionNumberInHaiLine,
    KThxByeMustBeLastLine,
    VariableDoesNotExist(Identifier),
    FunctionDoesNotExist(Identifier),
    NotEnoughArgsForFunction,
    InvalidIdentifier(String),
    InvalidIdentifierExpr(Tokens),
    TokenCannotBeExpression(Token),
    MissingExpression(Tokens),
    CouldNotGetCurrentVariableScope,
    CannotReturnFromFunctionOutsideFunction,
    FunctionMustEndIfUSaySo,
    IncorrectFunctionArguments(Identifier, Tokens),
    LineParseError(Tokens),
    FunctionArgumentsMustEndWithMkay,
    FunctionParseError,
    InvalidExpression(Tokens),
    YarnIsNotANumbar(Yarn),
    YarnIsNotANumbr(Yarn),
    YarnIsNotANumber(Yarn),
    CannotPerformMathsOnNoob,
    DivisionByZero,
    NumberOverflow,
    TroofExpressionMustEndWithMkay,
    TroofExpressionHasInvalidNumberOfArguments,
    ObtwMustStartLine,
    TldrMustEndLine,
    TldrMustBeAfterObtw,
    QuestionMarkIsNotAnExpression,
    ExclamationMarkIsNotAnExpression,
    CannotVisibleANoob,
    CannotVisibleATroof,
    VisibleMustHaveAnArg,
    BadGimmeh,
    CannotRedeclareVariable(Identifier),
    NoValueInItVariable,
    ORlyParseError,
    ORlyBlockMustHaveYaRly,
    ORlyBlockMustEndOic,
    ORlyBlockCanOnlyHaveOneNoWai,
    ORlyNoWaiBlockMustBeLast,
    CanHasMustEndQuestionMark,
    UnexpectedEOF,
    InvalidSyntax(Tokens),
    UnclosedInterpolation,
    InvalidUnicodeCodepoint,
    BadMaekCastType,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::HaiMustBeFirstLine => write!(f, "Must start with HAI"),
            AppError::MustGiveVersionNumberInHaiLine => {
                write!(f, "Must give version number in HAI line")
            }
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
            AppError::InvalidIdentifierExpr(tokens) => {
                write!(f, "Invalid identifier expression: {tokens}")
            }
            AppError::TokenCannotBeExpression(token) => {
                write!(f, "Token {token} cannot be expression")
            }
            AppError::MissingExpression(tokens) => {
                write!(f, "Missing expression: {tokens}")
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
            AppError::CannotPerformMathsOnNoob => {
                write!(f, "Cannot perform maths expressions with NOOB")
            }
            AppError::DivisionByZero => {
                write!(f, "Division by zero")
            }
            AppError::NumberOverflow => {
                write!(f, "Number overflow")
            }
            AppError::TroofExpressionMustEndWithMkay => {
                write!(f, "Troof expression must end with MKAY")
            }
            AppError::TroofExpressionHasInvalidNumberOfArguments => {
                write!(f, "Troof expression has invalid number of arguments")
            }
            AppError::ObtwMustStartLine => {
                write!(f, "OBTW must start a line")
            }
            AppError::TldrMustEndLine => {
                write!(f, "TLDR must end a line")
            }
            AppError::TldrMustBeAfterObtw => {
                write!(f, "TLDR must be after OBTW")
            }
            AppError::QuestionMarkIsNotAnExpression => {
                write!(f, "Question mark is not an expression")
            }
            AppError::ExclamationMarkIsNotAnExpression => {
                write!(f, "Exclamation mark is not an expression")
            }
            AppError::CannotVisibleANoob => {
                write!(f, "Cannot VISIBLE a NOOB")
            }
            AppError::CannotVisibleATroof => {
                write!(f, "Cannot VISIBLE a TROOF")
            }
            AppError::VisibleMustHaveAnArg => {
                write!(f, "VISIBLE must have an arg")
            }
            AppError::BadGimmeh => {
                write!(f, "Bad GIMMEH")
            }
            AppError::CannotRedeclareVariable(var) => {
                write!(f, "Cannot redeclare variable: {var} ")
            }
            AppError::NoValueInItVariable => {
                write!(f, "No value in IT")
            }
            AppError::ORlyParseError => {
                write!(f, "O RLY parse error")
            }
            AppError::ORlyBlockMustHaveYaRly => {
                write!(f, "O RLY must have YA RLY block")
            }
            AppError::ORlyBlockMustEndOic => {
                write!(f, "O RLY must end with OIC")
            }
            AppError::ORlyBlockCanOnlyHaveOneNoWai => {
                write!(f, "O RLY block can only have one NO WAI block")
            }
            AppError::ORlyNoWaiBlockMustBeLast => {
                write!(f, "NO WAI block must appear last inside O RLY")
            }
            AppError::CanHasMustEndQuestionMark => {
                write!(f, "CAN HAS must end with a question mark")
            }
            AppError::UnexpectedEOF => {
                write!(f, "Unexpected EOF")
            }
            AppError::InvalidSyntax(tokens) => {
                write!(f, "Invalid syntax: {tokens}")
            }
            AppError::UnclosedInterpolation => {
                write!(f, "Unclosed Interpolation")
            }
            AppError::InvalidUnicodeCodepoint => {
                write!(f, "Invalid Unicode codepoint")
            }
            AppError::BadMaekCastType => {
                write!(f, "Token cannot be a MAEK cast type")
            }
        }
    }
}
