use crate::ast;
use crate::error::Error;

#[allow(non_snake_case)]
mod E0001;

pub(crate) trait Checker<'ast> {
    fn run(&self) -> Result<(), Error>;
}

pub struct CheckerManager<'ast> {
    passes: Vec<Box<dyn Checker<'ast> + 'ast>>,
}

impl<'ast> CheckerManager<'ast> {
    pub fn new(ast: &'ast ast::CompUnit, reporter_id: usize) -> Self {
        CheckerManager {
            passes: vec![E0001::E0001::new(&ast, reporter_id)],
        }
    }

    pub fn run(&self) -> Vec<Error> {
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
