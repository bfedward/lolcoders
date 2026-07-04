use std::fmt;

use crate::{
    app_error::AppError,
    expression::{BoolOp, ComparisonExpr, ComparisonOp, MathOp, MathsExpr},
    types::primitive::{Numbar, Number, Numbr, Troof, Yarn},
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

impl Value {
    pub fn as_number(&self) -> Result<Number, AppError> {
        match self {
            Value::Numbr(n) => Ok(Number::from(n)),

            Value::Numbar(n) => Ok(Number::from(n)),

            Value::Yarn(y) => {
                // Try integer first
                let int: Result<Numbr, AppError> = y.clone().try_into();
                if let Ok(int) = int {
                    return Ok(Number::from(&int));
                }

                // Then float
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

pub fn eval_comparison_expr(
    op: &ComparisonExpr,
    left: Value,
    right: Value,
) -> Result<Value, AppError> {
    let comp = match op.op {
        ComparisonOp::BothSaem => left == right,
        ComparisonOp::Diffrint => left != right,
    };

    Ok(Value::Troof(Troof::new(comp)))
}

pub fn eval_maths_expr(op: &MathsExpr, left: Value, right: Value) -> Result<Value, AppError> {
    match op.op {
        MathOp::Sum => apply_numeric_op(left, right, i64::checked_add, |a, b| a + b),

        MathOp::Diff => apply_numeric_op(left, right, i64::checked_sub, |a, b| a - b),

        MathOp::Produkt => apply_numeric_op(left, right, i64::checked_mul, |a, b| a * b),
        MathOp::Quoshunt => {
            let l = left.as_number()?;
            let r = right.as_number()?;

            if check_zero(&r) {
                return Err(AppError::DivisionByZero);
            }

            match (l, r) {
                (Number::Int(_), Number::Int(0)) => Err(AppError::DivisionByZero),

                (Number::Int(a), Number::Int(b)) => {
                    Ok(Value::Numbar(Numbar::new(a as f64 / b as f64)))
                }

                (Number::Int(a), Number::Float(b)) => Ok(Value::Numbar(Numbar::new(a as f64 / b))),

                (Number::Float(a), Number::Int(b)) => Ok(Value::Numbar(Numbar::new(a / b as f64))),

                (Number::Float(a), Number::Float(b)) => Ok(Value::Numbar(Numbar::new(a / b))),
            }
        }

        MathOp::Mod => apply_numeric_op(left, right, |a, b| Some(a % b), |a, b| a % b),

        MathOp::Biggr => apply_numeric_op(left, right, |a, b| Some(a.max(b)), f64::max),

        MathOp::Smallr => apply_numeric_op(left, right, |a, b| Some(a.min(b)), f64::min),
    }
}

fn apply_numeric_op<X, Y>(
    left: Value,
    right: Value,
    int_op: X,
    float_op: Y,
) -> Result<Value, AppError>
where
    X: Fn(i64, i64) -> Option<i64>,
    Y: Fn(f64, f64) -> f64,
{
    let l = left.as_number()?;
    let r = right.as_number()?;

    let result = match (l, r) {
        (Number::Int(a), Number::Int(b)) => {
            let res = int_op(a, b).ok_or(AppError::NumberOverflow)?;
            Number::Int(res)
        }

        (Number::Int(a), Number::Float(b)) => {
            Number::Float(check_float_overflow(float_op(a as f64, b))?)
        }

        (Number::Float(a), Number::Int(b)) => {
            Number::Float(check_float_overflow(float_op(a, b as f64))?)
        }

        (Number::Float(a), Number::Float(b)) => {
            Number::Float(check_float_overflow(float_op(a, b))?)
        }
    };

    Ok(result.into_value())
}

fn check_zero(n: &Number) -> bool {
    match n {
        Number::Int(0) => true,
        Number::Float(f) => *f == 0.0,
        _ => false,
    }
}

fn check_float_overflow(f: f64) -> Result<f64, AppError> {
    if f.is_infinite() {
        return Err(AppError::NumberOverflow);
    }
    if f.is_nan() {
        return Err(AppError::NumberOverflow);
    }
    Ok(f)
}

pub fn eval_bool_expr(op: &BoolOp, exprs: Vec<Value>) -> Result<Value, AppError> {
    let expr_count = exprs.len();

    match op {
        BoolOp::Both | BoolOp::Either | BoolOp::Won => {
            if expr_count != 2 {
                return Err(AppError::TroofExpressionHasInvalidNumberOfArguments);
            }
        }
        BoolOp::Not => {
            if expr_count != 1 {
                return Err(AppError::TroofExpressionHasInvalidNumberOfArguments);
            }
        }
        BoolOp::All | BoolOp::Any => {
            if expr_count == 0 {
                return Err(AppError::TroofExpressionHasInvalidNumberOfArguments);
            }
        }
    }

    let res = match op {
        BoolOp::Both | BoolOp::All | BoolOp::Any => {
            if exprs.iter().all(|t| t.as_troof().value()) {
                Troof::new(true)
            } else {
                Troof::new(false)
            }
        }
        BoolOp::Either => {
            if exprs.iter().any(|t| t.as_troof().value()) {
                Troof::new(true)
            } else {
                Troof::new(false)
            }
        }
        BoolOp::Won => {
            let how_many = exprs.iter().fold(0, |mut acc, x| {
                if x.as_troof().value() {
                    acc += 1;
                }
                acc
            });

            Troof::new(how_many == 1)
        }
        BoolOp::Not => {
            if exprs.iter().all(|t| t.as_troof().value()) {
                Troof::new(false)
            } else {
                Troof::new(true)
            }
        }
    };

    Ok(Value::Troof(res))
}
