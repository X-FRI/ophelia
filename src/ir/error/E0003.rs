use crate::ast;
use crate::error::{Error, Reporter};
use crate::ir::scopes::Scopes;
use crate::ir::values::Initializer;
use codespan_reporting::diagnostic::Label;
use koopa::ir::Program;

use super::REPORTER;

/// E0002: Break statement is not in a loop
pub struct E0003<'ast> {
    pub ast: &'ast ast::Break,
}

impl<'ast> E0003<'ast> {
    pub fn run<Out>(&self, scopes: &mut Scopes<'ast>) -> Result<Out, Error> {
        scopes.loop_info.last().ok_or(Err(Error {
            message: String::from("The Break statement is not in the loop"),
            code: String::from("E0003"),
            labels: vec![Label::primary(
                unsafe { Reporter::reporter_id(REPORTER) },
                self.ast.pos.0..self.ast.pos.1,
            )
            .with_message("This is not within a loop statement".to_string())],
            note: vec![String::from(
                "Break must be within a loop statement",
            )],
        }))?
    }
}
