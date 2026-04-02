use std::fmt;

use crate::{
    app_error::AppError,
    lexer::Token,
    types::{
        identifier::Identifier,
        primitive::{Numbar, Numbr, Troof, Yarn},
    },
};

pub mod identifier;
pub mod primitive;

#[derive(Debug, Clone)]
pub enum Value {
    Numbar(Numbar),
    Numbr(Numbr),
    Yarn(Yarn),
    Troof(Troof),
    Noob,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Numbar(n) => write!(f, "{n}"),
            Value::Numbr(n) => write!(f, "{n}"),
            Value::Yarn(s) => write!(f, "{s}"),
            Value::Troof(b) => write!(f, "{b}"),
            Value::Noob => write!(f, "NOOB"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Numbar(Numbar),
    Numbr(Numbr),
    Yarn(Yarn),
    Troof(Troof),
    Variable(Identifier),
    Noob,
}

impl TryFrom<&Token> for Expr {
    type Error = AppError;

    fn try_from(token: &Token) -> Result<Self, AppError> {
        match token {
            Token::Numbar(n) => Ok(Expr::Numbar(Numbar::new(*n))),
            Token::Numbr(n) => Ok(Expr::Numbr(Numbr::new(*n))),
            Token::Yarn(s) => Ok(Expr::Yarn(Yarn::new(s.clone()))),
            Token::Troof(b) => Ok(Expr::Troof(Troof::new(*b))),
            Token::Noob => Ok(Expr::Noob),
            Token::Identifier(ident) => Ok(Expr::Variable(ident.clone())),
            Token::Keyword(_) => Err(AppError::TokenCannotBeExpression),
        }
    }
}

impl TryFrom<&[Token]> for Expr {
    type Error = AppError;

    fn try_from(tokens: &[Token]) -> Result<Self, Self::Error> {
        match tokens {
            [single] => Expr::try_from(single),

            [] => Err(AppError::MissingExpression),

            _ => Err(AppError::UnexpectedTokensInExpression),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Hai(Option<f64>),
    Visible(Expr),
    IHasA(Identifier, Expr),
    HowIzI(Identifier, Vec<Identifier>, Vec<Statement>),
    IIz(Identifier, Vec<Expr>),
    VarRIIzFunc(Identifier, Identifier, Vec<Expr>),
    FoundYr(Expr),
    Gtfo,
    KThxBye,
}
