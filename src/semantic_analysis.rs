use crate::ast::{self, DefineFun, GlobalItem};
use ariadne::{ColorGenerator, Label, Report, ReportKind, Source};

pub fn scan(ast: &ast::Program, source: &String) {
    for item in &ast.items {
        match item {
            GlobalItem::DefineFun(fun) => scan_fun_def(fun, source),
        }
    }
}

fn scan_fun_def(fun: &DefineFun, source: &String) {
    if fun.ident.name != "main" {
        let mut colors = ColorGenerator::new();

        Report::build(ReportKind::Error, "helloworld.oph", 4)
            .with_code(3)
            .with_message(format!("Incorrect main function"))
            .with_label(
                Label::new(("hello_world.oph", fun.pos.start..fun.pos.end))
                    .with_message("This should be the definition of the main function")
                    .with_color(colors.next())
            )
            .with_label(
                Label::new(("hello_world.oph", fun.ident.pos.start..fun.ident.pos.end))
                    .with_message("Change to `main`")
                    .with_color(colors.next()),
            )
            .with_note("The entry module should start with the main function")
            .finish()
            .print(("hello_world.oph", Source::from(source)))
            .unwrap()
    }
}
