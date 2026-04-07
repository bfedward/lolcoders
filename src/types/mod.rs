use std::fmt;
use std::ops::Add;

use crate::{
    app_error::AppError,
    keywords::Keyword,
    lexer::{Token, Tokens},
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

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Numbar(Numbar),
    Numbr(Numbr),
    Yarn(Yarn),
    Troof(Troof),
    Variable(Identifier),
    Noob,
    Sum(Box<Expr>, Box<Expr>),
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
            Token::Keyword(_) => Err(AppError::TokenCannotBeExpression(token.clone())),
        }
    }
}

impl Expr {
    pub fn parse(tokens: &[Token]) -> Result<(Self, usize), AppError> {
        match tokens.first() {
            Some(Token::Keyword(Keyword::Sum)) => {
                // Check that OF comes after SUM
                match tokens.get(1) {
                    Some(Token::Keyword(Keyword::Of)) => {}
                    _ => return Err(AppError::InvalidExpression(Tokens(tokens.to_vec()))),
                }

                // Parse expression after "SUM OF"
                // We have &tokens[2..] here because SUM + OF = 2 tokens
                // We don't know what comes after OF! Could be a literal, e.g. SUM OF 1 AN 2
                // Or could be a nested expression like SUM OF SUM OF 1 AN 2 AN 4
                //
                // consumed_left is the number of tokens consumed from tokens[2..]
                // starting immediately after "SUM OF".
                //
                // e.g. consumed_left would be 1 for a literal or 5 for
                // a nested SUM with two literals.
                let (left, consumed_left) = Expr::parse(&tokens[2..])?;

                // SUM + OF = 2 tokens
                // consumed_left = the number of tokens that were consumed in parsing
                // the left side of the SUM.
                // We expect AN between the left and right sides of the SUM.
                match tokens.get(2 + consumed_left) {
                    Some(Token::Keyword(Keyword::An)) => {}
                    _ => return Err(AppError::InvalidExpression(Tokens(tokens.to_vec()))),
                }

                // SUM + OF + AN = 3 tokens
                // consumed_left = the number of tokens that were consumed in parsing
                // the left side of the SUM.
                //
                // consumed_right is the number of tokens that are consumed to parse the right
                // side of the SUM, which could be a literal or another nexted expression.
                let (right, consumed_right) = Expr::parse(&tokens[3 + consumed_left..])?;

                // We have to use Boxes because Rust needs to know the size of everything on the stack.
                // We use Box because Expr is recursive.
                // Without Box, Expr::Sum(Expr, Expr) would have infinite size at compile time
                // because Expr would contain itself directly.
                // Box gives us a fixed-size pointer on the stack with the actual data on the heap.
                //
                // SUM + OF + AN = 3 tokens
                Ok((
                    Expr::Sum(Box::new(left), Box::new(right)),
                    3 + consumed_left + consumed_right,
                ))
            }

            // everything except SUM is just converting 1 Token to one Expr.
            Some(token) => Ok((Expr::try_from(token)?, 1)),

            None => Err(AppError::MissingExpression),
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
