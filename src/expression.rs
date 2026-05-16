use crate::{
    app_error::AppError,
    keywords::Keyword,
    lexer::{Token, Tokens},
    types::{
        identifier::Identifier,
        primitive::{Numbar, Numbr, Troof, Yarn},
    },
};

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonOp {
    BothSaem,
    Diffrint,
}

impl TryFrom<&Token> for ComparisonOp {
    type Error = AppError;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        match token {
            Token::Keyword(Keyword::Both) => Ok(ComparisonOp::BothSaem),
            Token::Keyword(Keyword::Diffrint) => Ok(ComparisonOp::Diffrint),
            _ => Err(AppError::InvalidExpression(Tokens(vec![token.clone()]))),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BoolOp {
    Both,
    Either,
    Won,
    Not,
    All,
    Any,
}

impl TryFrom<&Token> for BoolOp {
    type Error = AppError;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        match token {
            Token::Keyword(Keyword::Both) => Ok(BoolOp::Both),
            Token::Keyword(Keyword::Either) => Ok(BoolOp::Either),
            Token::Keyword(Keyword::Won) => Ok(BoolOp::Won),
            Token::Keyword(Keyword::Not) => Ok(BoolOp::Not),
            Token::Keyword(Keyword::All) => Ok(BoolOp::All),
            Token::Keyword(Keyword::Any) => Ok(BoolOp::Any),
            _ => Err(AppError::InvalidExpression(Tokens(vec![token.clone()]))),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MathOp {
    Sum,
    Diff,
    Produkt,
    Quoshunt,
    Mod,
    Biggr,
    Smallr,
}

impl TryFrom<&Token> for MathOp {
    type Error = AppError;

    fn try_from(token: &Token) -> Result<Self, AppError> {
        match token {
            Token::Keyword(Keyword::Sum) => Ok(MathOp::Sum),
            Token::Keyword(Keyword::Diff) => Ok(MathOp::Diff),
            Token::Keyword(Keyword::Produkt) => Ok(MathOp::Produkt),
            Token::Keyword(Keyword::Quoshunt) => Ok(MathOp::Quoshunt),
            Token::Keyword(Keyword::Mod) => Ok(MathOp::Mod),
            Token::Keyword(Keyword::Biggr) => Ok(MathOp::Biggr),
            Token::Keyword(Keyword::Smallr) => Ok(MathOp::Smallr),
            _ => Err(AppError::InvalidExpression(Tokens(vec![token.clone()]))),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MathsExpr {
    pub op: MathOp,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComparisonExpr {
    pub op: ComparisonOp,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Numbar(Numbar),
    Numbr(Numbr),
    Yarn(Yarn),
    Troof(Troof),
    Variable(Identifier),
    Noob,
    Math(MathsExpr),
    Bool { op: BoolOp, args: Vec<Expr> },
    Comparison(ComparisonExpr),
    Negation(Box<Expr>),
}

impl TryFrom<&Token> for Expr {
    type Error = AppError;

    fn try_from(token: &Token) -> Result<Self, AppError> {
        match token {
            Token::Numbar(n) => Ok(Expr::Numbar(n.clone())),
            Token::Numbr(n) => Ok(Expr::Numbr(n.clone())),
            Token::Yarn(s) => Ok(Expr::Yarn(s.clone())),
            Token::Troof(b) => Ok(Expr::Troof(b.clone())),
            Token::Noob => Ok(Expr::Noob),
            Token::Identifier(ident) => Ok(Expr::Variable(ident.clone())),
            Token::Keyword(_) => Err(AppError::TokenCannotBeExpression(token.clone())),
            Token::QuestionMark => Err(AppError::QuestionMarkIsNotAnExpression),
            Token::ExclamationMark => Err(AppError::ExclamationMarkIsNotAnExpression),
        }
    }
}

impl Expr {
    pub fn parse(tokens: &[Token]) -> Result<(Self, usize), AppError> {
        // if there's only one Token
        if tokens.len() == 1
            && let Some(first) = tokens.first()
        {
            return Ok((Expr::try_from(first)?, 1));
        }

        // parsing is done by matching on the first token.
        // this doesn't work for BOTH, because BOTH SAEM is a comparison expr
        // and BOTH OF is a boolean expression.
        match tokens {
            [
                Token::Keyword(Keyword::Both),
                Token::Keyword(Keyword::Saem),
                ..,
            ] => {
                let (comp_expr, consumed) = Expr::parse_comparison_expr(tokens)?;
                return Ok((Expr::Comparison(comp_expr), consumed));
            }
            [
                Token::Keyword(Keyword::Both),
                Token::Keyword(Keyword::Of),
                ..,
            ] => {
                let (op, exprs, consumed) = Expr::parse_bool_expr(tokens)?;
                return Ok((Expr::Bool { op, args: exprs }, consumed));
            }
            _ => (),
        }

        match tokens.first() {
            Some(Token::Keyword(Keyword::Not)) => {
                let (negation, consumed) = Expr::parse(&tokens[1..])?;
                Ok((Expr::Negation(Box::new(negation)), consumed + 1))
            }

            // maths expr
            Some(Token::Keyword(k)) if MathOp::try_from(&Token::Keyword(k.clone())).is_ok() => {
                let (maths_expr, consumed) = Expr::parse_math_expr(tokens)?;
                Ok((Expr::Math(maths_expr), consumed))
            }

            // bool expr
            Some(Token::Keyword(k)) if BoolOp::try_from(&Token::Keyword(k.clone())).is_ok() => {
                let (op, exprs, consumed) = Expr::parse_bool_expr(tokens)?;
                Ok((Expr::Bool { op, args: exprs }, consumed))
            }

            // comparison expr
            Some(Token::Keyword(k))
                if ComparisonOp::try_from(&Token::Keyword(k.clone())).is_ok() =>
            {
                let (comp_expr, consumed) = Expr::parse_comparison_expr(tokens)?;
                Ok((Expr::Comparison(comp_expr), consumed))
            }

            // this is for converting single tokens to an expression
            Some(token) => Ok((Expr::try_from(token)?, 1)),

            None => Err(AppError::MissingExpression(Tokens(tokens.to_vec()))),
        }
    }

    fn parse_math_expr(tokens: &[Token]) -> Result<(MathsExpr, usize), AppError> {
        // which maths op are we doing?
        let op = MathOp::try_from(
            tokens
                .first()
                .ok_or(AppError::MissingExpression(Tokens(tokens.to_vec())))?,
        )?;

        // Check that OF comes after the maths expression (e.g. "SUM")
        match tokens.get(1) {
            Some(Token::Keyword(Keyword::Of)) => {}
            _ => return Err(AppError::InvalidExpression(Tokens(tokens.to_vec()))),
        }

        // Parse expression after "SUM OF"
        // We have &tokens[2..] here because SUM + OF = 2 tokens
        // We don't know what comes after OF! Could be a literal, e.g. SUM OF 1 AN 2
        // Or could be a nested expression like SUM OF SUM OF 1 AN 2 AN 4
        //
        // consumed_left is the number of tokens consumed from tokens[2..]
        // starting immediately after "SUM OF".
        //
        // e.g. consumed_left would be 1 for a literal or 5 for
        // a nested SUM with two literals.
        let (left, consumed_left) = Expr::parse(&tokens[2..])?;

        // SUM + OF = 2 tokens
        // consumed_left = the number of tokens that were consumed in parsing
        // the left side of the SUM.
        // We expect AN between the left and right sides of the SUM.
        match tokens.get(2 + consumed_left) {
            Some(Token::Keyword(Keyword::An)) => {}
            _ => return Err(AppError::InvalidExpression(Tokens(tokens.to_vec()))),
        }

        // SUM + OF + AN = 3 tokens
        // consumed_left = the number of tokens that were consumed in parsing
        // the left side of the SUM.
        //
        // consumed_right is the number of tokens that are consumed to parse the right
        // side of the SUM, which could be a literal or another nexted expression.
        let (right, consumed_right) = Expr::parse(&tokens[3 + consumed_left..])?;

        // We have to use Boxes because Rust needs to know the size of everything on the stack.
        // We use Box because Expr is recursive.
        // Without Box, Expr::Sum(Expr, Expr) would have infinite size at compile time
        // because Expr would contain itself directly.
        // Box gives us a fixed-size pointer on the stack with the actual data on the heap.
        //
        // SUM + OF + AN = 3 tokens

        // this would need changed to return MathsExpr::Sum, MathsExpr::Diff and so on
        Ok((
            MathsExpr {
                op,
                left: Box::new(left),
                right: Box::new(right),
            },
            3 + consumed_left + consumed_right,
        ))
    }

    fn parse_bool_expr(tokens: &[Token]) -> Result<(BoolOp, Vec<Expr>, usize), AppError> {
        let op = BoolOp::try_from(
            tokens
                .first()
                .ok_or(AppError::MissingExpression(Tokens(tokens.to_vec())))?,
        )?;

        match op {
            BoolOp::Not => {
                // NOT only has one following expr
                let (expr, consumed) = Expr::parse(&tokens[2..])?;
                Ok((op, vec![expr], consumed + 1))
            }
            BoolOp::Both | BoolOp::Either | BoolOp::Won => {
                // BOTH, EITHER and WON have two following expr.
                // This is exactly the same logic in parse_maths_expr(), except here
                // we return (BoolOp, Vec<Expr>, usize) instead of (MathsExpr, usize)
                match tokens.get(1) {
                    Some(Token::Keyword(Keyword::Of)) => {}
                    _ => return Err(AppError::InvalidExpression(Tokens(tokens.to_vec()))),
                }

                let (left, consumed_left) = Expr::parse(&tokens[2..])?;

                match tokens.get(2 + consumed_left) {
                    Some(Token::Keyword(Keyword::An)) => {}
                    _ => return Err(AppError::InvalidExpression(Tokens(tokens.to_vec()))),
                }

                let (right, consumed_right) = Expr::parse(&tokens[3 + consumed_left..])?;

                Ok((op, vec![left, right], 3 + consumed_left + consumed_right))
            }
            BoolOp::All | BoolOp::Any => {
                // ALL and ANY both have an indefinite number of exprs ending with MKAY.
                match tokens.get(1) {
                    Some(Token::Keyword(Keyword::Of)) => {}
                    _ => return Err(AppError::InvalidExpression(Tokens(tokens.to_vec()))),
                }

                match tokens.last() {
                    Some(Token::Keyword(Keyword::Mkay)) => (),
                    _ => return Err(AppError::TroofExpressionMustEndWithMkay),
                }

                let mut total_consumed = 2;
                let mut exprs = Vec::new();

                // len() - 2 because we know MKAY is at the end.
                while total_consumed < tokens.len() - 2 {
                    let (expr, consumed) = Expr::parse(&tokens[total_consumed..])?;
                    exprs.push(expr);
                    total_consumed += consumed;
                }

                Ok((op, exprs, total_consumed))
            }
        }
    }

    fn parse_comparison_expr(tokens: &[Token]) -> Result<(ComparisonExpr, usize), AppError> {
        let op = ComparisonOp::try_from(
            tokens
                .first()
                .ok_or(AppError::MissingExpression(Tokens(tokens.to_vec())))?,
        )?;

        // BOTH has SAEM
        let consume_from = match tokens.get(1) {
            Some(Token::Keyword(Keyword::Saem)) => 2,
            _ => 1,
        };

        let (left, consumed_left) = Expr::parse(&tokens[consume_from..])?;

        match tokens.get(consume_from + consumed_left) {
            Some(Token::Keyword(Keyword::An)) => {}
            _ => return Err(AppError::InvalidExpression(Tokens(tokens.to_vec()))),
        }

        let (right, consumed_right) = Expr::parse(&tokens[consume_from + 1 + consumed_left..])?;

        Ok((
            ComparisonExpr {
                op,
                left: Box::new(left),
                right: Box::new(right),
            },
            consume_from + 1 + consumed_left + consumed_right,
        ))
    }
}
