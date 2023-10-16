use super::error::E0003;
use super::error::E0004;
use super::error::E0005;
use super::error::E0006;
use super::error::E0007;
use super::error::E0008;
use super::error::E0009;
use super::error::Result;
use super::error::E0002;
use super::eval::Evaluate;
use super::fun::FunctionInfo;
use super::scopes::{current_fun, current_fun_mut, Scopes};
use super::values::{ExprValue, Initializer, Value};
use super::DimsToType;
use crate::ast::{self, *};
use koopa::ir::builder_traits::*;
use koopa::ir::{BinaryOp, FunctionData, Program, Type, TypeKind};

/// Trait for generating Koopa IR program.
pub trait GenerateProgram<'ast> {
    type Out;

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out>;
}

impl<'ast> GenerateProgram<'ast> for CompUnit {
    type Out = ();

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        let mut new_decl = |name, params_ty, ret_ty| {
            scopes
                .new_fun(
                    name,
                    program.new_func(FunctionData::new_decl(
                        format!("@{}", name),
                        params_ty,
                        ret_ty,
                    )),
                )
                .unwrap();
        };
        // generate SysY library funtion declarations
        new_decl("getint", vec![], Type::get_i32());
        new_decl("getch", vec![], Type::get_i32());
        new_decl(
            "getarray",
            vec![Type::get_pointer(Type::get_i32())],
            Type::get_i32(),
        );
        new_decl("putint", vec![Type::get_i32()], Type::get_unit());
        new_decl("putch", vec![Type::get_i32()], Type::get_unit());
        new_decl(
            "putarray",
            vec![Type::get_i32(), Type::get_pointer(Type::get_i32())],
            Type::get_unit(),
        );
        new_decl("starttime", vec![], Type::get_unit());
        new_decl("stoptime", vec![], Type::get_unit());
        // generate global items
        for item in &self.items {
            item.gen(program, scopes)?;
        }
        Ok(())
    }
}

impl<'ast> GenerateProgram<'ast> for GlobalItem {
    type Out = ();

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        match self {
            Self::Decl(decl) => decl.gen(program, scopes),
            Self::FuncDef(def) => def.gen(program, scopes),
        }
    }
}

impl<'ast> GenerateProgram<'ast> for Decl {
    type Out = ();

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        match self {
            Self::Const(c) => c.gen(program, scopes),
            Self::Var(v) => v.gen(program, scopes),
        }
    }
}

impl<'ast> GenerateProgram<'ast> for ConstDecl {
    type Out = ();

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        for def in &self.defs {
            def.gen(program, scopes)?;
        }
        Ok(())
    }
}

impl<'ast> GenerateProgram<'ast> for ConstDef {
    type Out = ();

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        // generate type and initializer
        let ty = self.dims.to_type(scopes)?;
        let init = self.init.gen(program, scopes)?.reshape(&ty)?;
        // generate constant
        if ty.is_i32() {
            match init {
                Initializer::Const(num) => scopes.new_value(&self.id.name, Value::Const(num))?,
                _ => unreachable!(),
            }
        } else {
            let value = if scopes.is_global() {
                let init = init.into_const(program, scopes)?;
                let value = program.new_value().global_alloc(init);
                program.set_value_name(value, Some(format!("@{}", self.id.name)));
                value
            } else {
                let info = current_fun!(scopes);
                let alloc = info.new_alloc(program, ty, Some(&self.id.name));
                init.into_stores(program, scopes, alloc);
                alloc
            };
            // add to scope
            scopes.new_value(&self.id.name, Value::Value(value))?;
        }
        Ok(())
    }
}

impl<'ast> GenerateProgram<'ast> for ConstInitVal {
    type Out = Initializer;

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        Ok(match self {
            Self::Expr(exp) => Initializer::Const(exp.gen(program, scopes)?),
            Self::List(list) => Initializer::List(
                list.iter()
                    .map(|v| v.gen(program, scopes))
                    .collect::<Result<_>>()?,
            ),
        })
    }
}

impl<'ast> GenerateProgram<'ast> for VarDecl {
    type Out = ();

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        for def in &self.defs {
            def.gen(program, scopes)?;
        }
        Ok(())
    }
}

