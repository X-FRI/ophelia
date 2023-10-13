use super::fun::FunctionInfo;
use super::values::Value;
use super::{Error, Result};
use koopa::ir::Value as IrValue;
use koopa::ir::{BasicBlock, Function, Program, Type};
use std::collections::HashMap;

pub struct Scopes<'ast> {
    vals: Vec<HashMap<&'ast str, Value>>,
    funs: HashMap<&'ast str, Function>,
    pub current_fun: Option<FunctionInfo>,
    pub loop_info: Vec<(BasicBlock, BasicBlock)>,
}

/// Returns a reference to the current funtion information.
macro_rules! current_fun {
    ($scopes:expr) => {
        $scopes.current_fun.as_ref().unwrap()
    };
}
pub(crate) use current_fun;

/// Returns a mutable reference to the current funtion information.
macro_rules! current_fun_mut {
    ($scopes:expr) => {
        $scopes.current_fun.as_mut().unwrap()
    };
}
pub(crate) use current_fun_mut;

impl<'ast> Scopes<'ast> {
    /// Creates a new `Scopes`.
    pub fn new() -> Self {
        Self {
            vals: vec![HashMap::new()],
            funs: HashMap::new(),
            current_fun: None,
            loop_info: Vec::new(),
        }
    }

    /// Returns `true` if is currently in global scope.
    pub fn is_global(&self) -> bool {
        self.current_fun.is_none()
    }

    /// Inserts a new value to the current scope.
    pub fn new_value(&mut self, id: &'ast str, value: Value) -> Result<()> {
        let is_global = self.is_global();
        let cur = self.vals.last_mut().unwrap();
        if cur.contains_key(id) || (is_global && self.funs.contains_key(id)) {
            Err(Error::DuplicatedDef)
        } else {
            cur.insert(id, value);
            Ok(())
        }
    }

    /// Returns the value by the given identifier.
    pub fn value(&self, id: &str) -> Result<&Value> {
        let mut cur = self.vals.len() as i32 - 1;
        while cur >= 0 {
            if let Some(value) = self.vals[cur as usize].get(id) {
                return Ok(value);
            }
            cur -= 1;
        }
        Err(Error::SymbolNotFound)
    }

    /// Inserts a new funtion to the current scope.
    pub fn new_fun(&mut self, id: &'ast str, fun: Function) -> Result<()> {
        if self.funs.contains_key(id) || self.vals.first().unwrap().contains_key(id) {
            Err(Error::DuplicatedDef)
        } else {
            self.funs.insert(id, fun);
            Ok(())
        }
    }

    /// Returns the funtion by the given identifier.
    pub fn fun(&self, id: &str) -> Result<Function> {
        self.funs.get(id).copied().ok_or(Error::SymbolNotFound)
    }

    /// Enters a new scope.
    pub fn enter(&mut self) {
        self.vals.push(HashMap::new());
    }

    /// Exits from the current scope.
    pub fn exit(&mut self) {
        self.vals.pop();
    }

    /// Returns type of the given value.
    pub fn ty(&self, program: &Program, value: IrValue) -> Type {
        if value.is_global() {
            program.borrow_value(value).ty().clone()
        } else {
            program
                .func(current_fun!(self).fun())
                .dfg()
                .value(value)
                .ty()
                .clone()
        }
    }
}
