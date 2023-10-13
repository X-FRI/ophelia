pub mod E0002;
pub mod E0003;
pub mod E0004;

use crate::error::{self, Reporter};

/// Result type of IR generator.
pub type Result<T> = std::result::Result<T, error::Error>;
pub static mut REPORTER: Option<Reporter> = None;

/// Possible errors during the IR generator process
pub enum Error {
    DuplicatedDef,
    SymbolNotFound,
    FailedToEval,
    InvalidArrayLen,
    InvalidInit,
    ArrayAssign,
    NotInLoop,
    RetValInVoidFunc,
    DerefInt,
    UseVoidValue,
    ArgMismatch,
    NonIntCalc,
}