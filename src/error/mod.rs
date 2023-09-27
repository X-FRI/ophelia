use codespan_reporting::diagnostic::Label;

mod reporter;

pub use reporter::Reporter;

pub struct Error {
    pub message: String,
    pub code: String,
    pub labels: Vec<Label<usize>>,
    pub note: Vec<String>,
}
