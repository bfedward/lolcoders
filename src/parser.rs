use crate::{
    app_error::AppError,
    lexer::{Keyword, Token, Tokens, tokenize_line},
    types::{
        Expr, Statement,
        primitive::{Numbar, Numbr, Troof, Yarn},
    },
};
use std::iter::Peekable;

pub fn parse_line(
    tokens: &[Token],
    lines: &mut Peekable<std::str::Lines>,
) -> Result<Option<Statement>, AppError> {
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
            Token::Keyword(Keyword::A),
            Token::Keyword(var_type),
        ] => {
            let variable_name = variable_name.clone();
            match var_type {
                Keyword::Yarn => Ok(Some(Statement::IHasA(
                    variable_name,
                    Expr::Yarn(Yarn::default()),
                ))),
                Keyword::Troof => Ok(Some(Statement::IHasA(
                    variable_name,
                    Expr::Troof(Troof::default()),
                ))),
                Keyword::Numbar => Ok(Some(Statement::IHasA(
                    variable_name,
                    Expr::Numbar(Numbar::default()),
                ))),
                Keyword::Numbr => Ok(Some(Statement::IHasA(
                    variable_name,
                    Expr::Numbr(Numbr::default()),
                ))),
                Keyword::Noob => Ok(Some(Statement::IHasA(variable_name.clone(), Expr::Noob))),
                _ => Err(AppError::UnknownVariableType),
            }
        }

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
            Token::Keyword(Keyword::How),
            Token::Keyword(Keyword::Iz),
            Token::Keyword(Keyword::I),
            Token::Identifier(_),
            ..,
        ] => {
            let func = parse_function(tokens, lines)?;
            Ok(Some(func))
        }

        [
            Token::Keyword(Keyword::I),
            Token::Keyword(Keyword::Iz),
            Token::Identifier(called_func),
            Token::Keyword(Keyword::Mkay),
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
                return Err(AppError::IncorrectFunctionArguments(called_func.clone()));
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
                    _ => return Err(AppError::IncorrectFunctionArguments(called_func.clone())),
                }
            }

            match rest.last() {
                Some(Token::Keyword(Keyword::Mkay)) => (),
                _ => return Err(AppError::FunctionArgumentsMustEndWithMkay),
            }

            Ok(Some(Statement::IIz(called_func.clone(), args)))
        }

        [
            Token::Identifier(var_name),
            Token::Keyword(Keyword::R),
            Token::Keyword(Keyword::I),
            Token::Keyword(Keyword::Iz),
            Token::Identifier(called_func),
            Token::Keyword(Keyword::Mkay),
        ] => Ok(Some(Statement::VarRIIzFunc(
            var_name.clone(),
            called_func.clone(),
            Vec::new(),
        ))),

        [Token::Keyword(Keyword::KThxBye)] => Ok(Some(Statement::KThxBye)),

        _ => Err(AppError::LineParseError(Tokens(tokens.to_vec()))),
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
                _ => return Err(AppError::IncorrectFunctionArguments(func_name.clone())),
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
                    _ => return Err(AppError::IncorrectFunctionArguments(func_name.clone())),
                }
            }

            (func_name, params)
        }

        _ => return Err(AppError::FunctionParseError),
    };

    let mut body = Vec::new();

    let mut if_u_say_so = false;

    while let Some(line) = lines.next() {
        let tokens = tokenize_line(line)?;

        match tokens.as_slice() {
            [
                Token::Keyword(Keyword::If),
                Token::Keyword(Keyword::U),
                Token::Keyword(Keyword::Say),
                Token::Keyword(Keyword::So),
            ] => {
                if_u_say_so = true;
                break;
            }
            [
                Token::Keyword(Keyword::Found),
                Token::Keyword(Keyword::Yr),
                rest @ ..,
            ] => {
                let expr: Expr = rest.try_into()?;
                body.push(Statement::FoundYr(expr))
            }
            [Token::Keyword(Keyword::Gtfo)] => body.push(Statement::Gtfo),
            _ => {
                if let Some(stmt) = parse_line(&tokens, lines)? {
                    body.push(stmt);
                }
            }
        }
    }

    if !if_u_say_so {
        return Err(AppError::FunctionMustEndIfUSaySo);
    }

    Ok(Statement::HowIzI(func_name.clone(), params, body))
}
