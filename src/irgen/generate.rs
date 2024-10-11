use super::{ eval::Eval, ASTValue, Error };
use crate::{ ast::*, irgen::{ Context, Result } };
use koopa::ir::{
    builder::{ BasicBlockBuilder, LocalInstBuilder, ValueBuilder },
    BinaryOp,
    FunctionData,
    Program,
    Type,
    Value,
};
pub trait GenerateProgram {
    type Out;

    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out>;
}

impl GenerateProgram for CompUnit {
    type Out = ();

    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        self.func_def.generate(program, ctx)?;
        Ok(())
    }
}

impl GenerateProgram for FuncDef {
    type Out = ();

    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        let main_handle = program.new_func(
            FunctionData::with_param_names(
                format!("@{}", self.ident).into(),
                vec![],
                Type::get_i32()
            )
        );
        let main = program.func_mut(main_handle);
        ctx.curr_fuc = Some(main_handle);
        let entry1 = main.dfg_mut().new_bb().basic_block(Some("%entry".to_string()));
        main
            .layout_mut()
            .bbs_mut()
            .push_key_back(entry1)
            .map_err(|_| Error::SysError)?;

        ctx.curr_block = Some(entry1);
        self.block.generate(program, ctx)?;

        Ok(())
    }
}
impl GenerateProgram for Block {
    type Out = ();

    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        ctx.new_scope();
        for ele in self.items.iter() {
            ele.generate(program, ctx)?;
        }
        ctx.leave_scope();
        Ok(())
    }
}

impl GenerateProgram for BlockItem {
    type Out = ();
    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            BlockItem::Stmt(stmt) => {
                stmt.generate(program, ctx)?;
            }
            BlockItem::Decl(decl) => {
                decl.generate(program, ctx)?;
            }
        }
        Ok(())
    }
}

impl GenerateProgram for Decl {
    type Out = ();
    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            Decl::ConstDecl(const_decl) => {
                const_decl.generate(program, ctx)?;
            }
            Decl::VarDecl(var_decl) => {
                var_decl.generate(program, ctx)?;
            }
        }
        Ok(())
    }
}

impl GenerateProgram for VarDecl {
    type Out = ();
    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        for def in self.def_list.iter() {
            def.generate(program, ctx)?;
        }
        Ok(())
    }
}

impl GenerateProgram for VarDef {
    type Out = ();
    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            VarDef::IdOnly(id) => {
                let prev_def = ctx.look_up_in_curr_scope(id);
                match prev_def {
                    Some(_) => Err(Error::DuplicateDecl),
                    None => {
                        let func_data = program.func_mut(ctx.curr_fuc.unwrap());
                        let alloc = func_data.dfg_mut().new_value().alloc(Type::get_i32());
                        func_data.dfg_mut().set_value_name(alloc, Some(format!("@{}", id)));
                        func_data
                            .layout_mut()
                            .bb_mut(ctx.curr_block.unwrap())
                            .insts_mut()
                            .push_key_back(alloc)
                            .map_err(|_| Error::SysError)?;
                        ctx.insert_symbol(&id, ASTValue::Variable(alloc));
                        Ok(())
                    }
                }
            }
            VarDef::Assign(id, init_val) => {
                let prev_def = ctx.look_up_in_curr_scope(id);
                match prev_def {
                    Some(_) => Err(Error::DuplicateDecl),
                    None => {
                        let exp_val = init_val.generate(program, ctx)?;
                        let func_data = program.func_mut(ctx.curr_fuc.unwrap());
                        let alloc = func_data.dfg_mut().new_value().alloc(Type::get_i32());
                        func_data.dfg_mut().set_value_name(alloc, Some(format!("@{}", id)));
                        let store_ins = func_data.dfg_mut().new_value().store(exp_val, alloc);
                        func_data
                            .layout_mut()
                            .bb_mut(ctx.curr_block.unwrap())
                            .insts_mut()
                            .extend([alloc, store_ins]);
                        ctx.insert_symbol(&format!("{}", id).to_owned(), ASTValue::Variable(alloc));
                        Ok(())
                    }
                }
            }
        }
    }
}

impl GenerateProgram for InitVal {
    type Out = Value;
    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        self.exp.generate(program, ctx)
    }
}

impl GenerateProgram for ConstDecl {
    type Out = ();
    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        for ele in self.def_list.iter() {
            ele.generate(program, ctx)?;
        }
        Ok(())
    }
}

impl GenerateProgram for ConstDef {
    type Out = ();
    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        let eval_val = self.init_val.generate(program, ctx)?;
        ctx.symbol_table.insert(self.id.clone(), ASTValue::Const(eval_val));
        Ok(())
    }
}

