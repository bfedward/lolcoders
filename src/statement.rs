use bigdecimal::BigDecimal;

use crate::{
    expression::{CastTypes, Expr},
    types::{
        Value,
        identifier::{Identifier, IdentifierExpr},
    },
};

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Hai(BigDecimal),
    Visible(Vec<Expr>, bool),
    IHasA(IdentifierExpr, Expr),
    HowIzI(IdentifierExpr, Vec<IdentifierExpr>, Vec<Statement>),
    VarRIIzFunc(IdentifierExpr, Identifier, Vec<Expr>),
    FoundYr(Expr),
    Gtfo,
    KThxBye,
    CanHasLib(IdentifierExpr),
    Gimmeh(IdentifierExpr),
    Rassignment(IdentifierExpr, Expr),
    Recast(IdentifierExpr, CastTypes),
    Expr(Expr),
    ORly(ORlyBlock),
    Wtf(WtfBlock),
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

#[derive(Debug, PartialEq, Clone, Default)]
pub struct WtfBlock {
    pub omg_blocks: Vec<OmgBlock>,
    pub omgwtf_block: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct OmgBlock {
    pub condition: Value,
    pub statements: Vec<Statement>,
    pub has_gtfo: bool,
}
