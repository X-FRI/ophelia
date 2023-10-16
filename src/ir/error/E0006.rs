use crate::ast;
use crate::error::{Error, Reporter};
use crate::ir::scopes::{Scopes, current_fun};
use crate::ir::values::ExprValue;
use codespan_reporting::diagnostic::Label;
use koopa::ir::Program;

use super::REPORTER;

/// E0006: Cannot deference constant as an integer
pub struct E0006<'ast> {
    pub ast: &'ast ast::LVal,
}

impl<'ast> E0006<'ast> {
    pub fn run<Out>(
        &self,
        program: &mut Program,
        scopes: &mut Scopes<'ast>,
        num: i32,
    ) -> Result<Out, Error> {
        if self.ast.indices.is_empty() {
            let value = current_fun!(scopes).new_value(program).integer(*num);
            Ok(ExprValue::Int(value))
        } else {
            Err(Error {
                message: String::from("Cannot deference a constant as an integer"),
                code: String::from("E0006"),
                labels: vec![Label::primary(
                    unsafe { Reporter::reporter_id(REPORTER) },
                    self.ast.pos.0..self.ast.pos.1,
                )
                .with_message("This is an integer".to_string())],
                note: vec![String::from("Only pointers can be dereferenced")],
            })
        };
    }
}
