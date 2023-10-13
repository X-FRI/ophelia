#[derive(Debug)]
pub struct Position(pub usize, pub usize);

#[derive(Debug)]
pub struct CompUnit {
    pub items: Vec<GlobalItem>,
    pub pos: Position,
}

#[derive(Debug)]
pub struct Ident {
    pub name: String,
    pub pos: Position,
}

#[derive(Debug)]
pub enum GlobalItem {
    Decl(Decl),
    FuncDef(FuncDef),
}

#[derive(Debug)]
pub enum Decl {
    Const(ConstDecl),
    Var(VarDecl),
}

#[derive(Debug)]
pub struct ConstDecl {
    pub defs: Vec<ConstDef>,
    pub pos: Position,
}

#[derive(Debug)]
pub struct ConstDef {
    pub id: Ident,
    pub dims: Vec<ConstExpr>,
    pub init: ConstInitVal,
    pub pos: Position,
}

#[derive(Debug)]
pub enum ConstInitVal {
    Expr(ConstExpr),
    List(Vec<ConstInitVal>),
}

#[derive(Debug)]
pub struct VarDecl {
    pub defs: Vec<VarDef>,
    pub pos: Position,
}

#[derive(Debug)]
pub struct VarDef {
    pub id: Ident,
    pub dims: Vec<ConstExpr>,
    pub init: Option<InitVal>,
    pub pos: Position,
}

#[derive(Debug)]
pub enum InitVal {
    Expr(Expr),
    List(Vec<InitVal>),
}

#[derive(Debug)]
pub struct FuncDef {
    pub ty: Type,
    pub id: Ident,
    pub params: Vec<FuncFParam>,
    pub block: Block,
    pub pos: Position,
}

#[derive(Debug)]
pub enum Type {
    Unit(Position),
    Int(Position),
}

#[derive(Debug)]
pub struct FuncFParam {
    pub typ: Type,
    pub id: Ident,
    pub dims: Option<Vec<ConstExpr>>,
    pub pos: Position,
}

#[derive(Debug)]
pub struct Block {
    pub items: Vec<BlockItem>,
    pub pos: Position,
}

#[derive(Debug)]
pub enum BlockItem {
    Decl(Decl),
    Stmt(Stmt),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum Stmt {
    Assign(Assign),
    ExprStmt(ExprStmt),
    Block(Block),
    If(Box<If>),
    While(Box<While>),
    Break(Break),
    Continue(Continue),
    Return(Return),
}

#[derive(Debug)]
pub struct Assign {
    pub lval: LVal,
    pub exp: Expr,
    pub pos: Position,
}

#[derive(Debug)]
pub struct ExprStmt {
    pub exp: Option<Expr>,
    pub pos: Position,
}

#[derive(Debug)]
pub struct If {
    pub cond: Expr,
    pub then: Stmt,
    pub else_then: Option<Stmt>,
    pub pos: Position,
}

#[derive(Debug)]
pub struct While {
    pub cond: Expr,
    pub body: Stmt,
    pub pos: Position,
}

#[derive(Debug)]
pub struct Break {
    pub pos: Position,
}

#[derive(Debug)]
pub struct Continue {
    pub pos: Position,
}

#[derive(Debug)]
pub struct Return {
    pub exp: Option<Expr>,
    pub pos: Position,
}

#[derive(Debug)]
pub struct Expr {
    pub lor: LOrExpr,
    pub pos: Position,
}

#[derive(Debug)]
pub struct LVal {
    pub id: Ident,
    pub indices: Vec<Expr>,
    pub pos: Position,
}

#[derive(Debug)]
pub enum PrimaryExpr {
    Expr(Box<Expr>),
    LVal(LVal),
    Number(Number),
}

#[derive(Debug)]
pub struct Number {
    pub value: i32,
    pub pos: Position,
}

#[derive(Debug)]
pub enum UnaryExpr {
    Primary(PrimaryExpr),
    Call(FuncCall),
    Unary(UnaryOp, Box<UnaryExpr>),
}

#[derive(Debug)]
pub struct FuncCall {
    pub id: Ident,
    pub args: Vec<Expr>,
    pub pos: Position,
}

#[derive(Debug)]
pub enum MulExpr {
    Unary(UnaryExpr),
    MulUnary(Box<MulExpr>, MulOp, UnaryExpr),
}

#[derive(Debug)]
pub enum AddExpr {
    Mul(MulExpr),
    AddMul(Box<AddExpr>, AddOp, MulExpr),
}

#[derive(Debug)]
pub enum RelExpr {
    Add(AddExpr),
    RelAdd(Box<RelExpr>, RelOp, AddExpr),
}

#[derive(Debug)]
pub enum EqExpr {
    Rel(RelExpr),
    EqRel(Box<EqExpr>, EqOp, RelExpr),
}

#[derive(Debug)]
pub enum LAndExpr {
    Eq(EqExpr),
    LAndEq(Box<LAndExpr>, EqExpr),
}

#[derive(Debug)]
pub enum LOrExpr {
    LAnd(LAndExpr),
    LOrLAnd(Box<LOrExpr>, LAndExpr),
}

#[derive(Debug)]
pub struct ConstExpr {
    pub exp: Expr,
    pub pos: Position,
}

#[derive(Debug)]
pub enum UnaryOp {
    Neg(Position),
    LNot(Position),
}

#[derive(Debug)]
pub enum MulOp {
    Mul(Position),
    Div(Position),
    Mod(Position),
}

#[derive(Debug)]
pub enum AddOp {
    Add(Position),
    Sub(Position),
}

#[derive(Debug)]
pub enum RelOp {
    Lt(Position),
    Gt(Position),
    Le(Position),
    Ge(Position),
}

#[derive(Debug)]
pub enum EqOp {
    Eq(Position),
    Ne(Position),
}
