use crate::{
    expression::Expr,
    types::identifier::{Identifier, IdentifierExpr},
};

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Hai(f64),
    Visible(Vec<Expr>, bool),
    IHasA(IdentifierExpr, Expr),
    HowIzI(Identifier, Vec<Identifier>, Vec<Statement>),
    IIz(Identifier, Vec<Expr>),
    VarRIIzFunc(IdentifierExpr, Identifier, Vec<Expr>),
    FoundYr(Expr),
    Gtfo,
    KThxBye,
    CanHasLib(IdentifierExpr),
    Gimmeh(IdentifierExpr),
    Rassignment(IdentifierExpr, Expr),
    Expr(Expr),
    ORly(ORlyBlock),
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct ORlyBlock {
    pub ya_rly_block: Vec<Statement>,
    pub mebbe_blocks: Vec<MebbeBlock>,
    pub no_wai_block: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MebbeBlock {
    pub expr: Expr,
    pub statements: Vec<Statement>,
}
