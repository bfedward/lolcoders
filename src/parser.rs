use crate::{
    app_error::AppError,
    lexer::tokenize_line,
    types::{Expr, Statement},
};
use std::iter::Peekable;

pub fn parse_line(tokens: &[String]) -> Result<Option<Statement>, AppError> {
    if tokens.is_empty() {
        return Ok(None);
    }

    match tokens[0].as_str() {
        "HAI" => {
            if let Some(version) = tokens.get(1) {
                Ok(Some(Statement::Hai(Some(version.clone()))))
            } else {
                Ok(Some(Statement::Hai(None)))
            }
        }

        "VISIBLE" => {
            let expr = parse_expr(&tokens[1..]);
            Ok(Some(Statement::Visible(expr)))
        }

        "I" if tokens.len() >= 4 && tokens[1] == "HAS" && tokens[2] == "A" => {
            let name = tokens[3].clone();

            if tokens.len() > 5 && tokens[4] == "ITZ" {
                let expr = parse_expr(&tokens[5..]);
                Ok(Some(Statement::IHasA(name, expr)))
            } else {
                Ok(Some(Statement::IHasA(name, Expr::Noob)))
            }
        }

        "I" if tokens.len() >= 2 && tokens[1] == "IZ" => {
            let called_func = tokens[2].clone();

            let mut args = Vec::new();

            if tokens.len() > 4 {
                if let (Some(first_yr), Some(first_arg)) = (tokens.get(4), tokens.get(5)) {
                    if first_yr == "YR" {
                        let first_arg = parse_expr(&[first_arg.clone()]);
                        args.push(first_arg);

                        let mut param_index = 6;
                        loop {
                            let next_an = tokens.get(param_index);
                            let next_yr = tokens.get(param_index + 1);
                            let next_arg = tokens.get(param_index + 2);

                            if let (Some(next_an), Some(next_yr), Some(next_arg)) =
                                (next_an, next_yr, next_arg)
                            {
                                if next_an == "AN" && next_yr == "YR" {
                                    let next_arg = parse_expr(&[next_arg.clone()]);
                                    args.push(next_arg.clone());
                                }
                            } else {
                                return Err(AppError::ParseError);
                            }

                            param_index += 3;

                            if param_index >= tokens.len() {
                                break;
                            }
                        }
                    } else {
                        return Err(AppError::ParseError);
                    }
                } else {
                    return Err(AppError::ParseError);
                }
            }

            Ok(Some(Statement::IIz(called_func, args)))
        }

        "KTHXBYE" => Ok(Some(Statement::KThxBye)),

        _ => Err(AppError::ParseError),
    }
}

pub fn parse_function(
    tokens: &[String],
    lines: &mut Peekable<std::str::Lines>,
) -> Result<Statement, AppError> {
    let func_name = tokens[3].clone();

    let mut params = Vec::new();

    if tokens.len() > 4 {
        if let (Some(first_yr), Some(first_param)) = (tokens.get(4), tokens.get(5)) {
            if first_yr == "YR" {
                params.push(first_param.clone());

                let mut param_index = 6;
                loop {
                    let next_an = tokens.get(param_index);
                    let next_yr = tokens.get(param_index + 1);
                    let next_param = tokens.get(param_index + 2);

                    if let (Some(next_an), Some(next_yr), Some(next_param)) =
                        (next_an, next_yr, next_param)
                    {
                        if next_an == "AN" && next_yr == "YR" {
                            params.push(next_param.clone());
                        }
                    } else {
                        return Err(AppError::ParseError);
                    }

                    param_index += 3;

                    if param_index >= tokens.len() {
                        break;
                    }
                }
            } else {
                return Err(AppError::ParseError);
            }
        } else {
            return Err(AppError::ParseError);
        }
    }

    let mut body = Vec::new();

    while let Some(line) = lines.next() {
        let tokens = tokenize_line(line);

        if tokens.as_slice() == ["IF", "U", "SAY", "SO"] {
            break;
        }

        if let Some(stmt) = parse_line(&tokens)? {
            body.push(stmt);
        }
    }

    Ok(Statement::HowIzI(func_name, params, body))
}

pub fn parse_expr(tokens: &[String]) -> Expr {
    if tokens.is_empty() {
        return Expr::Noob;
    }

    let token = &tokens[0];

    if token.starts_with('"') && token.ends_with('"') {
        Expr::Yarn(token.trim_matches('"').to_string())
    } else if let Ok(num) = token.parse::<f64>() {
        Expr::Numbar(num)
    } else if let Ok(num) = token.parse::<i32>() {
        Expr::Numbr(num)
    } else if token == "WIN" {
        Expr::Troof(true)
    } else if token == "FAIL" {
        Expr::Troof(false)
    } else {
        Expr::Variable(token.clone())
    }
}
