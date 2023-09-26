use super::fun::FunctionInfo;
use koopa::ir::{Program, Value};
use std::collections::HashMap;

/// Some necessary information during assembly generation.
pub struct ProgramInfo<'p> {
    program: &'p Program,
    values: HashMap<Value, String>,
    current_fun: Option<FunctionInfo>,
}

/// Returns a reference to the current funtion information.
macro_rules! current_fun {
    ($info:expr) => {
        $info.current_fun().unwrap()
    };
}
pub(crate) use current_fun;

/// Returns a mutable reference to the current funtion information.
macro_rules! current_fun_mut {
    ($info:expr) => {
        $info.current_fun_mut().unwrap()
    };
}
pub(crate) use current_fun_mut;

impl<'p> ProgramInfo<'p> {
    /// Creates a new program information.
    pub fn new(program: &'p Program) -> Self {
        Self {
            program,
            values: HashMap::new(),
            current_fun: None,
        }
    }

    /// Returns a reference to the program.
    pub fn program(&self) -> &'p Program {
        self.program
    }

    /// Returns the name of the given global value.
    pub fn value(&self, value: Value) -> &str {
        self.values.get(&value).unwrap()
    }

    /// Inserts a new global value name.
    pub fn insert_value(&mut self, value: Value, name: String) {
        self.values.insert(value, name);
    }

    /// Returns a reference to the current funtion information.
    pub fn current_fun(&self) -> Option<&FunctionInfo> {
        self.current_fun.as_ref()
    }

    /// Returns a mutable reference to the current funtion information.
    pub fn current_fun_mut(&mut self) -> Option<&mut FunctionInfo> {
        self.current_fun.as_mut()
    }

    /// Sets the current funtion information.
    pub fn set_current_fun(&mut self, fun: FunctionInfo) {
        self.current_fun = Some(fun);
    }
}
