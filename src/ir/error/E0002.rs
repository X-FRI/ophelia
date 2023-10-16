use crate::ast;
use crate::error::{Error, Reporter};
use crate::ir::scopes::Scopes;
use crate::ir::values::Initializer;
use codespan_reporting::diagnostic::Label;
use koopa::ir::Program;

use super::REPORTER;

/// E0002: Fail to generate global value initialization ir
pub struct E0002<'ast> {
    pub ast: &'ast ast::Expr,
}

impl<'ast> E0002<'ast> {
    pub fn run<Out>(&self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Out, Error> {
        if scopes.is_global() {
            Initializer::Const(
                self.ast.eval(scopes).ok_or(Err(Error {
                    message: String::from("Unable to initialize constant expression"),
                    code: String::from("E0002"),
                    labels: vec![Label::primary(
                        unsafe { Reporter::reporter_id(REPORTER) },
                        self.ast.pos.0..self.ast.pos.1,
                    )
                    .with_message("Illegal constant expression".to_string())],
                    note: vec![String::from(
                        "",
                    )],
                }))?,
            )
        } else {
            Initializer::Value(self.ast.gen(program, scopes)?.into_int(program, scopes)?)
        }
    }
}
