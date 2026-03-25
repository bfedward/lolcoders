use crate::{
    app_error::AppError,
    lexer::{Keyword, Token, tokenize_line},
    types::{Expr, Statement},
};
use std::iter::Peekable;

pub fn parse_line(tokens: &[Token]) -> Result<Option<Statement>, AppError> {
    if tokens.is_empty() {
        return Ok(None);
    }

    match tokens {
        [Token::Keyword(Keyword::Hai)] => Ok(Some(Statement::Hai(None))),

        [Token::Keyword(Keyword::Hai), Token::Numbar(version)] => {
            Ok(Some(Statement::Hai(Some(*version))))
        }

        [Token::Keyword(Keyword::Visible), rest @ ..] => {
            let expr = rest.try_into()?;
            Ok(Some(Statement::Visible(expr)))
        }

        [
            Token::Keyword(Keyword::I),
            Token::Keyword(Keyword::Has),
            Token::Keyword(Keyword::A),
            Token::Identifier(variable_name),
        ] => Ok(Some(Statement::IHasA(variable_name.clone(), Expr::Noob))),

        [
            Token::Keyword(Keyword::I),
            Token::Keyword(Keyword::Has),
            Token::Keyword(Keyword::A),
            Token::Identifier(variable_name),
            Token::Keyword(Keyword::Itz),
            rest @ ..,
        ] => {
            let expr = rest.try_into()?;
            Ok(Some(Statement::IHasA(variable_name.clone(), expr)))
        }

        [
            Token::Keyword(Keyword::I),
            Token::Keyword(Keyword::Iz),
            Token::Identifier(called_func),
        ] => Ok(Some(Statement::IIz(called_func.clone(), Vec::new()))),

        [
            Token::Keyword(Keyword::I),
            Token::Keyword(Keyword::Iz),
            Token::Identifier(called_func),
            Token::Keyword(Keyword::Yr),
            rest @ ..,
        ] => {
            let mut args = Vec::new();
            let mut i = 0;

            if rest.is_empty() {
                return Err(AppError::ParseError);
            }

            // Parse first argument, which just has YR <arg>
            args.push(Expr::try_from(&rest[i])?);
            i += 1;

            // remaining args, which have AN YR <arg>
            while i < rest.len() {
                match rest.get(i..i + 3) {
                    Some(
                        [
                            Token::Keyword(Keyword::An),
                            Token::Keyword(Keyword::Yr),
                            expr_token,
                        ],
                    ) => {
                        args.push(Expr::try_from(expr_token)?);
                        i += 3;
                    }
                    _ => return Err(AppError::ParseError),
                }
            }

            Ok(Some(Statement::IIz(called_func.clone(), args)))
        }

        [Token::Keyword(Keyword::KThxBye)] => Ok(Some(Statement::KThxBye)),

        _ => Err(AppError::ParseError),
    }
}

pub fn parse_function(
    tokens: &[Token],
    lines: &mut Peekable<std::str::Lines>,
) -> Result<Statement, AppError> {
    let (func_name, params) = match tokens {
        [
            Token::Keyword(Keyword::How),
            Token::Keyword(Keyword::Iz),
            Token::Keyword(Keyword::I),
            Token::Identifier(func_name),
        ] => (func_name, Vec::new()),

        [
            Token::Keyword(Keyword::How),
            Token::Keyword(Keyword::Iz),
            Token::Keyword(Keyword::I),
            Token::Identifier(func_name),
            Token::Keyword(Keyword::Yr),
            rest @ ..,
        ] => {
            let mut params = Vec::new();
            let mut i = 0;

            // First param
            match rest.get(i) {
                Some(Token::Identifier(param)) => {
                    params.push(param.clone());
                    i += 1;
                }
                _ => return Err(AppError::ParseError),
            }

            // Remaining params: AN YR <param>
            while i < rest.len() {
                match rest.get(i..i + 3) {
                    Some(
                        [
                            Token::Keyword(Keyword::An),
                            Token::Keyword(Keyword::Yr),
                            Token::Identifier(param),
                        ],
                    ) => {
                        params.push(param.clone());
                        i += 3;
                    }
                    _ => return Err(AppError::ParseError),
                }
            }

            (func_name, params)
        }

        _ => return Err(AppError::ParseError),
    };

    let mut body = Vec::new();

    let mut found_end = false;

    while let Some(line) = lines.next() {
        let tokens = tokenize_line(line)?;

        if matches!(
            tokens.as_slice(),
            [
                Token::Keyword(Keyword::If),
                Token::Keyword(Keyword::U),
                Token::Keyword(Keyword::Say),
                Token::Keyword(Keyword::So)
            ]
        ) {
            found_end = true;
            break;
        }

        if let Some(stmt) = parse_line(&tokens)? {
            body.push(stmt);
        }
    }

    if !found_end {
        return Err(AppError::ParseError);
    }

    Ok(Statement::HowIzI(func_name.clone(), params, body))
}
