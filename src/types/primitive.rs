use std::str::FromStr;
use std::{fmt::Display, ops::Add};

use crate::app_error::AppError;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Numbar {
    value: f64,
}

impl Numbar {
    pub fn new(v: f64) -> Self {
        Numbar { value: v }
    }
}
impl Add<Numbar> for Numbar {
    type Output = Numbar;

    fn add(self, rhs: Numbar) -> Self::Output {
        Numbar::new(self.value + rhs.value)
    }
}

impl Add<Numbr> for Numbar {
    type Output = Numbar;

    fn add(self, rhs: Numbr) -> Self::Output {
        Numbar::new(self.value + rhs.value as f64)
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

impl Add<Numbr> for Numbr {
    type Output = Numbr;

    fn add(self, rhs: Numbr) -> Self::Output {
        Numbr::new(self.value + rhs.value)
    }
}

impl Add<Numbar> for Numbr {
    type Output = Numbar;

    fn add(self, rhs: Numbar) -> Self::Output {
        Numbar::new(self.value as f64 + rhs.value)
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

impl Add for Yarn {
    type Output = Yarn;

    fn add(self, rhs: Yarn) -> Self::Output {
        Yarn::new(self.value + &rhs.value)
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
