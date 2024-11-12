#[derive(Debug)]
pub struct CompUnit {
    pub items: Vec<GlobalItem>,
}

#[derive(Debug)]
pub enum FuncType {
    Int,
    Void,
}
#[derive(Debug)]
pub enum GlobalItem {
    FuncDef(FuncDef),
    Decl(Decl),
}
#[derive(Debug)]
pub enum Exp {
    LOrExp(LOrExp),
}

#[derive(Debug)]
pub enum UnaryExp {
    PrimaryExp(PrimaryExp),
    FuncCall(FuncCall),
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
    LVal(LVal),
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
    pub params: Vec<FuncFParam>,
    pub block: Block,
}
#[derive(Debug)]
pub struct FuncFParam {
    pub b_type: BType,
    pub name: String,
    pub array_indices: Option<Vec<ConstExp>>,
}

#[derive(Debug)]
pub struct FuncCall {
    pub func_name: String,
    pub params: Vec<Exp>,
}

#[derive(Debug)]
pub struct Block {
    pub items: Vec<BlockItem>,
}

#[derive(Debug)]
pub enum BlockItem {
    Decl(Decl),
    Stmt(Stmt),
}

#[derive(Debug)]
pub enum Decl {
    ConstDecl(ConstDecl),
    VarDecl(VarDecl),
}
#[derive(Debug)]
pub struct VarDecl {
    pub b_type: BType,
    pub def_list: Vec<VarDef>,
}
#[derive(Debug)]
pub enum VarDef {
    IdOnly(String, Vec<ConstExp>),
    Assign(String, Vec<ConstExp>, InitVal),
}
#[derive(Debug)]
pub enum InitVal {
    List(Vec<InitVal>),
    Single(Exp),
}

#[derive(Debug)]
pub struct ConstDecl {
    pub b_type: BType,
    pub def_list: Vec<ConstDef>,
}
#[derive(Debug)]
pub struct ConstDef {
    pub id: String,
    pub dims: Vec<ConstExp>,
    pub init_val: ConstInitVal,
}
#[derive(Debug)]
pub enum ConstInitVal {
    Single(ConstExp),
    List(Vec<ConstExp>),
}
#[derive(Debug)]
pub struct ConstExp {
    pub exp: Exp,
}
#[derive(Debug)]
pub enum BType {
    Int,
}
#[derive(Debug)]
pub struct LVal {
    pub id: String,
    pub indices: Vec<Box<Exp>>,
}

#[derive(Debug)]
pub enum Stmt {
    Ret(Exp),
    Assign(LVal, Exp),
    Exp(Option<Exp>),
    IfStmt(IfStmt),
    Block(Box<Block>),
    While(While),
    Break(Break),
    Continue(Continue),
}
#[derive(Debug)]
pub struct Break {}

#[derive(Debug)]
pub struct Continue {}
#[derive(Debug)]
pub struct IfStmt {
    pub cond: Exp,
    pub then: Box<Stmt>,
    pub else_stmt: Option<Box<Stmt>>,
}
#[derive(Debug)]
pub struct While {
    pub cond: Exp,
    pub body: Box<Stmt>,
}
