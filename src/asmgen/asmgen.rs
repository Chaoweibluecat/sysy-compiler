use std::{ borrow::Borrow, fs::File, io::Write };

use koopa::{ ir::ValueKind };
use crate::asmgen::Context;
use crate::irgen::Result;
// koopa IR => ASM
pub trait GenerateAsm {
    fn generate(&self, file: &mut File, ctx: &mut Context) -> Result<Self::Out>;
    type Out;
}

impl GenerateAsm for koopa::ir::Program {
    type Out = ();

    fn generate(&self, file: &mut File, ctx: &mut Context) -> Result<Self::Out> {
        writeln!(file, "  .text");
        for &func in self.func_layout() {
            let func_data = self.func(func);
            let name = func_data.name()[1..].to_string();
            writeln!(file, "  .global {}", name);
            ctx.func = Some(func);
            func_data.generate(file, ctx);
        }
        Ok(())
    }
}

impl GenerateAsm for koopa::ir::FunctionData {
    type Out = ();
    fn generate(&self, file: &mut File, ctx: &mut Context) -> Result<Self::Out> {
        let name = self.name()[1..].to_string();
        writeln!(file, "{}:", name);
        for (&bb, node) in self.layout().bbs() {
            for &inst in node.insts().keys() {
                let value_data = self.dfg().value(inst);
                value_data.generate(file, ctx);
            }
        }
        Ok(())
    }
}

impl GenerateAsm for koopa::ir::entities::ValueData {
    type Out = ();
    fn generate(&self, file: &mut File, ctx: &mut Context) -> Result<Self::Out> {
        match self.kind() {
            ValueKind::Integer(int) => {
                Ok(())
                //
            }
            ValueKind::Return(ret) => {
                if let Some(value) = ret.value() {
                    let res_val = value.generate(file, ctx).unwrap();
                    writeln!(file, "  li a0, {}", res_val);
                }
                writeln!(file, "  ret");
                Ok(())
            }
            // 其他种类暂时遇不到
            _ => unreachable!(),
        }
    }
}

impl GenerateAsm for koopa::ir::Value {
    type Out = i32;
    fn generate(&self, file: &mut File, ctx: &mut Context) -> Result<Self::Out> {
        let func_data = ctx.prog.func(ctx.func.unwrap());
        let value_data = func_data.dfg().value(*self);
        match value_data.kind() {
            ValueKind::Integer(v) => Ok(v.value()),
            _ => unreachable!(),
        }
    }
}