impl<'ast> GenerateProgram<'ast> for VarDef {
    type Out = ();

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        // generate type and initializer
        let ty = self.dims.to_type(scopes)?;
        let init = self
            .init
            .as_ref()
            .map(|i| i.gen(program, scopes)?.reshape(&ty))
            .transpose()?;
        // generate variable
        let value = if scopes.is_global() {
            let init = match init {
                Some(init) => init.into_const(program, scopes)?,
                None => program.new_value().zero_init(ty),
            };
            let value = program.new_value().global_alloc(init);
            program.set_value_name(value, Some(format!("@{}", self.id.name)));
            value
        } else {
            let info = current_fun!(scopes);
            let alloc = info.new_alloc(program, ty, Some(&self.id.name));
            if let Some(init) = init {
                init.into_stores(program, scopes, alloc);
            }
            alloc
        };
        // add to scope
        scopes.new_value(&self.id.name, Value::Value(value))?;
        Ok(())
    }
}

impl<'ast> GenerateProgram<'ast> for InitVal {
    type Out = Initializer;

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        Ok(match self {
            // self.ast.eval(scopes)
            Self::Expr(exp) => (E0002::E0002 { ast: exp }).run(program, scopes),
            Self::List(list) => Initializer::List(
                list.iter()
                    .map(|v| v.gen(program, scopes))
                    .collect::<Result<_>>()?,
            ),
        })
    }
}

impl<'ast> GenerateProgram<'ast> for FuncDef {
    type Out = ();

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        // generate parameter types and return type
        let params_ty = self
            .params
            .iter()
            .map(|p| p.gen(program, scopes))
            .collect::<Result<Vec<_>>>()?;
        let ret_ty = self.ty.gen(program, scopes)?;
        // create new fucntion
        let mut data = FunctionData::new(format!("@{}", self.id.name), params_ty, ret_ty);
        // get parameter list
        let params = data.params().to_owned();
        // generate entry/end/cur block
        let entry = data.dfg_mut().new_bb().basic_block(Some("%entry".into()));
        let end = data.dfg_mut().new_bb().basic_block(Some("%end".into()));
        let cur = data.dfg_mut().new_bb().basic_block(None);
        let mut ret_val = None;
        // generate return value
        if matches!(self.ty, ast::Type::Int(_)) {
            let alloc = data.dfg_mut().new_value().alloc(Type::get_i32());
            data.dfg_mut().set_value_name(alloc, Some("%ret".into()));
            ret_val = Some(alloc);
        }
        // update funtion information
        let fun = program.new_func(data);
        let mut info = FunctionInfo::new(fun, entry, end, ret_val);
        info.push_bb(program, entry);
        if let Some(ret_val) = info.ret_val() {
            info.push_inst(program, ret_val);
        }
        info.push_bb(program, cur);
        // generate allocations for parameters
        scopes.enter();
        for (param, value) in self.params.iter().zip(params) {
            let ty = program.func(fun).dfg().value(value).ty().clone();
            let alloc = info.new_alloc(program, ty, Some(&param.id.name));
            let store = info.new_value(program).store(value, alloc);
            info.push_inst(program, store);
            scopes.new_value(&param.id.name, Value::Value(alloc))?;
        }
        // update scope
        scopes.new_fun(&self.id.name, fun)?;
        scopes.current_fun = Some(info);
        // generate funtion body
        self.block.gen(program, scopes)?;
        scopes.exit();
        // handle end basic block
        let mut info = scopes.current_fun.take().unwrap();
        info.seal_entry(program, cur);
        info.seal_fun(program);
        Ok(())
    }
}

impl<'ast> GenerateProgram<'ast> for ast::Type {
    type Out = Type;

    fn gen(&'ast self, _: &mut Program, _: &mut Scopes<'ast>) -> Result<Self::Out> {
        Ok(match self {
            Self::Unit(_) => Type::get_unit(),
            Self::Int(_) => Type::get_i32(),
        })
    }
}

impl<'ast> GenerateProgram<'ast> for FuncFParam {
    type Out = Type;

    fn gen(&'ast self, _: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        Ok(match &self.dims {
            Some(dims) => Type::get_pointer(dims.to_type(scopes)?),
            None => Type::get_i32(),
        })
    }
}

impl<'ast> GenerateProgram<'ast> for Block {
    type Out = ();

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        scopes.enter();
        for item in &self.items {
            item.gen(program, scopes)?;
        }
        scopes.exit();
        Ok(())
    }
}

