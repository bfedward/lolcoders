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
pub enum Expr {
    Numbar(Numbar),
    Numbr(Numbr),
    Yarn(Yarn),
    Troof(Troof),
    Variable(Identifier),
    Noob,
    Math(MathsExpr),
}

impl TryFrom<&Token> for Expr {
    type Error = AppError;

    fn try_from(token: &Token) -> Result<Self, AppError> {
        match token {
            Token::Numbar(n) => Ok(Expr::Numbar(Numbar::new(*n))),
            Token::Numbr(n) => Ok(Expr::Numbr(Numbr::new(*n))),
            Token::Yarn(s) => Ok(Expr::Yarn(Yarn::new(s.clone()))),
            Token::Troof(b) => Ok(Expr::Troof(Troof::new(*b))),
            Token::Noob => Ok(Expr::Noob),
            Token::Identifier(ident) => Ok(Expr::Variable(ident.clone())),
            Token::Keyword(_) => Err(AppError::TokenCannotBeExpression(token.clone())),
        }
    }
}

impl Expr {
    pub fn parse(tokens: &[Token]) -> Result<(Self, usize), AppError> {
        match tokens.first() {
            Some(Token::Keyword(k)) if MathOp::try_from(&Token::Keyword(k.clone())).is_ok() => {
                let (maths_expr, consumed) = Expr::parse_math_expr(tokens)?;
                Ok((Expr::Math(maths_expr), consumed))
            }

            // this is for converting single tokens to an expression
            Some(token) => Ok((Expr::try_from(token)?, 1)),

            None => Err(AppError::MissingExpression),
        }
    }

    fn parse_math_expr(tokens: &[Token]) -> Result<(MathsExpr, usize), AppError> {
        // which maths op are we doing?
        let op = MathOp::try_from(tokens.first().ok_or(AppError::MissingExpression)?)?;

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
}
