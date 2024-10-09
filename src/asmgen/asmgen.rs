use std::{fs::File, io::Write};

use super::InsData;
use crate::asmgen::Context;
use crate::irgen::{Error, Result};
use koopa::ir::{BinaryOp, Value, ValueKind};
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
            let func_data: &koopa::ir::FunctionData = self.func(func);
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
                ctx.register_value(inst);
                // 对于每个指令进行代码生成,注意,value无须递归;
                // 及联关系已经体现在IR中 例如 %1 = 1 + 1 ， %2 = 1 + 1%
                // 先生成1%,再生成2%;
                // 2的生成需要知道%1对于的value放到了哪个寄存器里
                // 这一步不能放到value_data的generate过程中(因为valueData数据结构不包含value),
                // 所以放到外面分类
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
                    match res_val {
                        InsData::Int(i) => {
                            writeln!(file, "  li    a0, {}", i);
                        }
                        InsData::TempResult(temp) => {
                            writeln!(file, "  mv a0, {}", temp);
                        }
                    }
                }
                writeln!(file, "  ret");
                Ok(())
            }
            ValueKind::Binary(binary) => {
                let lhs = binary.lhs();
                let rhs = binary.rhs();
                let left = lhs.generate(file, ctx)?;
                let right = rhs.generate(file, ctx)?;

                let left_reg = match left {
                    InsData::Int(i) => {
                        if i != 0 {
                            writeln!(file, "  li    t0, {}", i);
                            "t0".into()
                        } else {
                            "x0".into()
                        }
                    }
                    InsData::TempResult(reg) => reg,
                };

                let right_reg = match right {
                    InsData::Int(i) => {
                        writeln!(file, "  li    t1, {}", i);
                        "t1".into()
                    }
                    InsData::TempResult(reg) => reg,
                };
                let result = ctx.curr_reg()?;
                generate_op_asm(file, binary.op(), &left_reg, &right_reg, &result);

                Ok(())
            }
            // 其他种类暂时遇不到
            _ => unreachable!(),
        }
    }
}

impl GenerateAsm for koopa::ir::Value {
    type Out = InsData;
    fn generate(&self, file: &mut File, ctx: &mut Context) -> Result<Self::Out> {
        let func_data = ctx.prog.func(ctx.func.unwrap());
        let value_data = func_data.dfg().value(*self);
        match value_data.kind() {
            ValueKind::Integer(v) => Ok(InsData::Int(v.value())),
            _ => {
                let reg = ctx.find_value_ref(*self)?;
                Ok(InsData::TempResult(reg.clone()))
            }
        }
    }
}

pub fn generate_op_asm(
    file: &mut File,
    binary_op: BinaryOp,
    left: &String,
    right: &String,
    result: &String,
) {
    match binary_op {
        BinaryOp::Sub => {
            writeln!(file, "  sub   {}, {}, {}", result, left, right);
        }
        BinaryOp::Eq => {
            writeln!(file, "  xor   {}, {}, {}", result, left, right);
            writeln!(file, "  seqz  {}, {}", result, result);
        }
        BinaryOp::NotEq => {
            writeln!(file, "  xor   {}, {}, {}", result, left, right);
            writeln!(file, "  snez  {}, {}", result, result);
        }
        BinaryOp::Mul => {
            writeln!(file, "  mul   {}, {}, {}", result, left, right);
        }
        BinaryOp::Div => {
            writeln!(file, "  div   {}, {}, {}", result, left, right);
        }
        BinaryOp::Mod => {
            writeln!(file, "  rem   {}, {}, {}", result, left, right);
        }
        BinaryOp::Add => {
            writeln!(file, "  add   {}, {}, {}", result, left, right);
        }
        BinaryOp::Lt => {
            writeln!(file, "  slt   {}, {}, {}", result, left, right);
        }
        BinaryOp::Gt => {
            writeln!(file, "  sgt   {}, {}, {}", result, left, right);
        }
        BinaryOp::Le => {
            // <= => !(<)
            writeln!(file, "  sgt   {}, {}, {}", result, left, right);
            // 0->1,1->0 使用seqz
            writeln!(file, "  seqz  {}, {}", result, result);
        }
        BinaryOp::Ge => {
            writeln!(file, "  slt   {}, {}, {}", result, left, right);
            writeln!(file, "  seqz  {}, {}", result, result);
        }

        BinaryOp::And => {
            writeln!(file, "  and   {}, {}, {}", result, left, right);
        }

        BinaryOp::Or => {
            writeln!(file, "  or    {}, {}, {}", result, left, right);
        }
        _ => unreachable!(),
    }
}

impl<'a> Context<'a> {
    fn next_reg(&mut self) -> Result<i32> {
        self.curr_reg = self.curr_reg + 1;
        if self.curr_reg >= 14 {
            Err(Error::SysError)
        } else {
            Ok(self.curr_reg)
        }
    }

    fn map_num(&self, reg_num: i32) -> Result<String> {
        if reg_num < 7 {
            Ok(format!("t{}", reg_num))
        } else if reg_num < 14 {
            Ok(format!("a{}", reg_num - 7))
        } else {
            unreachable!()
        }
    }

    fn curr_reg(&self) -> Result<String> {
        self.map_num(self.curr_reg)
    }

    fn find_value_ref(&self, value: Value) -> Result<String> {
        let reg = self.value_2_regs.get(&value).unwrap();
        self.map_num(*reg)
    }

    fn register_value(&mut self, value: Value) -> Result<()> {
        let reg_num = self.next_reg()?;
        self.value_2_regs.insert(value, reg_num);
        Ok(())
    }
}
