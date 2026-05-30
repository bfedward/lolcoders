use crate::{
    app_error::AppError,
    expression::Expr,
    keywords::Keyword,
    lexer::{Token, Tokens, tokenize_line},
    statement::{MebbeBlock, ORlyBlock, Statement},
    types::primitive::{Numbar, Numbr, Troof, Yarn},
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
        [Token::Keyword(Keyword::Hai)] => Err(AppError::MustGiveVersionNumberInHaiLine),

        [
            Token::Keyword(Keyword::Can),
            Token::Keyword(Keyword::Has),
            Token::Identifier(lib),
            Token::QuestionMark,
        ] => Ok(Some(Statement::CanHasLib(lib.clone()))),

        [Token::Keyword(Keyword::Hai), Token::Numbar(version)] => {
            Ok(Some(Statement::Hai(version.value())))
        }

        [Token::Keyword(Keyword::Visible), rest @ ..] => {
            let mut exprs = Vec::new();
            let mut offset = 0;
            let mut no_new_line = false;

            let mut slice = rest;

            if let Some(last) = slice.last()
                && *last == Token::ExclamationMark
            {
                no_new_line = true;
                slice = &slice[..slice.len() - 1];
            }

            while offset < slice.len() {
                if slice[offset] == Token::Keyword(Keyword::An) {
                    offset += 1;
                    continue;
                }

                let (expr, consumed) = Expr::parse(&slice[offset..])?;
                exprs.push(expr);
                offset += consumed;
            }

            if exprs.is_empty() {
                return Err(AppError::VisibleMustHaveAnArg);
            }

            Ok(Some(Statement::Visible(exprs, no_new_line)))
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
            Token::Keyword(Keyword::Noob),
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
            Token::Keyword(Keyword::A),
            rest @ ..,
        ] => {
            let (expr, _) = Expr::parse(rest)?;
            Ok(Some(Statement::IHasA(variable_name.clone(), expr)))
        }

        [
            Token::Keyword(Keyword::I),
            Token::Keyword(Keyword::Has),
            Token::Keyword(Keyword::A),
            Token::Identifier(variable_name),
            Token::Keyword(Keyword::Itz),
            rest @ ..,
        ] => {
            let (expr, _) = Expr::parse(rest)?;
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
                return Err(AppError::IncorrectFunctionArguments(
                    called_func.clone(),
                    Tokens(tokens.to_vec()),
                ));
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
                    _ => {
                        return Err(AppError::IncorrectFunctionArguments(
                            called_func.clone(),
                            Tokens(tokens.to_vec()),
                        ));
                    }
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
            Token::Keyword(Keyword::Yr),
            rest @ ..,
        ] => {
            let mut args = Vec::new();
            let mut i = 0;

            if rest.is_empty() {
                return Err(AppError::IncorrectFunctionArguments(
                    called_func.clone(),
                    Tokens(tokens.to_vec()),
                ));
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
                    None => break,
                    _ => {
                        return Err(AppError::IncorrectFunctionArguments(
                            called_func.clone(),
                            Tokens(tokens.to_vec()),
                        ));
                    }
                }
            }

            match rest.last() {
                Some(Token::Keyword(Keyword::Mkay)) => (),
                _ => return Err(AppError::FunctionArgumentsMustEndWithMkay),
            }

            Ok(Some(Statement::VarRIIzFunc(
                var_name.clone(),
                called_func.clone(),
                args,
            )))
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

        [Token::Keyword(Keyword::Gimmeh), Token::Identifier(input)] => {
            Ok(Some(Statement::Gimmeh(input.clone())))
        }

        [
            Token::Identifier(var),
            Token::Keyword(Keyword::R),
            rest @ ..,
        ] => {
            let (expr, _) = Expr::parse(rest)?;
            Ok(Some(Statement::Rassignment(var.clone(), expr)))
        }

        [
            Token::Keyword(Keyword::O),
            Token::Keyword(Keyword::Rly),
            Token::QuestionMark,
        ] => {
            let o_rly_block = parse_o_rly_block(tokens, lines)?;
            Ok(Some(Statement::ORly(o_rly_block)))
        }

        [Token::Keyword(Keyword::KThxBye)] => Ok(Some(Statement::KThxBye)),

        // a line of lolcode may just be an expression.
        _ => {
            // attempt to parse the whole line as an expression.
            match Expr::parse(tokens) {
                // if parsing produces an Expr and all line tokens are consumed,
                // then the line was just an expression.
                Ok((expr, consumed)) if consumed == tokens.len() => Ok(Some(Statement::Expr(expr))),

                // if parsing the line did not produce an Expr or some line tokens were
                // not consumed, then there is a line parse error.
                _ => Err(AppError::LineParseError(Tokens(tokens.to_vec()))),
            }
        }
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
                _ => {
                    return Err(AppError::IncorrectFunctionArguments(
                        func_name.clone(),
                        Tokens(tokens.to_vec()),
                    ));
                }
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
                    _ => {
                        return Err(AppError::IncorrectFunctionArguments(
                            func_name.clone(),
                            Tokens(tokens.to_vec()),
                        ));
                    }
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
                let (expr, _) = Expr::parse(rest)?;
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

pub fn parse_o_rly_block(
    tokens: &[Token],
    lines: &mut Peekable<std::str::Lines>,
) -> Result<ORlyBlock, AppError> {
    let mut o_rly_block = ORlyBlock::default();

    match tokens {
        [
            Token::Keyword(Keyword::O),
            Token::Keyword(Keyword::Rly),
            Token::QuestionMark,
        ] => (),
        _ => return Err(AppError::ORlyParseError),
    };

    let line = lines.next().ok_or(AppError::ORlyBlockMustHaveYaRly)?;
    let tokens = tokenize_line(line)?;

    match tokens.as_slice() {
        [Token::Keyword(Keyword::Ya), Token::Keyword(Keyword::Rly)] => (),
        _ => return Err(AppError::ORlyBlockMustHaveYaRly),
    }

    let (ya_rly_block_stmts, boundary) = parse_o_rly_sub_block(lines)?;
    o_rly_block.ya_rly_block = ya_rly_block_stmts;

    let mut boundary = boundary;

    while let ORlyBoundary::Mebbe(expr) = boundary {
        let (stmts, next_boundary) = parse_o_rly_sub_block(lines)?;

        o_rly_block.mebbe_blocks.push(MebbeBlock {
            expr,
            statements: stmts,
        });

        boundary = next_boundary;
    }

    match boundary {
        ORlyBoundary::NoWai => {
            let (stmts, next) = parse_o_rly_sub_block(lines)?;

            o_rly_block.no_wai_block = stmts;

            match next {
                ORlyBoundary::Oic => Ok(o_rly_block),
                ORlyBoundary::Mebbe(_) => Err(AppError::ORlyNoWaiBlockMustBeLast),
                ORlyBoundary::NoWai => Err(AppError::ORlyBlockCanOnlyHaveOneNoWai),
            }
        }

        ORlyBoundary::Oic => Ok(o_rly_block),

        ORlyBoundary::Mebbe(_) => Err(AppError::ORlyNoWaiBlockMustBeLast),
    }
}

enum ORlyBoundary {
    Mebbe(Expr),
    NoWai,
    Oic,
}

fn parse_o_rly_sub_block(
    lines: &mut Peekable<std::str::Lines>,
) -> Result<(Vec<Statement>, ORlyBoundary), AppError> {
    let mut statements = Vec::new();

    while let Some(line) = lines.peek() {
        let tokens = tokenize_line(line)?;

        match tokens.as_slice() {
            [Token::Keyword(Keyword::Mebbe), rest @ ..] => {
                let (expr, consumed) = Expr::parse(rest)?;
                if consumed != rest.len() {
                    return Err(AppError::ORlyParseError);
                }
                lines.next();
                return Ok((statements, ORlyBoundary::Mebbe(expr)));
            }
            [Token::Keyword(Keyword::No), Token::Keyword(Keyword::Wai)] => {
                lines.next();
                return Ok((statements, ORlyBoundary::NoWai));
            }
            [Token::Keyword(Keyword::Oic)] => {
                lines.next();
                return Ok((statements, ORlyBoundary::Oic));
            }
            _ => {
                let line = lines.next().ok_or(AppError::ORlyParseError)?;
                let tokens = tokenize_line(line)?;

                if let Some(stmt) = parse_line(&tokens, lines)? {
                    statements.push(stmt);
                }
            }
        }
    }

    Err(AppError::ORlyBlockMustEndOic)
}
