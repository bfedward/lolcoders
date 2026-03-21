#[derive(Debug, Clone)]
pub enum Value {
    Numbar(f64),
    Numbr(i32),
    Yarn(String),
    Troof(bool),
    Noob,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Numbar(f64),
    Numbr(i32),
    Yarn(String),
    Troof(bool),
    Variable(String),
    Noob,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Hai(Option<String>),
    Visible(Expr),
    IHasA(String, Expr),
    HowIzI(String, Vec<String>, Vec<Statement>),
    IIz(String, Vec<Expr>),
    KThxBye,
}
