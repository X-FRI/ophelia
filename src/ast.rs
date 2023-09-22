#[derive(Debug)]
pub struct Position {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct Program {
    pub items: Vec<GlobalItem>,
}

#[derive(Debug)]
pub enum GlobalItem {
    DefineFun(DefineFun),
}

#[derive(Debug)]
pub struct DefineFun {
    pub pos: Position,
    pub typ: FunType,
    pub ident: Ident,
    pub body: Block,
}

#[derive(Debug)]
pub enum Type {
    Int,
}

#[derive(Debug)]
pub struct FunType {
    pub pos: Position,
    pub typ: Type,
}

#[derive(Debug)]
pub struct Block {
    pub pos: Position,
    pub items: Vec<BlockItem>,
}

#[derive(Debug)]
pub enum BlockItem {
    Stmt(Stmt),
}

#[derive(Debug)]
pub enum Stmt {
    Return(Return),
}

#[derive(Debug)]
pub struct Return {
    pub pos: Position,
    pub expr: i32,
}

#[derive(Debug)]
pub struct Ident {
    pub pos: Position,
    pub name: String,
}
