use std::fmt::Display;
use std::str::FromStr;

use crate::app_error::AppError;
use crate::types::Value;

// This is used for internal logic of maths operations. When parsing a yarn, we want to check if it
// contains a Numbar first and a Numbr second. So this Number enum holds both possibilies and allows
// generic maths logic.
pub enum Number {
    Int(i64),
    Float(f64),
}

impl Number {
    pub fn into_value(self) -> Value {
        match self {
            Number::Int(i) => Value::Numbr(Numbr::new(i)),
            Number::Float(f) => Value::Numbar(Numbar::new(f)),
        }
    }
}

impl From<&Numbar> for Number {
    fn from(value: &Numbar) -> Self {
        Number::Float(value.value)
    }
}

impl From<&Numbr> for Number {
    fn from(value: &Numbr) -> Self {
        Number::Int(value.value)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Numbar {
    value: f64,
}

impl Numbar {
    pub fn new(v: f64) -> Self {
        Numbar { value: v }
    }
}

impl Display for Numbar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl TryFrom<Yarn> for Numbar {
    type Error = AppError;

    fn try_from(value: Yarn) -> Result<Self, Self::Error> {
        let num =
            f64::from_str(value.value.as_str()).map_err(|_| AppError::YarnIsNotANumbar(value))?;

        Ok(Numbar::new(num))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Numbr {
    value: i64,
}

impl Numbr {
    pub fn new(v: i64) -> Self {
        Numbr { value: v }
    }
}

impl TryFrom<Yarn> for Numbr {
    type Error = AppError;

    fn try_from(value: Yarn) -> Result<Self, Self::Error> {
        let num = i64::from_str(&value.value).map_err(|_| AppError::YarnIsNotANumbr(value))?;

        Ok(Numbr::new(num))
    }
}

impl Display for Numbr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Yarn {
    value: String,
}

impl Yarn {
    pub fn new(v: String) -> Self {
        Yarn { value: v }
    }
}

impl Display for Yarn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Troof {
    value: bool,
}

impl Troof {
    pub fn new(v: bool) -> Self {
        Troof { value: v }
    }
}

impl Display for Troof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.value {
            true => write!(f, "WIN"),
            false => write!(f, "FAIL"),
        }
    }
}