impl<'ast> GenerateProgram<'ast> for BlockItem {
    type Out = ();

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        match self {
            Self::Decl(decl) => decl.gen(program, scopes),
            Self::Stmt(stmt) => stmt.gen(program, scopes),
        }
    }
}

impl<'ast> GenerateProgram<'ast> for Stmt {
    type Out = ();

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        match self {
            Self::Assign(s) => s.gen(program, scopes),
            Self::ExprStmt(s) => s.gen(program, scopes),
            Self::Block(s) => s.gen(program, scopes),
            Self::If(s) => s.gen(program, scopes),
            Self::While(s) => s.gen(program, scopes),
            Self::Break(s) => s.gen(program, scopes),
            Self::Continue(s) => s.gen(program, scopes),
            Self::Return(s) => s.gen(program, scopes),
        }
    }
}

impl<'ast> GenerateProgram<'ast> for Assign {
    type Out = ();

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        // generate value and left-value pointer
        let exp = self.exp.gen(program, scopes)?.into_int(program, scopes)?;
        let lval = self.lval.gen(program, scopes)?.into_ptr()?;
        // generate store
        let info = current_fun!(scopes);
        let store = info.new_value(program).store(exp, lval);
        info.push_inst(program, store);
        Ok(())
    }
}

impl<'ast> GenerateProgram<'ast> for ExprStmt {
    type Out = ();

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        if let Some(exp) = &self.exp {
            exp.gen(program, scopes)?;
        }
        Ok(())
    }
}

impl<'ast> GenerateProgram<'ast> for If {
    type Out = ();

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        // generate condition
        let cond = self.cond.gen(program, scopes)?.into_int(program, scopes)?;
        // generate branch and then/else basic block
        let info = current_fun_mut!(scopes);
        let then_bb = info.new_basic_block(program, Some("%if_then"));
        let else_bb = info.new_basic_block(program, Some("%if_else"));
        let br = info.new_value(program).branch(cond, then_bb, else_bb);
        info.push_inst(program, br);
        info.push_bb(program, then_bb);
        // generate then statement
        self.then.gen(program, scopes)?;
        // generate jump and end basic block
        let info = current_fun_mut!(scopes);
        let end_bb = info.new_basic_block(program, Some("%if_end"));
        let jump = info.new_value(program).jump(end_bb);
        info.push_inst(program, jump);
        info.push_bb(program, else_bb);
        // generate else statement
        if let Some(else_then) = &self.else_then {
            else_then.gen(program, scopes)?;
        }
        // generate jump
        let info = current_fun_mut!(scopes);
        let jump = info.new_value(program).jump(end_bb);
        info.push_inst(program, jump);
        info.push_bb(program, end_bb);
        Ok(())
    }
}

impl<'ast> GenerateProgram<'ast> for While {
    type Out = ();

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        // generate loop entry basic block
        let info = current_fun_mut!(scopes);
        let entry_bb = info.new_basic_block(program, Some("%while_entry"));
        let jump = info.new_value(program).jump(entry_bb);
        info.push_inst(program, jump);
        info.push_bb(program, entry_bb);
        // generate condition
        let cond = self.cond.gen(program, scopes)?.into_int(program, scopes)?;
        // generate branch and loop body/end basic block
        let info = current_fun_mut!(scopes);
        let body_bb = info.new_basic_block(program, Some("%while_body"));
        let end_bb = info.new_basic_block(program, Some("%while_end"));
        let br = info.new_value(program).branch(cond, body_bb, end_bb);
        info.push_inst(program, br);
        info.push_bb(program, body_bb);
        // generate loop body
        scopes.loop_info.push((entry_bb, end_bb));
        self.body.gen(program, scopes)?;
        scopes.loop_info.pop();
        // generate jump
        let info = current_fun_mut!(scopes);
        let jump = info.new_value(program).jump(entry_bb);
        info.push_inst(program, jump);
        info.push_bb(program, end_bb);
        Ok(())
    }
}

impl<'ast> GenerateProgram<'ast> for Break {
    type Out = ();

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        // jump to the end of loop
        let info = &mut current_fun_mut!(scopes);
        let (_, end) = 
            // scopes.loop_info.last()
            (E0003::E0003 { ast: self }).run(scopes);
        let jump = info.new_value(program).jump(*end);
        info.push_inst(program, jump);
        // push new basic block
        let next = info.new_basic_block(program, None);
        info.push_bb(program, next);
        Ok(())
    }
}

