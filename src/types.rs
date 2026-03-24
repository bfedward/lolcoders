use std::fmt;

use crate::{app_error::AppError, lexer::Token};

const KEYWORDS: &[&str] = &[
    "HAI", "KTHXBYE", "VISIBLE", "I", "HAS", "A", "HOW", "IZ", "IF", "U", "SAY", "SO", "YR", "AN",
    "MKAY", "WIN", "FAIL",
];

#[derive(Debug, Clone)]
pub enum Value {
    Numbar(f64),
    Numbr(i64),
    Yarn(String),
    Troof(bool),
    Noob,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Numbar(f64),
    Numbr(i64),
    Yarn(String),
    Troof(bool),
    Variable(Identifier),
    Noob,
}

impl TryFrom<&Token> for Expr {
    type Error = AppError;

    fn try_from(token: &Token) -> Result<Self, AppError> {
        match token {
            Token::Numbar(n) => Ok(Expr::Numbar(*n)),
            Token::Numbr(n) => Ok(Expr::Numbr(*n)),
            Token::Yarn(s) => Ok(Expr::Yarn(s.clone())),
            Token::Troof(b) => Ok(Expr::Troof(*b)),
            Token::Noob => Ok(Expr::Noob),
            Token::Identifier(ident) => Ok(Expr::Variable(ident.clone())),
            Token::Keyword(_) => Err(AppError::TokenCannotBeExpression),
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
    KThxBye,
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Identifier {
    name: String,
}

impl Identifier {
    pub fn new(name: String) -> Result<Self, AppError> {
        if name.is_empty() {
            return Err(AppError::InvalidIdentifier(name));
        }

        if KEYWORDS.contains(&name.as_str()) {
            return Err(AppError::InvalidIdentifier(name));
        }

        let mut chars = name.chars();

        match chars.next() {
            Some(c) if c.is_ascii_alphabetic() => {}
            _ => return Err(AppError::InvalidIdentifier(name)),
        }

        if !chars.all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err(AppError::InvalidIdentifier(name));
        }

        Ok(Self { name })
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
