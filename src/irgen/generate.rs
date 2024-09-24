use koopa::ir::{
    builder::{ BasicBlockBuilder, LocalInstBuilder, ValueBuilder },
    FunctionData,
    Program,
    Type,
};
use crate::{ ast::{ CompUnit, FuncDef, Stmt, Block }, irgen::{ Result, Context } };

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
        let func = program.func_mut(cur_func_id);
        let cur_block_id = ctx.curr_block.unwrap();
        let res_val = func.dfg_mut().new_value().integer(self.num);
        let ret = func.dfg_mut().new_value().ret(Some(res_val));
        func.layout_mut().bb_mut(cur_block_id).insts_mut().push_key_back(ret);

        Ok(())
    }
}
