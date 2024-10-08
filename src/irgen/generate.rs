use crate::{
    ast::*,
    irgen::{Context, Result}
};
use koopa::ir::{
    builder::{BasicBlockBuilder, LocalInstBuilder, ValueBuilder},
    BinaryOp, FunctionData, Program, Type, Value,
};
pub trait GenerateProgram {
    type Out;

    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out>;
}

impl GenerateProgram for CompUnit {
    type Out = ();

    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        self.func_def.generate(program, ctx);
        Ok(())
    }
}

impl GenerateProgram for FuncDef {
    type Out = ();

    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        let main_handle = program.new_func(FunctionData::with_param_names(
            format!("@{}", self.ident).into(),
            vec![],
            Type::get_i32(),
        ));
        let main = program.func_mut(main_handle);
        ctx.curr_fuc = Some(main_handle);
        let entry1 = main
            .dfg_mut()
            .new_bb()
            .basic_block(Some("%entry".to_string()));
        main.layout_mut().bbs_mut().push_key_back(entry1);

        ctx.curr_block = Some(entry1);
        self.block.generate(program, ctx);

        Ok(())
    }
}
impl GenerateProgram for Block {
    type Out = ();

    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        for ele in self.items.iter() {
            ele.generate(program, ctx);
        }
        Ok(())
    }
}

impl GenerateProgram for BlockItem {
    type Out = ();
    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            BlockItem::Stmt(stmt) => {
                stmt.generate(program, ctx);
            }
            BlockItem::Decl(decl) => {
                decl.generate(program, ctx);
            }
            _ => unimplemented!(),
        }
        Ok(())
    }
}


impl GenerateProgram for Decl {
    type Out = ();
    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            Decl::ConstDecl(const_decl) => {
                const_decl.generate(program, ctx);
            }
        }
        Ok(())
    }
}

impl GenerateProgram for ConstDecl {
    type Out = ();
    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
            for ele in self.def_list.iter() {
                ele.generate(program, ctx);
            }
        Ok(())
    }
}

impl GenerateProgram for ConstDef {
    type Out = ();
    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        self.init_val.
        Ok(())
    }
}

impl GenerateProgram for ConstInitVal {
    type Out = i32;
    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        self.exp.();
        Ok(())
    }
}

impl GenerateProgram for Stmt {
    type Out = ();

    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        let cur_func_id = ctx.curr_fuc.unwrap();
        let cur_block_id = ctx.curr_block.unwrap();
        let res_val = self.exp.generate(program, ctx)?;
        let ret = program
            .func_mut(cur_func_id)
            .dfg_mut()
            .new_value()
            .ret(Some(res_val));
        program
            .func_mut(cur_func_id)
            .layout_mut()
            .bb_mut(cur_block_id)
            .insts_mut()
            .push_key_back(ret)
            .unwrap();
        Ok(())
    }
}

impl GenerateProgram for Exp {
    type Out = Value;

    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            Exp::LOrExp(add_exp) => add_exp.generate(program, ctx),
            _ => unreachable!(),
        }
    }
}

impl Eval for Exp {
    type Out = i32;

    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            Exp::LOrExp(add_exp) => add_exp.generate(program, ctx),
            _ => unreachable!(),
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
                let func_data = program.func_mut(ctx.curr_fuc.unwrap());
                let res = func_data
                    .dfg_mut()
                    .new_value()
                    .binary(koopa_op, left_value, right_value);
                func_data
                    .layout_mut()
                    .bb_mut(ctx.curr_block.unwrap())
                    .insts_mut()
                    .push_key_back(res);
                Ok(res)
            }
        }
    }
}

impl GenerateProgram for MulExp {
    type Out = Value;
    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            MulExp::UnaryExp(unary_Exp) => unary_Exp.generate(program, ctx),
            MulExp::MulExp(left, op, right) => {
                let left_value: Value = left.generate(program, ctx)?;
                let right_value = right.generate(program, ctx)?;
                let func_data: &mut FunctionData = program.func_mut(ctx.curr_fuc.unwrap());

                let koopa_op: BinaryOp = match op {
                    MulOp::Multi => BinaryOp::Mul,
                    MulOp::Divide => BinaryOp::Div,
                    MulOp::Mod => BinaryOp::Mod,
                };
                let res = func_data
                    .dfg_mut()
                    .new_value()
                    .binary(koopa_op, left_value, right_value);

                func_data
                    .layout_mut()
                    .bb_mut(ctx.curr_block.unwrap())
                    .insts_mut()
                    .push_key_back(res);
                Ok(res)
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
            LAndExp::LAndExp(left, op, right) => {
                let left_value: Value = left.generate(program, ctx)?;
                let right_value = right.generate(program, ctx)?;
                let koopa_op: BinaryOp = match op {
                    LAndOp::And => BinaryOp::And,
                };
                register_binary(program, ctx, left_value, right_value, koopa_op)
            }
        }
    }
}

impl GenerateProgram for LOrExp {
    type Out = Value;
    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            LOrExp::LAndExp(and) => and.generate(program, ctx),
            LOrExp::LOrExp(left, op, right) => {
                let left_value: Value = left.generate(program, ctx)?;
                let right_value = right.generate(program, ctx)?;
                let koopa_op: BinaryOp = match op {
                    LOrOp::Or => BinaryOp::Or,
                };
                register_binary(program, ctx, left_value, right_value, koopa_op)
            }
        }
    }
}

impl GenerateProgram for UnaryExp {
    type Out = Value;

    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            UnaryExp::PrimaryExp(prim_exp) => match prim_exp {
                PrimaryExp::Number(num) => {
                    // num作为primary_exp,是一个dfg中的value,但不对应指令
                    let func_data = program.func_mut(ctx.curr_fuc.unwrap());
                    let val = func_data.dfg_mut().new_value().integer(*num);
                    Ok(val)
                }
                PrimaryExp::Exp(exp) => exp.generate(program, ctx),
                PrimaryExp::LVal(lval) => {
                    // mock
                    let func_data = program.func_mut(ctx.curr_fuc.unwrap());
                    let val = func_data.dfg_mut().new_value().integer(1);
                    Ok(val)
                }
            },
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
                            .push_key_back(res);

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
    op: BinaryOp,
) -> Result<Value> {
    let func_data: &mut FunctionData = program.func_mut(ctx.curr_fuc.unwrap());

    let res = func_data.dfg_mut().new_value().binary(op, left, right);

    func_data
        .layout_mut()
        .bb_mut(ctx.curr_block.unwrap())
        .insts_mut()
        .push_key_back(res);
    Ok(res)
}
