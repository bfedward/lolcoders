use std::{fmt, str::FromStr};

use bigdecimal::BigDecimal;

use crate::{
    app_error::AppError,
    keywords::Keyword,
    types::{
        identifier::Identifier,
        primitive::{Numbar, Numbr, Troof, Yarn},
    },
};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Identifier(Identifier),
    Yarn(Yarn),
    Numbr(Numbr),
    Numbar(Numbar),
    Troof(Troof),
    QuestionMark,
    ExclamationMark,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Keyword(Keyword::Noob) => write!(f, "NOOB"),
            Token::Keyword(k) => write!(f, "{k}"),
            Token::Identifier(id) => write!(f, "{id}"),
            Token::Yarn(s) => write!(f, "\"{s}\""),
            Token::Numbr(n) => write!(f, "{n}"),
            Token::Numbar(n) => write!(f, "{n}"),
            Token::Troof(b) => write!(f, "{}", if b.value() { "WIN" } else { "FAIL" }),
            Token::QuestionMark => write!(f, "?"),
            Token::ExclamationMark => write!(f, "!"),
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

pub fn normalise_source(source: String) -> Result<String, AppError> {
    let mut result = String::new();
    let mut current_line = String::new();

    let mut in_string = false;
    let mut in_btw_comment = false;
    let mut in_obtw_comment = false;

    let source = source.replace("\r\n", "\n").replace('\r', "\n");

    let mut chars = source.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' => {
                if !in_btw_comment && !in_obtw_comment {
                    in_string = !in_string;
                    current_line.push(c);
                }
            }
            '\n' if !in_string && !in_btw_comment && !in_obtw_comment => {
                if current_line.ends_with("...") && !in_btw_comment {
                    current_line.truncate(current_line.len() - 3);
                    current_line.push(' ');
                } else {
                    in_btw_comment = false;
                    result.push_str(&current_line);
                    result.push(c);
                    current_line.clear();
                }
            }
            '\n' if in_btw_comment && !in_obtw_comment => {
                in_btw_comment = false;
                result.push(c);
            }
            ',' if !in_string && !in_btw_comment && !in_obtw_comment => {
                current_line.push('\n');
                result.push_str(&current_line);
                current_line.clear();
            }
            '\t' if !in_string && !in_btw_comment && !in_obtw_comment => {
                current_line.push(' ');
            }
            // BTW
            'B' if !in_string && !in_btw_comment && !in_obtw_comment => {
                if chars.peek() == Some(&'T') {
                    let mut clone = chars.clone();
                    clone.next(); // T
                    if clone.peek() == Some(&'W') {
                        chars.next();
                        chars.next();
                        result.push_str(&current_line);
                        current_line.clear();
                        in_btw_comment = true;
                    } else {
                        current_line.push(c);
                    }
                } else {
                    current_line.push(c);
                }
            }
            // OBTW
            'O' if !in_string && !in_btw_comment && !in_obtw_comment => {
                if chars.peek() == Some(&'B') {
                    let mut clone = chars.clone();
                    clone.next(); // B
                    if clone.peek() == Some(&'T') {
                        clone.next(); // T
                        if clone.peek() == Some(&'W') {
                            chars.next(); // W
                            chars.next();
                            chars.next();
                            current_line.push_str("OBTW");
                            in_obtw_comment = true;
                        } else {
                            current_line.push(c);
                        }
                    } else {
                        current_line.push(c);
                    }
                } else {
                    current_line.push(c);
                }
            }
            // TLDR
            'T' if !in_string && !in_btw_comment && in_obtw_comment => {
                if chars.peek() == Some(&'L') {
                    let mut clone = chars.clone();
                    clone.next(); // L
                    if clone.peek() == Some(&'D') {
                        clone.next(); // D
                        if clone.peek() == Some(&'R') {
                            chars.next(); // R
                            chars.next();
                            chars.next();
                            current_line.push_str("TLDR");
                            in_obtw_comment = false;
                        }
                    }
                }
            }
            _ => {
                if !in_btw_comment {
                    current_line.push(c);
                }
            }
        }
    }

    result.push_str(&current_line);

    remove_obtw_commentary(result)
}

