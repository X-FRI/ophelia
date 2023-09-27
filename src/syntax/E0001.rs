use crate::ast;
use crate::error::Error;
use crate::semantic::Checker;
use codespan_reporting::diagnostic::Label;

/// E0001: The entry module should start with the main function.
pub struct E0001<'ast> {
    ast: &'ast ast::CompUnit,
    reporter_id: usize,
}

impl<'ast> Checker<'ast> for E0001<'ast> {
    fn run(&self) -> Result<(), Error> {
        for fun in &self.ast.items {
            match &fun {
                ast::GlobalItem::FuncDef(fun_def) => {
                    if fun_def.id.id != "main" {
                        return Err(Error {
                            message: String::from("Incorrect main function"),
                            code: String::from("E0001"),
                            labels: vec![Label::primary(
                                self.reporter_id,
                                fun_def.id.pos.0..fun_def.id.pos.1,
                            )
                            .with_message(format!("expected `main`, found {}", fun_def.id.id))],
                            note: vec![String::from(
                                "The entry module should start with the main function",
                            )],
                        });
                    }
                }
                _ => return Ok(()),
            }
        }

        Ok(())
    }
}

impl E0001<'_> {
    pub fn new(ast: &ast::CompUnit, reporter_id: usize) -> Box<E0001> {
        Box::new(E0001 { ast, reporter_id })
    }
}
