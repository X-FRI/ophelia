mod ast;
mod error;
mod semantic_analysis;

use error::Reporter;
use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fs::read_to_string;
use std::io::Result;

lalrpop_mod! {
    #[allow(clippy::all)]
    ophelia
}

fn main() -> Result<()> {
    let mut args = args();

    args.next();
    let _mode = args.next().unwrap();
    let file = args.next().unwrap();

    args.next();
    // let _output = args.next().unwrap();

    let source_code = read_to_string(&file)?;
    let reporter = Reporter::new(&file, &source_code);

    let ast = ophelia::ProgramParser::new().parse(&source_code).unwrap();

    semantic_analysis::scan(&ast, &reporter);

    Ok(())
}
