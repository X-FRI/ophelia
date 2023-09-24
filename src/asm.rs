use koopa::ir::{Function, FunctionData, Program, ValueKind};
use koopa::ir::entities::ValueData;

pub fn gen(ir: &Program) -> String {
    let mut asm = String::from(".text\n.global main\n");

    for &func in ir.func_layout() {
        let fun_ir = ir.func(func);
        asm += format!("\n{}:\n{}", fun_ir.name(), gen_fun(fun_ir)).as_str()
    }
    asm
}

fn gen_fun(ir: &FunctionData) -> String {
    let mut fun_asm = String::new();

    for (&block, node) in ir.layout().bbs() {
        let mut asm = String::new();

        for &inst in node.insts().keys() {
            asm += format!("\t{}\n", gen_instr(ir.dfg().value(inst), &ir)).as_str()
        }

        fun_asm += asm.as_str()
    }

    fun_asm
}

fn gen_instr(ir: &ValueData, fun_ir: &FunctionData) -> String {
    match ir.kind() {
        ValueKind::Integer(ir) => {
            format!("{}", ir.value())
        }
        ValueKind::Return(ir) => {
            format!("li a0, {}\n\tret",
                    match ir.value() {
                        None => "".to_owned(),
                        Some(ret) => gen_instr(fun_ir.dfg().value(ret), fun_ir)
                    })
        }
        _ => unimplemented!()
    }
}