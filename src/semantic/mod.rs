use crate::ast;
use crate::error::Error;

mod E0001;

pub(crate) trait Pass<'ast> {
    fn run(&'ast self) -> Result<(), Error>;
}

pub struct PassManager<'ast> {
    passes: Vec<Box<dyn Pass<'ast>>>,
    reporter_id: usize,
}

impl<'ast> PassManager<'ast> {
    pub fn new(ast: &'ast ast::CompUnit, reporter_id: usize) -> Self {
        PassManager {
            reporter_id,
            passes: vec![Box::new(E0001::E0001 { ast, reporter_id })],
        }
    }

    pub fn run(&'ast self) -> Vec<Error> {
        let mut pass_results = Vec::new();

        for pass in &self.passes {
            match pass.run() {
                Err(e) => pass_results.push(e),
                _ => (),
            }
        }

        pass_results
    }
}
