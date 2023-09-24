use koopa::ir::Program;
use crate::ast;

pub fn gen(ast: &ast::Program) -> String {
    let mut ir = String::new();

    for global_item in &ast.items {
        ir += match global_item {
            ast::GlobalItem::DefineFun(ast) => gen_fun(&ast)
        }.as_str()
    }

    ir
}

pub fn gen_bin(ast: &ast::Program) -> Program {
    let driver = koopa::front::Driver::from(gen(ast));
    driver.generate_program().unwrap()
}

fn gen_fun(ast: &ast::DefineFun) -> String {
    format!("fun @{}(): {} {{%entry: {}}}", ast.ident.name, gen_type(&ast.typ.typ), gen_block(&ast.body))
}

fn gen_type(ast: &ast::Type) -> String {
    match ast {
        ast::Type::Int => "i32"
    }.to_string()
}

fn gen_block(ast: &ast::Block) -> String {
    let mut ir = String::new();

    for block_item in &ast.items {
        ir += match block_item {
            ast::BlockItem::Stmt(ast) => gen_stmt(&ast)
        }.as_str()
    }

    ir
}

fn gen_stmt(ast: &ast::Stmt) -> String {
    match ast {
        ast::Stmt::Return(ast) => gen_stmt_ret(ast)
    }
}

fn gen_stmt_ret(ast: &ast::Return) -> String {
    format!("ret {}", ast.expr)
}