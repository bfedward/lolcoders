use std::fmt;

use crate::app_error::AppError;

const KEYWORDS: &[&str] = &[
    "HAI", "KTHXBYE", "VISIBLE", "I", "HAS", "A", "HOW", "IZ", "IF", "U", "SAY", "SO", "YR", "AN",
    "MKAY", "WIN", "FAIL", "YARN", "TROOF", "NUMBR", "NUMBAR", "NOOB", "FOUND", "GTFO", "R", "SUM",
    "OF",
];

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Identifier {
    name: String,
}

impl Identifier {
    pub fn new(name: String) -> Result<Self, AppError> {
        if name.is_empty() {
            return Err(AppError::InvalidIdentifier(name));
        }

        if KEYWORDS.contains(&name.as_str()) {
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
