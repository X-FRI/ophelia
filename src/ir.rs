use crate::ast;
use koopa::ir::Program;

pub fn gen(ast: &ast::Program) -> String {
    let mut ir = String::new();

    for global_item in &ast.items {
        ir += match global_item {
            ast::GlobalItem::DefineFun(ast) => gen_fun(&ast),
        }
        .as_str()
    }

    ir
}

pub fn gen_bin(ast: &ast::Program) -> Program {
    let driver = koopa::front::Driver::from(gen(ast));
    driver.generate_program().unwrap()
}

fn gen_fun(ast: &ast::DefineFun) -> String {
    format!(
        "fun @{}(): {} {{\n%entry: \n\t{}\n}}",
        ast.ident.name,
        gen_type(&ast.typ.typ),
        gen_block(&ast.body)
    )
}

fn gen_type(ast: &ast::Type) -> String {
    match ast {
        ast::Type::Int => "i32",
    }
    .to_string()
}

fn gen_block(ast: &ast::Block) -> String {
    let mut ir = String::new();

    for block_item in &ast.items {
        ir += match block_item {
            ast::BlockItem::Stmt(ast) => gen_stmt(&ast),
        }
        .as_str()
    }

    ir
}

fn gen_stmt(ast: &ast::Stmt) -> String {
    match ast {
        ast::Stmt::Return(ast) => gen_stmt_ret(&ast.expr),
    }
}

fn gen_stmt_ret(ast: &ast::Expr) -> String {
    let mut symbol: u32 = 0;
    format!("{}\n\tret %{}", gen_expr(ast, &mut symbol), symbol)
}

fn gen_expr(ast: &ast::Expr, symbol: &mut u32) -> String {
    match ast {
        ast::Expr::UnaryExpr(ast) => gen_unary_expr(&ast.to_owned(), symbol),
    }
}

fn gen_unary_expr(ast: &ast::UnaryExpr, symbol: &mut u32) -> String {
    match ast {
        ast::UnaryExpr::Primary(ast) => gen_primary_expr(&ast.to_owned(), symbol),
        ast::UnaryExpr::Unary(op, ast) => gen_unary_op(&op, &ast.to_owned(), symbol),
    }
}

fn gen_unary_op(op: &ast::UnaryOp, ast: &ast::UnaryExpr, symbol: &mut u32) -> String {
    match op {
        ast::UnaryOp::Add(_) => gen_unary_expr(&ast, symbol),
        ast::UnaryOp::Sub(_) => gen_unary_sub(&ast, symbol),
        ast::UnaryOp::Not(_) => gen_unary_not(&ast, symbol),
    }
}

fn gen_unary_sub(ast: &ast::UnaryExpr, symbol: &mut u32) -> String {
    *symbol += 1;
    format!("%{} = sub 0, %{}", *symbol, *symbol - 1)
}

fn gen_unary_not(ast: &ast::UnaryExpr, symbol: &mut u32) -> String {
    *symbol += 1;
    format!(
        "{}\n\t%{} = eq {}, 0",
        gen_unary_expr(&ast, symbol),
        symbol,
        *symbol - 1
    )
}

fn gen_primary_expr(ast: &ast::PrimaryExpr, symbol: &mut u32) -> String {
    match ast {
        ast::PrimaryExpr::Expr(ast) => gen_expr(&ast.to_owned(), symbol),
        ast::PrimaryExpr::Number(ast) => gen_number(&ast, symbol),
    }
}

fn gen_number(ast: &ast::Number, symbol: &mut u32) -> String {
    *symbol += 1;
    format!("\n\t%{} = {}", symbol, ast.value)
}
