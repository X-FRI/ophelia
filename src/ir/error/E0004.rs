use crate::ast;
use crate::error::{Error, Reporter};
use codespan_reporting::diagnostic::Label;

use super::REPORTER;

/// E0004: A function whose return value type is Unit cannot have a return value
pub struct E0004<'ast> {
    pub ast: &'ast ast::Return,
}

impl<'ast> E0004<'ast> {
    pub fn run<Out>(&self) -> Result<Out, Error> {
        Err(Error {
            message: String::from("Illegal return value statement"),
            code: String::from("E0004"),
            labels: vec![Label::primary(
                unsafe { Reporter::reporter_id(REPORTER) },
                self.ast.exp.unwrap().pos.0..self.ast.exp.unwrap().pos.1,
            )
            .with_message("Invalid return value".to_string())],
            note: vec![String::from("A function whose return value type is Unit cannot have a return value")],
        })
    }
}
