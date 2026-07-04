use crate::{
    app_error::AppError,
    expression::Expr,
    keywords::Keyword,
    lexer::{Token, Tokens, tokenize_line},
    statement::{MebbeBlock, ORlyBlock, Statement},
    types::{
        identifier::IdentifierExpr,
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
        [Token::Keyword(Keyword::Hai)] => Err(AppError::MustGiveVersionNumberInHaiLine),

        [
            Token::Keyword(Keyword::Can),
            Token::Keyword(Keyword::Has),
            rest @ ..,
        ] => {
            let Some(Token::QuestionMark) = rest.last() else {
                return Err(AppError::CanHasMustEndQuestionMark);
            };

            let ident_tokens = &rest[..rest.len() - 1];

            let (ident_expr, consumed) = IdentifierExpr::parse(ident_tokens)?;

            if consumed != ident_tokens.len() {
                return Err(AppError::InvalidIdentifierExpr(Tokens(tokens.to_vec())));
            }

            Ok(Some(Statement::CanHasLib(ident_expr)))
        }

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
            rest @ ..,
        ] => {
            let mut i = 0;

            if rest.get(i) == Some(&Token::Keyword(Keyword::A)) {
                i += 1;
            }

            let (var, consumed) = IdentifierExpr::parse(&rest[i..])?;
            i += consumed;

            let mut init = Expr::Noob;

            if let Some(Token::Keyword(Keyword::Itz)) = rest.get(i) {
                i += 1;

                // optional "A" after ITZ
                if rest.get(i) == Some(&Token::Keyword(Keyword::A)) {
                    i += 1;
                }

                // special typed init
                let (expr, consumed) = match rest.get(i) {
                    Some(Token::Keyword(Keyword::Yarn)) => (Expr::Yarn(Yarn::default()), 1),
                    Some(Token::Keyword(Keyword::Troof)) => (Expr::Troof(Troof::default()), 1),
                    Some(Token::Keyword(Keyword::Numbar)) => (Expr::Numbar(Numbar::default()), 1),
                    Some(Token::Keyword(Keyword::Numbr)) => (Expr::Numbr(Numbr::default()), 1),
                    Some(Token::Keyword(Keyword::Noob)) => (Expr::Noob, 1),
                    Some(_) => Expr::parse(&rest[i..])?,
                    None => return Err(AppError::UnexpectedEOF),
                };

                init = expr;

                i += consumed;
            }

            if i != rest.len() {
                return Err(AppError::InvalidSyntax(Tokens(tokens.to_vec())));
            }

            Ok(Some(Statement::IHasA(var, init)))
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

            let var_id = var_name.clone().into();

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
                var_id,
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
        ] => {
            let var_id = var_name.clone().into();

            Ok(Some(Statement::VarRIIzFunc(
                var_id,
                called_func.clone(),
                Vec::new(),
            )))
        }

        [Token::Keyword(Keyword::Gimmeh), rest @ ..] => {
            let (gimmeh_id, consumed) = IdentifierExpr::parse(rest)?;

            if consumed != rest.len() {
                return Err(AppError::InvalidIdentifierExpr(Tokens(tokens.to_vec())));
            }

            Ok(Some(Statement::Gimmeh(gimmeh_id)))
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

        
        _ => {
            // reassignment with R has an unknown number of tokens before and after R,
            // so handle it specifically instead of in the match statement below.
            if let Some(r_pos) = tokens.iter().position(|t| *t == Token::Keyword(Keyword::R)) {
                let lhs = &tokens[..r_pos];
                let rhs = &tokens[r_pos + 1..];

                let (ident_expr, consumed) = IdentifierExpr::parse(lhs)?;

                if consumed != lhs.len() {
                    return Err(AppError::InvalidIdentifierExpr(Tokens(lhs.to_vec())));
                }

                let (expr, consumed) = Expr::parse(rhs)?;

                if consumed != rhs.len() {
                    return Err(AppError::InvalidExpression(Tokens(rhs.to_vec())));
                }

                return Ok(Some(Statement::Rassignment(ident_expr, expr)));
            }




            // a line of lolcode may just be an expression.
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
