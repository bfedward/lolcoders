use std::fmt;
use std::ops::Add;

use crate::{
    app_error::AppError,
    expression::Expr,
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

impl Add for Value {
    type Output = Result<Value, AppError>;

    fn add(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::Numbr(a), Value::Numbr(b)) => Ok(Value::Numbr(a + b)),

            (Value::Numbar(a), Value::Numbar(b)) => Ok(Value::Numbar(a + b)),

            // mixed numeric
            (Value::Numbr(a), Value::Numbar(b)) => Ok(Value::Numbar(b + a)),

            (Value::Numbar(a), Value::Numbr(b)) => Ok(Value::Numbar(a + b)),

            (Value::Yarn(a), Value::Yarn(b)) => {
                let a_numbar: Result<Numbar, AppError> = a.clone().try_into();
                let b_numbar: Result<Numbar, AppError> = b.clone().try_into();

                match (a_numbar, b_numbar) {
                    (Ok(a_numbar), Ok(b_numbar)) => Ok(Value::Numbar(a_numbar + b_numbar)),
                    (Ok(a_numbar), Err(AppError::YarnIsNotANumbar(_))) => {
                        let b_numbr: Numbr = b.try_into()?;
                        Ok(Value::Numbar(a_numbar + b_numbr))
                    }
                    (Err(AppError::YarnIsNotANumbar(_)), Ok(b_numbar)) => {
                        let a_numbr: Numbr = a.try_into()?;
                        Ok(Value::Numbar(a_numbr + b_numbar))
                    }
                    _ => Err(AppError::CannotSumYarns(a, b)),
                }
            }
            (Value::Numbar(numbar), Value::Yarn(yarn))
            | (Value::Yarn(yarn), Value::Numbar(numbar)) => {
                let yarn_numbar: Result<Numbar, AppError> = yarn.clone().try_into();

                match yarn_numbar {
                    Ok(yarn_numbar) => Ok(Value::Numbar(numbar + yarn_numbar)),
                    Err(AppError::YarnIsNotANumbar(_)) => {
                        let yarn_numbr: Numbr = yarn.try_into()?;
                        Ok(Value::Numbar(numbar + yarn_numbr))
                    }
                    _ => Err(AppError::CannotSumYarn(yarn)),
                }
            }
            (Value::Numbr(numbr), Value::Yarn(yarn)) | (Value::Yarn(yarn), Value::Numbr(numbr)) => {
                let yarn_numbr: Result<Numbr, AppError> = yarn.clone().try_into();

                match yarn_numbr {
                    Ok(yarn_numbr) => Ok(Value::Numbr(numbr + yarn_numbr)),
                    _ => Err(AppError::CannotSumYarn(yarn)),
                }
            }
            (Value::Troof(_), _) => Err(AppError::CannotPerformMathsOnTroof),
            (_, Value::Troof(_)) => Err(AppError::CannotPerformMathsOnTroof),
            (Value::Noob, _) => Err(AppError::CannotPerformMathsOnNoob),
            (_, Value::Noob) => Err(AppError::CannotPerformMathsOnTroof),
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