impl<'ast> GenerateProgram<'ast> for Continue {
    type Out = ();

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        // jump to the entry of loop
        let info = &mut current_fun_mut!(scopes);
        
        // scopes.loop_info.last().
        let (entry, _) = 
            (E0005::E0005 { ast: self }).run(scopes);
            
        let jump = info.new_value(program).jump(*entry);
        info.push_inst(program, jump);
        
        // push new basic block
        let next = info.new_basic_block(program, None);
        info.push_bb(program, next);
        Ok(())
    }
}

impl<'ast> GenerateProgram<'ast> for Return {
    type Out = ();

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        if let Some(ret_val) = current_fun!(scopes).ret_val() {
            // generate store
            if let Some(val) = &self.exp {
                let value = val.gen(program, scopes)?.into_int(program, scopes)?;
                let info = current_fun!(scopes);
                let store = info.new_value(program).store(value, ret_val);
                info.push_inst(program, store);
            }
        } else if self.exp.is_some() {
            return (E0004::E0004 { ast: self.exp }).run()
        }
        // jump to the end basic block
        let info = &mut current_fun_mut!(scopes);
        let jump = info.new_value(program).jump(info.end());
        info.push_inst(program, jump);
        // push new basic block
        let next = info.new_basic_block(program, None);
        info.push_bb(program, next);
        Ok(())
    }
}

impl<'ast> GenerateProgram<'ast> for Expr {
    type Out = ExprValue;

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        self.lor.gen(program, scopes)
    }
}

impl<'ast> GenerateProgram<'ast> for LVal {
    type Out = ExprValue;

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        // handle constant
        let mut value = match scopes.value(&self.id.name)? {
            Value::Value(value) => *value,
            Value::Const(num) => {
                // self.indices.is_empty()
                (E0006::E0006 { ast: self }).run(program, scopes, num)?;
            }
        };
        // check type
        let mut is_ptr_ptr = false;
        let mut dims = match scopes.ty(program, value).kind() {
            TypeKind::Pointer(base) => {
                let mut ty = base;
                let mut dims = 0;
                loop {
                    ty = match ty.kind() {
                        TypeKind::Array(base, _) => base,
                        TypeKind::Pointer(base) => {
                            is_ptr_ptr = true;
                            base
                        }
                        _ => break dims,
                    };
                    dims += 1;
                }
            }
            _ => 0,
        };
        // generate load for array parameter
        if is_ptr_ptr {
            let info = current_fun!(scopes);
            value = info.new_value(program).load(value);
            info.push_inst(program, value);
        }
        // handle array dereference
        for (i, index) in self.indices.iter().enumerate() {
            // check if dereferencing integer
            // if dims == 0
            (E0007::E0007 { ast: self }).run(dims)?;
            dims -= 1;
            // generate index
            let index = index.gen(program, scopes)?.into_val(program, scopes)?;
            // generate pointer calculation
            let info = current_fun!(scopes);
            value = if is_ptr_ptr && i == 0 {
                info.new_value(program).get_ptr(value, index)
            } else {
                info.new_value(program).get_elem_ptr(value, index)
            };
            info.push_inst(program, value);
        }
        // generate pointer calculation for funtion arguments
        if dims == 0 {
            Ok(ExprValue::IntPtr(value))
        } else {
            if !is_ptr_ptr || !self.indices.is_empty() {
                let info = current_fun!(scopes);
                let zero = info.new_value(program).integer(0);
                value = info.new_value(program).get_elem_ptr(value, zero);
                info.push_inst(program, value);
            }
            Ok(ExprValue::ArrPtr(value))
        }
    }
}

impl<'ast> GenerateProgram<'ast> for PrimaryExpr {
    type Out = ExprValue;

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        match self {
            Self::Expr(exp) => exp.gen(program, scopes),
            Self::LVal(lval) => lval.gen(program, scopes),
            Self::Number(num) => Ok(ExprValue::Int(
                current_fun!(scopes).new_value(program).integer(num.value),
            )),
        }
    }
}

impl<'ast> GenerateProgram<'ast> for UnaryExpr {
    type Out = ExprValue;

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        match self {
            Self::Primary(exp) => exp.gen(program, scopes),
            Self::Call(call) => call.gen(program, scopes),
            Self::Unary(op, exp) => {
                let exp = exp.gen(program, scopes)?.into_int(program, scopes)?;
                let info = current_fun!(scopes);
                let zero = info.new_value(program).integer(0);
                let value = match op {
                    UnaryOp::Neg(_) => info.new_value(program).binary(BinaryOp::Sub, zero, exp),
                    UnaryOp::LNot(_) => info.new_value(program).binary(BinaryOp::Eq, exp, zero),
                };
                info.push_inst(program, value);
                Ok(ExprValue::Int(value))
            }
        }
    }
}

