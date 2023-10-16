use crate::ast;
use crate::error::{Error, Reporter};
use codespan_reporting::diagnostic::Label;

use super::REPORTER;

/// E0007: Cannot deference an array as an integer
pub struct E0007<'ast> {
    pub ast: &'ast ast::LVal,
}

impl<'ast> E0007<'ast> {
    pub fn run<Out>(&self, dims: i32) -> Result<Out, Error> {
        if dims == 0 {
            Err(Error {
                message: String::from("Cannot deference an array as an integer"),
                code: String::from("E0006"),
                labels: vec![Label::primary(
                    unsafe { Reporter::reporter_id(REPORTER) },
                    self.ast.pos.0..self.ast.pos.1,
                )
                .with_message("This is an integer".to_string())],
                note: vec![String::from("Only pointers can be dereferenced")],
            })
        }
    }
}
