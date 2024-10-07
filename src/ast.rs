use std::ops::Add;

#[derive(Debug)]
pub struct CompUnit {
    pub func_def: FuncDef,
}

#[derive(Debug)]
pub enum FuncType {
    Int,
}
#[derive(Debug)]
pub enum Exp {
    LOrExp(LOrExp),
}

#[derive(Debug)]
pub enum UnaryExp {
    PrimaryExp(PrimaryExp),
    UnaryExp(UnaryOp, Box<UnaryExp>),
}
#[derive(Debug)]
pub enum MulExp {
    UnaryExp(UnaryExp),
    MulExp(Box<MulExp>, MulOp, UnaryExp),
}
#[derive(Debug)]
pub enum MulOp {
    Multi,
    Divide,
    Mod,
}
#[derive(Debug)]
pub enum AddExp {
    MulExp(MulExp),
    AddExp(Box<AddExp>, AddOp, MulExp),
}
#[derive(Debug)]
pub enum AddOp {
    Add,
    Minus,
}

#[derive(Debug)]
pub enum RelOp {
    Gt,
    Lt,
    Ge,
    Le,
}
#[derive(Debug)]
pub enum EqOp {
    Eq,
    Ne,
}
#[derive(Debug)]
pub enum LAndOp {
    And,
}
#[derive(Debug)]
pub enum LOrOp {
    Or,
}
#[derive(Debug)]
pub enum RelExp {
    AddExp(AddExp),
    RelExp(Box<RelExp>, RelOp, AddExp),
}
#[derive(Debug)]
pub enum EqExp {
    RelExp(RelExp),
    EqExp(Box<EqExp>, EqOp, RelExp),
}
#[derive(Debug)]
pub enum LAndExp {
    EqExp(EqExp),
    LAndExp(Box<LAndExp>, LAndOp, EqExp),
}
#[derive(Debug)]
pub enum LOrExp {
    LAndExp(LAndExp),
    LOrExp(Box<LOrExp>, LOrOp, LAndExp),
}

#[derive(Debug)]
pub enum PrimaryExp {
    Number(i32),
    Exp(Box<Exp>),
}

#[derive(Debug)]
pub enum UnaryOp {
    POSITIVE,
    NEGATIVE,
    NOT,
}

#[derive(Debug)]
pub struct FuncDef {
    pub func_type: FuncType,
    pub ident: String,
    pub block: Block,
}

#[derive(Debug)]
pub struct Block {
    pub stmt: Stmt,
}

#[derive(Debug)]
pub struct Stmt {
    pub exp: Exp,
}