impl<'ast> GenerateProgram<'ast> for FuncCall {
    type Out = ExprValue;

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        // get funtion from scope
        let fun = scopes.fun(&self.id.name)?;
        // get funtion type
        let (params_ty, is_void) = match program.func(fun).ty().kind() {
            TypeKind::Function(params, ret) => (params.clone(), ret.is_unit()),
            _ => unreachable!(),
        };
        // generate arguments
        let args = self
            .args
            .iter()
            .map(|a| a.gen(program, scopes)?.into_val(program, scopes))
            .collect::<Result<Vec<_>>>()?;
        // check argument types
        // if params_ty.len() != args.len() {
        (E0008::E0008 { ast: self }).run(params_ty, &args)?;
        (E0009::E0009 { ast: self }).run(params_ty, &args, program, scopes)?;

        // generate funtion call
        let info = current_fun!(scopes);
        let call = info.new_value(program).call(fun, args);
        info.push_inst(program, call);
        // return value if not void
        if is_void {
            Ok(ExprValue::Void)
        } else {
            Ok(ExprValue::Int(call))
        }
    }
}

impl<'ast> GenerateProgram<'ast> for MulExpr {
    type Out = ExprValue;

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        match self {
            Self::Unary(exp) => exp.gen(program, scopes),
            Self::MulUnary(lhs, op, rhs) => {
                let lhs = lhs.gen(program, scopes)?.into_int(program, scopes)?;
                let rhs = rhs.gen(program, scopes)?.into_int(program, scopes)?;
                let op = op.gen(program, scopes)?;
                let info = current_fun!(scopes);
                let value = info.new_value(program).binary(op, lhs, rhs);
                info.push_inst(program, value);
                Ok(ExprValue::Int(value))
            }
        }
    }
}

impl<'ast> GenerateProgram<'ast> for AddExpr {
    type Out = ExprValue;

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        match self {
            Self::Mul(exp) => exp.gen(program, scopes),
            Self::AddMul(lhs, op, rhs) => {
                let lhs = lhs.gen(program, scopes)?.into_int(program, scopes)?;
                let rhs = rhs.gen(program, scopes)?.into_int(program, scopes)?;
                let op = op.gen(program, scopes)?;
                let info = current_fun!(scopes);
                let value = info.new_value(program).binary(op, lhs, rhs);
                info.push_inst(program, value);
                Ok(ExprValue::Int(value))
            }
        }
    }
}

impl<'ast> GenerateProgram<'ast> for RelExpr {
    type Out = ExprValue;

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        match self {
            Self::Add(exp) => exp.gen(program, scopes),
            Self::RelAdd(lhs, op, rhs) => {
                let lhs = lhs.gen(program, scopes)?.into_int(program, scopes)?;
                let rhs = rhs.gen(program, scopes)?.into_int(program, scopes)?;
                let op = op.gen(program, scopes)?;
                let info = current_fun!(scopes);
                let value = info.new_value(program).binary(op, lhs, rhs);
                info.push_inst(program, value);
                Ok(ExprValue::Int(value))
            }
        }
    }
}

impl<'ast> GenerateProgram<'ast> for EqExpr {
    type Out = ExprValue;

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        match self {
            Self::Rel(exp) => exp.gen(program, scopes),
            Self::EqRel(lhs, op, rhs) => {
                let lhs = lhs.gen(program, scopes)?.into_int(program, scopes)?;
                let rhs = rhs.gen(program, scopes)?.into_int(program, scopes)?;
                let op = op.gen(program, scopes)?;
                let info = current_fun!(scopes);
                let value = info.new_value(program).binary(op, lhs, rhs);
                info.push_inst(program, value);
                Ok(ExprValue::Int(value))
            }
        }
    }
}

