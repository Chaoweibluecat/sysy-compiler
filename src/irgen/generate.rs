use crate::{
    ast::{Block, CompUnit, Exp, FuncDef, PrimaryExp, Stmt, UnaryExp, UnaryOp},
    irgen::{Context, Result},
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
        self.stmt.generate(program, ctx);
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
            Exp::UnaryExp(unary_exp) => unary_exp.generate(program, ctx),
            _ => unreachable!(),
        }
    }
}

impl GenerateProgram for UnaryExp {
    type Out = Value;

    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            UnaryExp::PrimaryExp(prim_exp) => match prim_exp {
                PrimaryExp::Number(num) => {
                    let func_data = program.func_mut(ctx.curr_fuc.unwrap());
                    let val = func_data.dfg_mut().new_value().integer(*num);
                    Ok(val)
                }
                PrimaryExp::Exp(exp) => exp.generate(program, ctx),
            },
            UnaryExp::UnaryExp(op, rexp) => {
                let rhs = rexp.generate(program, ctx)?;
                match op {
                    UnaryOp::POSITIVE => Ok(rhs),
                    _ => {
                        let func_data = program.func_mut(ctx.curr_fuc.unwrap());
                        let koopa_op = match op {
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
