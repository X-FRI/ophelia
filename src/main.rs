mod ast;
mod semantic_analysis;

use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fs::read_to_string;
use std::io::Result;

lalrpop_mod!(ophelia);

fn main() -> Result<()> {
    let mut args = args();

    args.next();
    let _mode = args.next().unwrap();
    let input = args.next().unwrap();
    args.next();
    // let _output = args.next().unwrap();

    let input = read_to_string(input)?;
    let ast = ophelia::ProgramParser::new().parse(&input).unwrap();

    semantic_analysis::scan(&ast, &input);

    Ok(())
}
