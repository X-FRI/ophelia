mod eval;
mod fun;
mod gen;
mod scopes;
mod values;
pub mod error;

use crate::ast::{CompUnit, ConstExpr};
use eval::Evaluate;
use gen::GenerateProgram;
use koopa::ir::{Program, Type};
use scopes::Scopes;
use std::fmt;

pub fn gen(comp_unit: &CompUnit) -> Result<Program> {
    let mut program = Program::new();
    comp_unit.gen(&mut program, &mut Scopes::new())?;
    Ok(program)
}


/// Helper trait for converting dimentions to type.
pub(crate) trait DimsToType {
    fn to_type(&self, scopes: &Scopes) -> Result<Type>;
}

impl DimsToType for Vec<ConstExpr> {
    fn to_type(&self, scopes: &Scopes) -> Result<Type> {
        self.iter().rev().fold(Ok(Type::get_i32()), |b, exp| {
            let base = b?;
            let len = exp.eval(scopes).ok_or(Error::FailedToEval)?;
            (len >= 1)
                .then(|| Type::get_array(base, len as usize))
                .ok_or(Error::InvalidArrayLen)
        })
    }
}
