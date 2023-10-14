use crate::ast;
use crate::error::{Error, Reporter};
use crate::ir::scopes::Scopes;
use codespan_reporting::diagnostic::Label;

use super::REPORTER;

/// E0005: Continue statement is not in a loop
pub struct E0005<'ast> {
    pub ast: &'ast ast::Continue,
}

impl<'ast> E0005<'ast> {
    pub fn run<Out>(&self, scopes: &mut Scopes<'ast>) -> Result<Out, Error> {
        scopes.loop_info.last().ok_or(Err(Error {
            message: String::from("The `continute` statement is not in the loop"),
            code: String::from("E0005"),
            labels: vec![Label::primary(
                unsafe { Reporter::reporter_id(REPORTER) },
                self.ast.pos.0..self.ast.pos.1,
            )
            .with_message("This is not within a loop statement".to_string())],
            note: vec![String::from(
                "`continue` must be within a loop statement",
            )],
        }))?
    }
}