impl GenerateProgram for ConstInitVal {
    type Out = i32;
    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        self.exp.generate(program, ctx)
    }
}

impl GenerateProgram for ConstExp {
    type Out = i32;
    fn generate(&self, _: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        self.exp.eval(ctx)
    }
}

impl GenerateProgram for Stmt {
    type Out = ();

    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        let cur_func_id = ctx.curr_fuc.unwrap();
        let cur_block_id = ctx.curr_block.unwrap();
        match self {
            Stmt::Ret(exp) => {
                let res_val = exp.generate(program, ctx)?;
                let ret = program.func_mut(cur_func_id).dfg_mut().new_value().ret(Some(res_val));
                program
                    .func_mut(cur_func_id)
                    .layout_mut()
                    .bb_mut(cur_block_id)
                    .insts_mut()
                    .push_key_back(ret)
                    .unwrap();
                Ok(())
            }

            Stmt::Assign(lval, exp) => {
                let sym_val = ctx.look_up_symbol(&lval.id);
                match sym_val {
                    Some(ASTValue::Const(_)) => Err(Error::RedefineConstValue),
                    None => Err(Error::UnknownSymbol),
                    Some(ASTValue::Variable(lval)) => {
                        let local_lval = lval.clone();
                        let rval = exp.generate(program, ctx)?;
                        let func_data = program.func_mut(ctx.curr_fuc.unwrap());
                        let store_ins = func_data.dfg_mut().new_value().store(rval, local_lval);
                        func_data
                            .layout_mut()
                            .bb_mut(ctx.curr_block.unwrap())
                            .insts_mut()
                            .extend([store_ins]);
                        Ok(())
                    }
                }
            }
            Stmt::Exp(exp) => {
                exp.as_ref().map(|e| { e.generate(program, ctx) });
                Ok(())
            }
            Stmt::Block(block) => { block.generate(program, ctx) }
            _ => unimplemented!(),
        }
    }
}

impl GenerateProgram for Exp {
    type Out = Value;

    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            Exp::LOrExp(add_exp) => add_exp.generate(program, ctx),
        }
    }
}

impl GenerateProgram for AddExp {
    type Out = Value;
    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            AddExp::MulExp(mul_exp) => mul_exp.generate(program, ctx),
            AddExp::AddExp(left, op, right) => {
                let left_value = left.generate(program, ctx)?;
                let right_value = right.generate(program, ctx)?;
                let koopa_op: BinaryOp = match op {
                    AddOp::Add => BinaryOp::Add,
                    AddOp::Minus => BinaryOp::Sub,
                };
                register_binary(program, ctx, left_value, right_value, koopa_op)
            }
        }
    }
}

impl GenerateProgram for MulExp {
    type Out = Value;
    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            MulExp::UnaryExp(unary_exp) => unary_exp.generate(program, ctx),
            MulExp::MulExp(left, op, right) => {
                let left_value: Value = left.generate(program, ctx)?;
                let right_value = right.generate(program, ctx)?;
                let koopa_op: BinaryOp = match op {
                    MulOp::Multi => BinaryOp::Mul,
                    MulOp::Divide => BinaryOp::Div,
                    MulOp::Mod => BinaryOp::Mod,
                };
                register_binary(program, ctx, left_value, right_value, koopa_op)
            }
        }
    }
}

impl GenerateProgram for RelExp {
    type Out = Value;
    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            RelExp::AddExp(add) => add.generate(program, ctx),
            RelExp::RelExp(left, op, right) => {
                let left_value: Value = left.generate(program, ctx)?;
                let right_value = right.generate(program, ctx)?;
                let koopa_op: BinaryOp = match op {
                    RelOp::Ge => BinaryOp::Ge,
                    RelOp::Gt => BinaryOp::Gt,
                    RelOp::Le => BinaryOp::Le,
                    RelOp::Lt => BinaryOp::Lt,
                };
                register_binary(program, ctx, left_value, right_value, koopa_op)
            }
        }
    }
}

impl GenerateProgram for EqExp {
    type Out = Value;
    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            EqExp::RelExp(rel) => rel.generate(program, ctx),
            EqExp::EqExp(left, op, right) => {
                let left_value: Value = left.generate(program, ctx)?;
                let right_value = right.generate(program, ctx)?;
                let koopa_op: BinaryOp = match op {
                    EqOp::Eq => BinaryOp::Eq,
                    EqOp::Ne => BinaryOp::NotEq,
                };
                register_binary(program, ctx, left_value, right_value, koopa_op)
            }
        }
    }
}

