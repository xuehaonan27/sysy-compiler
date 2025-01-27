pub type IDENT = String;

#[derive(Debug)]
pub struct CompUnit {
    pub prev_unit: Option<Box<CompUnit>>,
    pub cur_unit: DeclOrFuncDef,
}

#[derive(Debug)]
pub enum DeclOrFuncDef {
    Decl(Box<Decl>),
    FuncDef(Box<FuncDef>),
}

#[derive(Debug)]
pub enum Decl {
    ConstDecl(Box<ConstDecl>),
    VarDecl(Box<VarDecl>),
}

#[derive(Debug)]
pub struct ConstDecl {
    pub btype: BType,
    pub const_defs: Vec<ConstDef>,
}

#[derive(Debug)]
pub enum BType {
    Int,
}

#[derive(Debug)]
pub struct ConstDef {
    pub ident: IDENT,
    pub dimensions: Vec<ConstExp>,
    pub init_val: ConstInitVal,
}

#[derive(Debug)]
pub enum ConstInitVal {
    Exp(Box<ConstExp>),
    Array(Vec<ConstInitVal>),
}

#[derive(Debug)]
pub struct VarDecl {
    pub btype: BType,
    pub var_defs: Vec<VarDef>,
}

#[derive(Debug)]
pub enum VarDef {
    Uninit(IDENT, Vec<ConstExp>),
    Init(IDENT, Vec<ConstExp>, InitVal),
}

#[derive(Debug)]
pub enum InitVal {
    Exp(Box<Exp>),
    Array(Vec<InitVal>),
}

#[derive(Debug)]
pub struct FuncDef {
    pub func_type: FuncType,
    pub ident: IDENT,
    pub params: Vec<FuncFParam>,
    pub block: Block,
}

#[derive(Debug)]
pub enum FuncType {
    Void,
    Int,
}

#[derive(Debug)]
pub struct FuncFParam {
    pub btype: BType,
    pub ident: IDENT,
    pub is_array: bool,
    pub dimensions: Vec<ConstExp>,
}

#[derive(Debug)]
pub struct Block {
    pub items: Vec<BlockItem>,
}

#[derive(Debug)]
pub enum BlockItem {
    Decl(Box<Decl>),
    Stmt(Box<Stmt>),
}

#[derive(Debug)]
pub enum Stmt {
    Assign(LVal, Exp),
    Exp(Option<Exp>),
    Block(Block),
    If(Exp, Box<Stmt>, Option<Box<Stmt>>),
    While(Exp, Box<Stmt>),
    Break,
    Continue,
    Return(Option<Exp>),
}

#[derive(Debug)]
pub struct LVal {
    pub ident: IDENT,
    pub indices: Vec<Exp>,
}

#[derive(Debug)]
pub enum Exp {
    LOrExp(Box<LOrExp>),
}

#[derive(Debug)]
pub enum LOrExp {
    LAndExp(Box<LAndExp>),
    Or(Box<LOrExp>, Box<LAndExp>),
}

#[derive(Debug)]
pub enum LAndExp {
    EqExp(Box<EqExp>),
    And(Box<LAndExp>, Box<EqExp>),
}

#[derive(Debug)]
pub enum EqExp {
    RelExp(Box<RelExp>),
    Eq(Box<EqExp>, Box<RelExp>),
    Neq(Box<EqExp>, Box<RelExp>),
}

#[derive(Debug)]
pub enum EqOp {
    Eq,
    Neq,
}

#[derive(Debug)]
pub enum RelExp {
    AddExp(Box<AddExp>),
    Lt(Box<RelExp>, Box<AddExp>),
    Gt(Box<RelExp>, Box<AddExp>),
    Le(Box<RelExp>, Box<AddExp>),
    Ge(Box<RelExp>, Box<AddExp>),
}

#[derive(Debug)]
pub enum AddExp {
    MulExp(Box<MulExp>),
    Add(Box<AddExp>, Box<MulExp>),
    Sub(Box<AddExp>, Box<MulExp>),
}

#[derive(Debug)]
pub enum MulExp {
    UnaryExp(Box<UnaryExp>),
    Mul(Box<MulExp>, Box<UnaryExp>),
    Div(Box<MulExp>, Box<UnaryExp>),
    Mod(Box<MulExp>, Box<UnaryExp>),
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
}

#[derive(Debug)]
pub enum FactorOp {
    Mul,
    Div,
    Mod,
}

#[derive(Debug)]
pub enum UnaryExp {
    PrimaryExp(Box<PrimaryExp>),
    Call(IDENT, Vec<Exp>),
    UnaryOp(UnaryOp, Box<UnaryExp>),
}

#[derive(Debug)]
pub enum PrimaryExp {
    Paren(Box<Exp>),
    LVal(LVal),
    Number(i32),
}

#[derive(Debug)]
pub enum UnaryOp {
    Add,
    Sub,
    Not,
}

#[derive(Debug)]
pub enum RelOp {
    Lt,
    Gt,
    Le,
    Ge,
}

#[derive(Debug)]
pub enum ConstExp {
    Exp(Box<Exp>),
}