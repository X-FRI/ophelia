use codespan_reporting::diagnostic::Label;

use crate::{
    ast::{self, DefineFun, GlobalItem},
    error::{self, Reporter},
};

pub fn scan(ast: &ast::Program, reporter: &Reporter) {
    for item in &ast.items {
        match item {
            GlobalItem::DefineFun(fun) => scan_fun_def(fun, reporter),
        }
    }
}

fn scan_fun_def(fun: &DefineFun, reporter: &Reporter) {
    if fun.ident.name != "main" {
        reporter.report(error::Error {
            message: String::from("Incorrect main function"),
            code: String::from("E0001"),
            labels: vec![
                Label::primary(reporter.id, fun.ident.pos.start..fun.ident.pos.end)
                    .with_message(format!("expected `main`, found {}", fun.ident.name)),
            ],
            note: vec![String::from(
                "The entry module should start with the main function",
            )],
        })
    }
}
