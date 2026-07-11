use std::fmt::Display;
use std::str::FromStr;

use bigdecimal::{BigDecimal, Zero};

use crate::app_error::AppError;
use crate::types::Value;

// This is used for internal logic of maths operations. When parsing a yarn, we want to check if it
// contains a Numbar first and a Numbr second. So this Number enum holds both possibilies and allows
// generic maths logic.
pub enum Number {
    Int(i64),
    Decimal(BigDecimal),
}

impl Number {
    pub fn into_value(self) -> Value {
        match self {
            Number::Int(i) => Value::Numbr(Numbr::new(i)),
            Number::Decimal(f) => Value::Numbar(Numbar::new(f)),
        }
    }
}

impl From<&Numbar> for Number {
    fn from(value: &Numbar) -> Self {
        Number::Decimal(value.value.clone())
    }
}

impl From<&Numbr> for Number {
    fn from(value: &Numbr) -> Self {
        Number::Int(value.value)
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Numbar {
    value: BigDecimal,
}

impl Numbar {
    pub fn new(v: BigDecimal) -> Self {
        Numbar { value: v }
    }

    pub fn value(&self) -> &BigDecimal {
        &self.value
    }

    fn truncated(&self) -> BigDecimal {
        let multiplier = BigDecimal::from(100);

        (&self.value * &multiplier).with_scale(0) / multiplier
    }
}

impl Display for Numbar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}", self.truncated())
    }
}

impl TryFrom<Yarn> for Numbar {
    type Error = AppError;

    fn try_from(value: Yarn) -> Result<Self, Self::Error> {
        let num = BigDecimal::from_str(value.value.as_str())
            .map_err(|_| AppError::YarnIsNotANumbar(value))?;

        Ok(Numbar::new(num))
    }
}

impl PartialEq<Numbr> for Numbar {
    fn eq(&self, other: &Numbr) -> bool {
        self.value == other.value
    }
}

#[derive(Debug, Clone, Default)]
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

impl PartialEq for Numbr {
    fn eq(&self, other: &Numbr) -> bool {
        self.value == other.value
    }
}

impl PartialEq<Numbar> for Numbr {
    fn eq(&self, other: &Numbar) -> bool {
        &BigDecimal::from(self.value) == other.value()
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Yarn {
    value: String,
}

impl Yarn {
    pub fn new(v: String) -> Self {
        Self { value: v }
    }

    pub fn from_literal(v: String) -> Self {
        let v = if v.starts_with('"') && v.ends_with('"') && v.len() >= 2 {
            &v[1..v.len() - 1]
        } else {
            &v
        };

        let mut chars = v.chars().peekable();
        let mut result = String::new();

        while let Some(c) = chars.next() {
            if c == ':' {
                match chars.peek() {
                    Some(')') => {
                        chars.next();
                        result.push('\n');
                    }
                    Some('>') => {
                        chars.next();
                        result.push('\t');
                    }
                    Some('o') => {
                        chars.next();
                        result.push('\x07');
                    }
                    Some('"') => {
                        chars.next();
                        result.push('"');
                    }
                    Some(':') => {
                        chars.next();
                        result.push(':');
                    }
                    _ => {
                        result.push(':');
                    }
                }
            } else {
                result.push(c);
            }
        }
        // dbg!(&result);

        Yarn { value: result }
    }

    pub fn concat(&mut self, other: Yarn) {
        self.value.push_str(&other.value);
    }
}

impl Display for Yarn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&Numbar> for Yarn {
    fn from(value: &Numbar) -> Self {
        Yarn::new(value.to_string())
    }
}

impl From<&Numbr> for Yarn {
    fn from(value: &Numbr) -> Self {
        Yarn::new(value.to_string())
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

    pub fn value(&self) -> bool {
        self.value
    }

    pub fn flip_value(&mut self) {
        self.value = !self.value
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

impl From<Numbar> for Troof {
    fn from(value: Numbar) -> Self {
        if value.value == BigDecimal::zero() {
            Troof::new(false)
        } else {
            Troof::new(true)
        }
    }
}

impl From<Numbr> for Troof {
    fn from(value: Numbr) -> Self {
        if value.value == 0 {
            Troof::new(false)
        } else {
            Troof::new(true)
        }
    }
}

impl From<Yarn> for Troof {
    fn from(value: Yarn) -> Self {
        if value.value.is_empty() {
            return Troof::new(false);
        }

        // Try integer first
        let int: Result<Numbr, AppError> = value.clone().try_into();
        if let Ok(int) = int {
            return Troof::new(int.value != 0);
        }

        // Then float
        let float: Result<Numbar, AppError> = value.clone().try_into();
        if let Ok(float) = float {
            return Troof::new(float.value != BigDecimal::zero());
        }

        Troof::new(true)
    }
}
