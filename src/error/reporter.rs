use codespan_reporting::diagnostic::Diagnostic;
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};

pub struct Reporter {
    pub id: usize,
    pub files: SimpleFiles<String, String>,
}

impl Reporter {
    pub fn new(filename: &str, source: &str) -> Reporter {
        let mut files = SimpleFiles::new();
        let id = files.add(filename.to_owned(), source.to_owned());

        Reporter { id, files }
    }

    pub fn report(&self, error: &super::Error) {
        let diagnostic = Diagnostic::error()
            .with_message(&error.message)
            .with_code(&error.code)
            .with_labels(error.labels.clone())
            .with_notes(error.note.clone());

        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config::default();

        term::emit(&mut writer.lock(), &config, &self.files, &diagnostic).unwrap();
    }

    pub fn report_all(&self, errors: &Vec<super::Error>) {
        for error in errors {
            self.report(error)
        }
    }
}
