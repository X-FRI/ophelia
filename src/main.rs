mod ast;
// mod codegen;
// mod irgen;
mod error;

use error::Reporter;
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
    let reporter = Reporter::new(&file, &source_code);

    let ast = ophelia::CompUnitParser::new().parse(&source_code).unwrap();

    println!("{:#?}", ast);

    Ok(())
    // generate IR
    // let program = irgen::generate_program(&comp_unit).map_err(Error::Generate)?;
    // if matches!(mode, Mode::Koopa) {
    //     return KoopaGenerator::from_path(output)
    //         .map_err(Error::File)?
    //         .generate_on(&program)
    //         .map_err(Error::Io);
    // }
    // generate RISC-V assembly
    // codegen::generate_asm(&program, &output).map_err(Error::Io)
}
