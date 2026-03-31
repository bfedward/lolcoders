use std::fmt::{Display};

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

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Numbr {
    value: i64,
}

impl Numbr {
    pub fn new(v: i64) -> Self {
        Numbr { value: v }
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
            false => write!(f, "FAIL")
        }
    }
}
