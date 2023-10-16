use crate::ast::{self, Type};
use crate::error::{Error, Reporter};
use crate::ir::scopes::Scopes;
use crate::ir::values::Value;
use codespan_reporting::diagnostic::Label;
use koopa::ir::Program;

use super::REPORTER;

/// E0008: Wrong parameter type
pub struct E0009<'ast> {
    pub ast: &'ast ast::FuncCall,
}

impl<'ast> E0009<'ast> {
    pub fn run<Out>(
        &self,
        params_ty: &Vec<Type>,
        args: &Vec<Value>,
        program: &mut Program,
        scopes: &mut Scopes<'ast>,
    ) -> Result<Out, Error> {
        for (param_ty, arg) in params_ty.iter().zip(&args) {
            let ty = &scopes.ty(program, *arg);
            if param_ty != ty {
                Err(Error {
                    message: String::from("Wrong parameter type"),
                    code: String::from("E0009"),
                    labels: vec![Label::primary(
                        unsafe { Reporter::reporter_id(REPORTER) },
                        param_ty.pos().0..param_ty.pos().1,
                    )
                    .with_message(format!("expect: {}", ty.to_string()))],
                    note: vec![String::from("The parameter type of the function call does not match the actual parameter type")],
                })
            }
        }
    }
}
