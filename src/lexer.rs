use std::fmt;

use crate::{app_error::AppError, keywords::Keyword, types::identifier::Identifier};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Identifier(Identifier),
    Yarn(String),
    Numbr(i64),
    Numbar(f64),
    Troof(bool),
    Noob,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Keyword(k) => write!(f, "{k}"),
            Token::Identifier(id) => write!(f, "{id}"),
            Token::Yarn(s) => write!(f, "\"{s}\""),
            Token::Numbr(n) => write!(f, "{n}"),
            Token::Numbar(n) => write!(f, "{n}"),
            Token::Troof(b) => write!(f, "{}", if *b { "WIN" } else { "FAIL" }),
            Token::Noob => write!(f, "NOOB"),
        }
    }
}

pub struct Tokens(pub Vec<Token>);

impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let strings: Vec<String> = self.0.iter().map(|t| t.to_string()).collect();

        write!(f, "{}", strings.join(" "))
    }
}

pub fn tokenize_line(line: &str) -> Result<Vec<Token>, AppError> {
    let raw_tokens = split_line(line);

    let tokens = raw_tokens
        .into_iter()
        .map(classify_token)
        .collect::<Result<Vec<Token>, AppError>>()?;

    Ok(tokens)
}

fn split_line(line: &str) -> Vec<String> {
    let mut raw_tokens = Vec::new();
    let mut current = String::new();
    let mut in_string = false;

    for c in line.chars() {
        match c {
            '"' => {
                in_string = !in_string;
                current.push(c);
            }
            ' ' if !in_string => {
                if !current.is_empty() {
                    raw_tokens.push(current.clone());
                    current.clear();
                }
            }
            _ => current.push(c),
        }
    }

    if !current.is_empty() {
        raw_tokens.push(current);
    }

    raw_tokens
}

fn classify_token(word: String) -> Result<Token, AppError> {
    if word.starts_with('"') && word.ends_with('"') {
        return Ok(Token::Yarn(word.trim_matches('"').to_string()));
    }

    if word == "WIN" {
        return Ok(Token::Troof(true));
    }
    if word == "FAIL" {
        return Ok(Token::Troof(false));
    }

    if word == "NOOB" {
        return Ok(Token::Noob);
    }

    if let Ok(n) = word.parse::<i64>() {
        return Ok(Token::Numbr(n));
    }

    if let Ok(n) = word.parse::<f64>() {
        return Ok(Token::Numbar(n));
    }

    if let Some(keyword) = Keyword::from_str(&word) {
        return Ok(Token::Keyword(keyword));
    }

    Ok(Token::Identifier(Identifier::new(word)?))
}