/// Generates logical operators.
macro_rules! generate_logical_ops {
    (
    $lhs:expr, $rhs:expr, $program:expr, $scopes:expr,
    $prefix:literal, $rhs_bb:ident, $end_bb:ident, $tbb:ident, $fbb:ident
  ) => {{
        // generate result
        let result = current_fun!($scopes).new_alloc($program, Type::get_i32(), None);
        // generate left-hand side expression
        let lhs = $lhs.gen($program, $scopes)?.into_int($program, $scopes)?;
        let info = current_fun_mut!($scopes);
        let zero = info.new_value($program).integer(0);
        let lhs = info.new_value($program).binary(BinaryOp::NotEq, lhs, zero);
        info.push_inst($program, lhs);
        let store = info.new_value($program).store(lhs, result);
        info.push_inst($program, store);
        // generate basic blocks and branch
        let $rhs_bb = info.new_basic_block($program, Some(concat!("%", $prefix, "_rhs")));
        let $end_bb = info.new_basic_block($program, Some(concat!("%", $prefix, "_end")));
        let br = info.new_value($program).branch(lhs, $tbb, $fbb);
        info.push_inst($program, br);
        // generate right-hand side expression
        info.push_bb($program, $rhs_bb);
        let rhs = $rhs.gen($program, $scopes)?.into_int($program, $scopes)?;
        let info = current_fun_mut!($scopes);
        let rhs = info.new_value($program).binary(BinaryOp::NotEq, rhs, zero);
        info.push_inst($program, rhs);
        let store = info.new_value($program).store(rhs, result);
        info.push_inst($program, store);
        // generate jump
        let jump = info.new_value($program).jump($end_bb);
        info.push_inst($program, jump);
        info.push_bb($program, $end_bb);
        // generate load
        let load = info.new_value($program).load(result);
        info.push_inst($program, load);
        Ok(ExprValue::Int(load))
    }};
}

impl<'ast> GenerateProgram<'ast> for LAndExpr {
    type Out = ExprValue;

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        match self {
            Self::Eq(exp) => exp.gen(program, scopes),
            Self::LAndEq(lhs, rhs) => generate_logical_ops! {
              lhs, rhs, program, scopes, "land", rhs_bb, end_bb, rhs_bb, end_bb
            },
        }
    }
}

impl<'ast> GenerateProgram<'ast> for LOrExpr {
    type Out = ExprValue;

    fn gen(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        match self {
            Self::LAnd(exp) => exp.gen(program, scopes),
            Self::LOrLAnd(lhs, rhs) => generate_logical_ops! {
              lhs, rhs, program, scopes, "lor", rhs_bb, end_bb, end_bb, rhs_bb
            },
        }
    }
}

impl<'ast> GenerateProgram<'ast> for ConstExpr {
    type Out = i32;

    fn gen(&'ast self, _: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        self.eval(scopes).ok_or(Error::FailedToEval)
    }
}

impl<'ast> GenerateProgram<'ast> for MulOp {
    type Out = BinaryOp;

    fn gen(&'ast self, _: &mut Program, _: &mut Scopes<'ast>) -> Result<Self::Out> {
        Ok(match self {
            MulOp::Mul(_) => BinaryOp::Mul,
            MulOp::Div(_) => BinaryOp::Div,
            MulOp::Mod(_) => BinaryOp::Mod,
        })
    }
}

impl<'ast> GenerateProgram<'ast> for AddOp {
    type Out = BinaryOp;

    fn gen(&'ast self, _: &mut Program, _: &mut Scopes<'ast>) -> Result<Self::Out> {
        Ok(match self {
            AddOp::Add(_) => BinaryOp::Add,
            AddOp::Sub(_) => BinaryOp::Sub,
        })
    }
}

impl<'ast> GenerateProgram<'ast> for RelOp {
    type Out = BinaryOp;

    fn gen(&'ast self, _: &mut Program, _: &mut Scopes<'ast>) -> Result<Self::Out> {
        Ok(match self {
            RelOp::Lt(_) => BinaryOp::Lt,
            RelOp::Gt(_) => BinaryOp::Gt,
            RelOp::Le(_) => BinaryOp::Le,
            RelOp::Ge(_) => BinaryOp::Ge,
        })
    }
}

impl<'ast> GenerateProgram<'ast> for EqOp {
    type Out = BinaryOp;

    fn gen(&'ast self, _: &mut Program, _: &mut Scopes<'ast>) -> Result<Self::Out> {
        Ok(match self {
            EqOp::Eq(_) => BinaryOp::Eq,
            EqOp::Ne(_) => BinaryOp::NotEq,
        })
    }
}
