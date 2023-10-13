use codespan_reporting::diagnostic::Label;

mod reporter;

pub use reporter::Reporter;

pub struct Error {
    pub message: String,
    pub code: String,
    pub labels: Vec<Label<usize>>,
    pub note: Vec<String>,
}

impl Reporter {
    pub fn reporter_id(reporter: Option<Reporter>) -> u32 {
        unsafe { reporter.unwrap().id }
    }
}
