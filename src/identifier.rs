use std::fmt;

use crate::{
    app_error::AppError,
    expression::Expr,
    keywords::Keyword,
    lexer::{Token, Tokens},
};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Identifier {
    name: String,
}

impl Identifier {
    pub fn new(name: String) -> Result<Self, AppError> {
        if name.is_empty() {
            return Err(AppError::InvalidIdentifier(name));
        }

        if Keyword::ALL.contains(&name.as_str()) {
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

#[derive(Debug, PartialEq, Clone)]
pub enum IdentifierExpr {
    Identifier(Identifier),
    Srs(Box<Expr>),
}

impl IdentifierExpr {
    pub fn parse(tokens: &[Token]) -> Result<(Self, usize), AppError> {
        match tokens {
            [Token::Keyword(Keyword::Srs), rest @ ..] => {
                let (expr, consumed) = Expr::parse(rest)?;

                Ok((IdentifierExpr::Srs(Box::new(expr)), consumed + 1))
            }

            [Token::Identifier(id), ..] => Ok((IdentifierExpr::Identifier(id.clone()), 1)),

            _ => Err(AppError::InvalidIdentifierExpr(Tokens(tokens.to_vec()))),
        }
    }
}

impl From<Identifier> for IdentifierExpr {
    fn from(value: Identifier) -> Self {
        Self::Identifier(value)
    }
}
