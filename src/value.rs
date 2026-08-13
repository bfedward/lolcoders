use std::fmt;

use crate::{
    app_error::AppError,
    lexer::{Token, Tokens},
    primitive::{Numbar, Number, Numbr, Troof, Yarn},
};

#[derive(Debug, Clone)]
pub enum Value {
    Numbar(Numbar),
    Numbr(Numbr),
    Yarn(Yarn),
    Troof(Troof),
    Noob,
}

impl Value {
    pub fn parse(tokens: &[Token]) -> Result<(Self, usize), AppError> {
        match tokens.first() {
            Some(value) => match value {
                Token::Yarn(yarn) => Ok((Value::Yarn(yarn.clone()), 1)),
                Token::Numbar(numbar) => Ok((Value::Numbar(numbar.clone()), 1)),
                Token::Numbr(numbr) => Ok((Value::Numbr(numbr.clone()), 1)),
                Token::Troof(troof) => Ok((Value::Troof(troof.clone()), 1)),
                _ => Err(AppError::TokenCannotBeValue(value.clone())),
            },
            None => Err(AppError::MissingValue(Tokens(tokens.to_vec()))),
        }
    }

    pub fn as_number(&self) -> Result<Number, AppError> {
        match self {
            Value::Numbr(n) => Ok(Number::from(n)),

            Value::Numbar(n) => Ok(Number::from(n)),

            Value::Yarn(y) => {
                let int: Result<Numbr, AppError> = y.clone().try_into();
                if let Ok(int) = int {
                    return Ok(Number::from(&int));
                }

                let float: Result<Numbar, AppError> = y.clone().try_into();
                if let Ok(float) = float {
                    return Ok(Number::from(&float));
                }

                Err(AppError::YarnIsNotANumber(y.clone()))
            }

            Value::Troof(t) => match t.value() {
                true => Ok(Number::Int(1)),
                false => Ok(Number::Int(0)),
            },
            Value::Noob => Err(AppError::CannotPerformMathsOnNoob),
        }
    }

    pub fn as_troof(&self) -> Troof {
        match self {
            Value::Numbar(numbar) => numbar.clone().into(),
            Value::Numbr(numbr) => numbr.clone().into(),
            Value::Yarn(yarn) => yarn.clone().into(),
            Value::Troof(troof) => troof.clone(),
            Value::Noob => Troof::new(false),
        }
    }

    pub fn as_yarn(&self) -> Result<Yarn, AppError> {
        match self {
            Value::Numbar(numbar) => Ok(numbar.into()),
            Value::Numbr(numbr) => Ok(numbr.into()),
            Value::Yarn(yarn) => Ok(yarn.clone()),
            Value::Troof(_) => Err(AppError::CannotVisibleATroof),
            Value::Noob => Err(AppError::CannotVisibleANoob),
        }
    }

    pub fn strict_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Numbar(x), Value::Numbar(y)) => x == y,
            (Value::Numbr(x), Value::Numbr(y)) => x == y,
            (Value::Yarn(x), Value::Yarn(y)) => x == y,
            (Value::Troof(x), Value::Troof(y)) => x == y,
            (Value::Noob, Value::Noob) => true,
            _ => false,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Numbar(x), Value::Numbar(y)) => x == y,
            (Value::Numbr(x), Value::Numbr(y)) => x == y,
            (Value::Numbar(x), Value::Numbr(y)) => x == y,
            (Value::Numbr(x), Value::Numbar(y)) => x == y,
            (Value::Yarn(x), Value::Yarn(y)) => x == y,
            (Value::Troof(x), Value::Troof(y)) => x == y,
            (Value::Noob, Value::Noob) => true,
            _ => false,
        }
    }
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