impl GenerateProgram for LAndExp {
    type Out = Value;
    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            LAndExp::EqExp(eq) => eq.generate(program, ctx),
            LAndExp::LAndExp(left, _, right) => {
                let left_value: Value = left.generate(program, ctx)?;
                let right_value = right.generate(program, ctx)?;
                let func_data = program.func_mut(ctx.curr_fuc.unwrap());
                let zero = func_data.dfg_mut().new_value().integer(0);
                let left_bool = register_binary(program, ctx, left_value, zero, BinaryOp::NotEq)?;
                let right_bool = register_binary(program, ctx, right_value, zero, BinaryOp::NotEq)?;
                register_binary(program, ctx, left_bool, right_bool, BinaryOp::And)
            }
        }
    }
}

impl GenerateProgram for LOrExp {
    type Out = Value;
    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            LOrExp::LAndExp(and) => and.generate(program, ctx),
            LOrExp::LOrExp(left, _, right) => {
                let left_value: Value = left.generate(program, ctx)?;
                let right_value = right.generate(program, ctx)?;
                let func_data = program.func_mut(ctx.curr_fuc.unwrap());
                let zero = func_data.dfg_mut().new_value().integer(0);
                let left_bool = register_binary(program, ctx, left_value, zero, BinaryOp::NotEq)?;
                let right_bool = register_binary(program, ctx, right_value, zero, BinaryOp::NotEq)?;
                register_binary(program, ctx, left_bool, right_bool, BinaryOp::Add)
            }
        }
    }
}

impl GenerateProgram for UnaryExp {
    type Out = Value;

    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            UnaryExp::PrimaryExp(prim_exp) =>
                match prim_exp {
                    PrimaryExp::Number(num) => {
                        // num作为primary_exp,是一个dfg中的value,但不对应指令
                        let func_data = program.func_mut(ctx.curr_fuc.unwrap());
                        let val = func_data.dfg_mut().new_value().integer(*num);
                        Ok(val)
                    }
                    PrimaryExp::Exp(exp) => exp.generate(program, ctx),
                    PrimaryExp::LVal(lval) => {
                        // lval作为表达式（即出现在等号右边时),此时需要求值
                        // 1. name => 内存位置 (查env//符号表)
                        // 2. 内存位置 => 内存值 (koopa中的load, 读出val作为指针实际指向的栈值，作为表达式的返回)
                        let value: Option<&ASTValue> = ctx.look_up_symbol(&lval.id);
                        match value {
                            None => Err(super::Error::UnknownSymbol),
                            Some(ASTValue::Const(val)) => {
                                // 表达式中的左值,如果是常量,直接取解析结果
                                let func_data: &mut FunctionData = program.func_mut(
                                    ctx.curr_fuc.unwrap()
                                );
                                Ok(func_data.dfg_mut().new_value().integer(*val))
                            }
                            Some(ASTValue::Variable(lval)) => {
                                let func_data: &mut FunctionData = program.func_mut(
                                    ctx.curr_fuc.unwrap()
                                );
                                let load = func_data.dfg_mut().new_value().load(*lval);
                                func_data
                                    .layout_mut()
                                    .bb_mut(ctx.curr_block.unwrap())
                                    .insts_mut()
                                    .push_key_back(load)
                                    .map_err(|_| Error::SysError)?;
                                Ok(load)
                            }
                        }
                    }
                }
            UnaryExp::UnaryExp(op, rexp) => {
                let rhs = rexp.generate(program, ctx)?;
                match op {
                    // 一元加号直接丢弃
                    UnaryOp::POSITIVE => Ok(rhs),
                    _ => {
                        let func_data = program.func_mut(ctx.curr_fuc.unwrap());
                        let koopa_op: BinaryOp = match op {
                            UnaryOp::NEGATIVE => BinaryOp::Sub,
                            UnaryOp::NOT => BinaryOp::Eq,
                            _ => unreachable!(),
                        };
                        let zero = func_data.dfg_mut().new_value().integer(0);
                        let res = func_data.dfg_mut().new_value().binary(koopa_op, zero, rhs);
                        func_data
                            .layout_mut()
                            .bb_mut(ctx.curr_block.unwrap())
                            .insts_mut()
                            .push_key_back(res)
                            .map_err(|_| Error::SysError)?;

                        Ok(res)
                    }
                }
            }
        }
    }
}

fn register_binary(
    program: &mut Program,
    ctx: &mut Context,
    left: Value,
    right: Value,
    op: BinaryOp
) -> Result<Value> {
    let func_data: &mut FunctionData = program.func_mut(ctx.curr_fuc.unwrap());
    let res = func_data.dfg_mut().new_value().binary(op, left, right);
    func_data
        .layout_mut()
        .bb_mut(ctx.curr_block.unwrap())
        .insts_mut()
        .push_key_back(res)
        .map_err(|_| Error::SysError)?;
    Ok(res)
}
