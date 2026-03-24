use crate::{app_error::AppError, types::Identifier};

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Hai,
    KThxBye,
    Visible,
    I,
    Itz,
    Has,
    A,
    How,
    Iz,
    If,
    U,
    Say,
    So,
    Yr,
    An,
    Mkay,
    Troof,
}

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

pub fn tokenize_line(line: &str) -> Result<Vec<Token>, AppError> {
    let raw_tokens = split_line(line);

    let tokens = raw_tokens
        .into_iter()
        .map(|word| classify_token(word))
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

    if let Some(keyword) = match_keyword(&word) {
        return Ok(Token::Keyword(keyword));
    }

    Ok(Token::Identifier(Identifier::new(word)?))
}

fn match_keyword(word: &str) -> Option<Keyword> {
    match word {
        "HAI" => Some(Keyword::Hai),
        "KTHXBYE" => Some(Keyword::KThxBye),
        "VISIBLE" => Some(Keyword::Visible),
        "I" => Some(Keyword::I),
        "ITZ" => Some(Keyword::Itz),
        "HAS" => Some(Keyword::Has),
        "A" => Some(Keyword::A),
        "HOW" => Some(Keyword::How),
        "IZ" => Some(Keyword::Iz),
        "IF" => Some(Keyword::If),
        "U" => Some(Keyword::U),
        "SAY" => Some(Keyword::Say),
        "SO" => Some(Keyword::So),
        "YR" => Some(Keyword::Yr),
        "AN" => Some(Keyword::An),
        "MKAY" => Some(Keyword::Mkay),
        "TROOF" => Some(Keyword::Troof),
        _ => None,
    }
}
