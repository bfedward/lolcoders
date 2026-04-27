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

pub fn normalise_source(source: String) -> Result<String, AppError> {
    dbg!(&source);
    let mut result = String::new();
    let mut current_line = String::new();

    let mut in_string = false;
    let mut in_btw_comment = false;
    let mut in_obtw_comment = false;

    let mut chars = source.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' => {
                if !in_btw_comment {
                    in_string = !in_string;
                    current_line.push(c);
                }
            }
            '\n' if !in_string && !in_btw_comment => {
                if current_line.ends_with("...") && !in_btw_comment {
                    current_line.push(' ');
                } else {
                    in_btw_comment = false;
                    result.push_str(&current_line);
                    result.push(c);
                    current_line.clear();
                }
            }
            '\n' if in_btw_comment => {
                in_btw_comment = false;
                result.push(c);
            }
            ',' if !in_string && !in_btw_comment => {
                current_line.push('\n');
                result.push_str(&current_line);
                current_line.clear();
            }
            '\t' if !in_string && !in_btw_comment => {
                current_line.push_str("    ");
            }
            // BTW
            'B' if !in_string && !in_btw_comment && !in_obtw_comment => {
                dbg!('B');
                dbg!(in_obtw_comment);
                if chars.peek() == Some(&'T') {
                    let mut clone = chars.clone();
                    clone.next(); // T
                    if clone.peek() == Some(&'W') {
                        chars.next();
                        chars.next();
                        result.push_str(&current_line);
                        current_line.clear();
                        in_btw_comment = true;
                        dbg!(in_btw_comment);
                    }
                } else {
                    current_line.push(c);
                }
            }
            // OBTW
            'O' if !in_string && !in_btw_comment && !in_obtw_comment => {
                dbg!('O');
                if chars.peek() == Some(&'B') {
                    dbg!('B');
                    let mut clone = chars.clone();
                    clone.next(); // B
                    if clone.peek() == Some(&'T') {
                        dbg!('T');
                        clone.next(); // T
                        if clone.peek() == Some(&'W') {
                            dbg!('W');
                            chars.next(); // W
                            chars.next();
                            chars.next();
                            current_line.push_str("OBTW");
                            in_obtw_comment = true;
                            dbg!(in_obtw_comment);
                        } else {
                            dbg!("No W");
                        }
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
        dbg!(&current_line);
    }

    dbg!(&result);

    remove_commentary(result)
}

fn remove_commentary(source: String) -> Result<String, AppError> {
    dbg!(&source);
    let mut result = String::new();

    let mut in_obtw = false;
    let mut just_left_obtw = false;

    for line in source.lines() {
        let line = line.trim_start(); // need this because we're using the first space to detect the first token

        let is_obtw = line.find("OBTW");
        if let Some(is_obtw) = is_obtw
            && is_obtw != 0
        {
            return Err(AppError::ObtwMustStartLine);
        }

        let is_tldr = line.find("TLDR");
        if let Some(is_tldr) = is_tldr
            && line.len() > is_tldr + 4
        {
            return Err(AppError::TldrMustEndLine);
        }

        dbg!(&line);
        // the end of the first token
        let mut new_line = String::new();
        let mut in_string = false;
        let mut first_token = true;

        let mut chars = line.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
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
                    dbg!('T');
                    if chars.peek() == Some(&'L') {
                        dbg!('L');
                        let mut clone = chars.clone();
                        clone.next(); // L
                        if clone.peek() == Some(&'D') {
                            dbg!('D');
                            clone.next(); // D
                            if clone.peek() == Some(&'R') {
                                dbg!('R');
                                chars.next(); // R
                                chars.next();
                                chars.next();

                                dbg!(first_token);

                                if !in_obtw {
                                    return Err(AppError::TldrMustBeAfterObtw);
                                }

                                in_obtw = false;
                                just_left_obtw = true;
                            }
                        }
                    }
                    if !in_obtw && !just_left_obtw {
                        dbg!("Pushing T");
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

        dbg!(&new_line);
    }

    dbg!(&result);

    Ok(result)
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
