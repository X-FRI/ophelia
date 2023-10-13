mod ast;
// mod codegen;
mod irgen;
mod error;
mod syntax;

use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fs::read_to_string;
use std::io;

lalrpop_mod! {
  #[allow(clippy::all)]
  ophelia
}

fn main() -> io::Result<()> {
    let mut args = args();

    args.next();
    let _mode = args.next().unwrap();
    let file = args.next().unwrap();

    args.next();
    // let _output = args.next().unwrap();

    let source_code = read_to_string(&file)?;
    let reporter = error::Reporter::new(&file, &source_code);

    let ast = ophelia::CompUnitParser::new().parse(&source_code).unwrap();
    let syntax_checker = syntax::CheckerManager::new(&ast, reporter.id);

    reporter.report_all(&syntax_checker.run());

    // println!("{:#?}", ast);

    // generate IR
    let program = irgen::gen(&ast).unwrap();
    // if matches!(mode, Mode::Koopa) {
    //     return KoopaGenerator::from_path(output)
    //         .map_err(Error::File)?
    //         .generate_on(&program)
    //         .map_err(Error::Io);
    // }
    // generate RISC-V assembly
    // codegen::generate_asm(&program, &output).map_err(Error::Io)

    Ok(())
}
