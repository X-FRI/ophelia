#[derive(Debug)]
pub struct Position(pub usize, pub usize);

#[derive(Debug)]
pub struct CompUnit {
    pub items: Vec<GlobalItem>,
    pub pos: Position,
}

#[derive(Debug)]
pub struct Ident {
    pub id: String,
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
    pub dims: Vec<ConstExp>,
    pub init: ConstInitVal,
    pub pos: Position,
}

#[derive(Debug)]
pub enum ConstInitVal {
    Exp(ConstExp),
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
    pub dims: Vec<ConstExp>,
    pub init: Option<InitVal>,
    pub pos: Position,
}

#[derive(Debug)]
pub enum InitVal {
    Exp(Exp),
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
    pub dims: Option<Vec<ConstExp>>,
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
    ExpStmt(ExpStmt),
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
    pub exp: Exp,
    pub pos: Position,
}

#[derive(Debug)]
pub struct ExpStmt {
    pub exp: Option<Exp>,
    pub pos: Position,
}

#[derive(Debug)]
pub struct If {
    pub cond: Exp,
    pub then: Stmt,
    pub else_then: Option<Stmt>,
    pub pos: Position,
}

#[derive(Debug)]
pub struct While {
    pub cond: Exp,
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
    pub exp: Option<Exp>,
    pub pos: Position,
}

#[derive(Debug)]
pub struct Exp {
    pub lor: LOrExp,
    pub pos: Position,
}

#[derive(Debug)]
pub struct LVal {
    pub id: Ident,
    pub indices: Vec<Exp>,
    pub pos: Position,
}

#[derive(Debug)]
pub enum PrimaryExp {
    Exp(Box<Exp>),
    LVal(LVal),
    Number(Number),
}

#[derive(Debug)]
pub struct Number {
    pub value: i32,
    pub pos: Position,
}

#[derive(Debug)]
pub enum UnaryExp {
    Primary(PrimaryExp),
    Call(FuncCall),
    Unary(UnaryOp, Box<UnaryExp>),
}

#[derive(Debug)]
pub struct FuncCall {
    pub id: Ident,
    pub args: Vec<Exp>,
    pub pos: Position,
}

#[derive(Debug)]
pub enum MulExp {
    Unary(UnaryExp),
    MulUnary(Box<MulExp>, MulOp, UnaryExp),
}

#[derive(Debug)]
pub enum AddExp {
    Mul(MulExp),
    AddMul(Box<AddExp>, AddOp, MulExp),
}

#[derive(Debug)]
pub enum RelExp {
    Add(AddExp),
    RelAdd(Box<RelExp>, RelOp, AddExp),
}

#[derive(Debug)]
pub enum EqExp {
    Rel(RelExp),
    EqRel(Box<EqExp>, EqOp, RelExp),
}

#[derive(Debug)]
pub enum LAndExp {
    Eq(EqExp),
    LAndEq(Box<LAndExp>, EqExp),
}

#[derive(Debug)]
pub enum LOrExp {
    LAnd(LAndExp),
    LOrLAnd(Box<LOrExp>, LAndExp),
}

#[derive(Debug)]
pub struct ConstExp {
    pub exp: Exp,
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
