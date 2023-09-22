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
    pub typ: FunType,
    pub name: String,
    pub body: Block,
}

#[derive(Debug)]
pub enum FunType {
    Int,
}

#[derive(Debug)]
pub struct Block {
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
    pub expr: i32,
}
