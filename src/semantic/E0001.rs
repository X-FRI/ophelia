use crate::ast;
use crate::error::Error;
use crate::semantic::Pass;
use codespan_reporting::diagnostic::Label;

/// E0001: The entry module should start with the main function.
pub struct E0001<'ast> {
    pub ast: &'ast ast::CompUnit,
    pub reporter_id: usize,
}

impl<'ast> Pass<'ast> for E0001<'ast> {
    fn run(&'ast self) -> Result<(), Error> {
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
