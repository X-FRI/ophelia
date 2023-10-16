use crate::ast::{self, Type};
use crate::error::{Error, Reporter};
use crate::ir::values::Value;
use codespan_reporting::diagnostic::Label;

use super::REPORTER;

/// E0008: Missing parameters
pub struct E0008<'ast> {
    pub ast: &'ast ast::FuncCall,
}

impl<'ast> E0008<'ast> {
    pub fn run<Out>(&self, params_ty: &Vec<Type>, args: &Vec<Value>) -> Result<Out, Error> {
        if params_ty.len() != args.len() {
            Err(Error {
                message: String::from("Missing parameters"),
                code: String::from("E0008"),
                labels: vec![Label::primary(
                    unsafe { Reporter::reporter_id(REPORTER) },
                    self.ast.pos.0..self.ast.pos.1,
                )
                .with_message(format!("expect: {}", Type::to_string(params_ty)))],
                note: vec![String::from("The parameters of the function call do not match the actual number of parameters")],
            })
        }
    }
}