fn remove_obtw_commentary(source: String) -> Result<String, AppError> {
    let mut result = String::new();

    let mut in_obtw = false;
    let mut just_left_obtw = false;

    for line in source.lines() {
        let line = line.trim_start(); // need this because we're using the first space to detect the first token

        // the end of the first token
        let mut new_line = String::new();
        let mut in_string = false;
        let mut first_token = true;
        let mut passed_tldr = false;

        let mut chars = line.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                _ if passed_tldr => {
                    return Err(AppError::TldrMustEndLine);
                }
                '"' if !in_obtw => {
                    in_string = !in_string;
                    new_line.push(c);
                }

                // BTW
                'B' if !in_string && !in_obtw => {
                    if chars.peek() == Some(&'T') {
                        let mut clone = chars.clone();
                        clone.next(); // T
                        if clone.peek() == Some(&'W') {
                            chars.next();
                            chars.next();
                            break;
                        }
                    }
                    new_line.push(c);
                }

                'O' if !in_string && !in_obtw => {
                    if chars.peek() == Some(&'B') {
                        let mut clone = chars.clone();
                        clone.next(); // B
                        if clone.peek() == Some(&'T') {
                            clone.next(); // T
                            if clone.peek() == Some(&'W') {
                                chars.next(); // W
                                chars.next();

                                if !first_token {
                                    return Err(AppError::ObtwMustStartLine);
                                }

                                in_obtw = true;
                            }
                        }
                    }
                    if !in_obtw {
                        new_line.push(c);
                    }
                }

                'T' if !in_string => {
                    if chars.peek() == Some(&'L') {
                        let mut clone = chars.clone();
                        clone.next(); // L
                        if clone.peek() == Some(&'D') {
                            clone.next(); // D
                            if clone.peek() == Some(&'R') {
                                chars.next(); // R
                                chars.next();
                                chars.next();

                                if !in_obtw {
                                    return Err(AppError::TldrMustBeAfterObtw);
                                }

                                in_obtw = false;
                                just_left_obtw = true;
                                passed_tldr = true;
                            }
                        }
                    }
                    if !in_obtw && !just_left_obtw {
                        new_line.push(c);
                    }
                    if just_left_obtw {
                        just_left_obtw = false;
                    }
                }

                ' ' if !in_obtw => {
                    first_token = false; // if we've encountered a space then
                    // we've passed the first token
                    new_line.push(c);
                }

                '\n' if in_obtw => {
                    first_token = true;
                    new_line.push(c);
                }

                _ => {
                    if !in_obtw {
                        new_line.push(c);
                    }
                }
            }
        }

        if !new_line.trim().is_empty() {
            result.push_str(new_line.trim_end());
            result.push('\n');
        }
    }

    Ok(result)
}

pub fn tokenize_line(line: &str) -> Result<Vec<Token>, AppError> {
    let raw_tokens = split_line(line);

    let mut tokens = Vec::new();

    for word in raw_tokens {
        tokens.extend(classify_token(word)?);
    }

    Ok(concat_adjacent_yarns(tokens))
}

fn concat_adjacent_yarns(tokens: Vec<Token>) -> Vec<Token> {
    let mut result = Vec::new();

    for token in tokens {
        match token {
            Token::Yarn(s) => {
                if let Some(Token::Yarn(prev)) = result.last_mut() {
                    prev.concat(s);
                } else {
                    result.push(Token::Yarn(s));
                }
            }
            other => result.push(other),
        }
    }

    result
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
            ' ' | '\t' if !in_string => {
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

fn classify_token(mut word: String) -> Result<Vec<Token>, AppError> {
    if word.ends_with('?') {
        word.pop();
        let mut tokens = core_classify_token(word)?;
        tokens.push(Token::QuestionMark);
        Ok(tokens)
    } else if word.ends_with('!') {
        word.pop();
        let mut tokens = core_classify_token(word)?;
        tokens.push(Token::ExclamationMark);
        Ok(tokens)
    } else {
        let tokens = core_classify_token(word)?;
        Ok(tokens)
    }
}

fn core_classify_token(word: String) -> Result<Vec<Token>, AppError> {
    if word.starts_with('"') && word.ends_with('"') {
        return Ok(vec![Token::Yarn(Yarn::from_literal(word))]);
    }

    if word == "WIN" {
        return Ok(vec![Token::Troof(Troof::new(true))]);
    }
    if word == "FAIL" {
        return Ok(vec![Token::Troof(Troof::new(false))]);
    }

    if word == "NOOB" {
        return Ok(vec![Token::Keyword(Keyword::Noob)]);
    }

    if let Ok(n) = word.parse::<i64>() {
        return Ok(vec![Token::Numbr(Numbr::new(n))]);
    }

    if let Ok(n) = BigDecimal::from_str(&word) {
        return Ok(vec![Token::Numbar(Numbar::new(n))]);
    }

    if let Some(keyword) = Keyword::from_str(&word) {
        return Ok(vec![Token::Keyword(keyword)]);
    }

    Ok(vec![Token::Identifier(Identifier::new(word)?)])
}
