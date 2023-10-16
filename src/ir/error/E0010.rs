use crate::ast;
use crate::error::{Error, Reporter};
use crate::ir::scopes::Scopes;
use codespan_reporting::diagnostic::Label;

use super::REPORTER;

/// E0002: Fail to generate global value initialization ir
pub struct E0010<'ast> {
    pub ast: &'ast ast::ConstExpr,
}

impl<'ast> E0010<'ast> {
    pub fn run<Out>(&self, scopes: &mut Scopes<'ast>) -> Result<Out, Error> {
        self.eval(scopes).ok_or(Err(Error {
            message: String::from("Unable to initialize constant expression"),
            code: String::from("E0010"),
            labels: vec![Label::primary(
                unsafe { Reporter::reporter_id(REPORTER) },
                self.ast.pos.0..self.ast.pos.1,
            )
            .with_message("Illegal constant expression".to_string())],
            note: vec![String::from("")],
        }))?
    }
}
