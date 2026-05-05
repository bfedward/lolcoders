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

#[derive(Debug, Clone, Default)]
pub struct Numbar {
    value: f64,
}

impl Numbar {
    pub fn new(v: f64) -> Self {
        Numbar { value: v }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

impl Display for Numbar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}", f64::trunc(self.value * 100.0) / 100.0)
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

impl PartialEq for Numbar {
    fn eq(&self, other: &Numbar) -> bool {
        self.value == other.value
    }
}

impl PartialEq<Numbr> for Numbar {
    fn eq(&self, other: &Numbr) -> bool {
        self.value == other.value as f64
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
        self.value as f64 == other.value
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Yarn {
    value: String,
}

impl Yarn {
    pub fn new(v: String) -> Self {
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
        if value.value == 0.0 {
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
            return Troof::new(float.value != 0.0);
        }

        Troof::new(false)
    }
}
