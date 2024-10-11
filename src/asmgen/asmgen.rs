use std::{ fs::File, io::Write };

use super::InsData;
use crate::asmgen::Context;
use crate::irgen::{ Error, Result };
use koopa::ir::entities::ValueData;
use koopa::ir::{ entities, BinaryOp, FunctionData, Type, Value, ValueKind };
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
            func_data.generate(file, ctx)?;
        }
        Ok(())
    }
}

impl GenerateAsm for koopa::ir::FunctionData {
    type Out = ();
    fn generate(&self, file: &mut File, ctx: &mut Context) -> Result<Self::Out> {
        let name = self.name()[1..].to_string();
        writeln!(file, "{}:", name);
        ctx.alloc_on_stack(self);
        writeln!(file, "  addi  sp, -{}", ctx.cur_fuc_stack_allocation.unwrap());
        for (_, node) in self.layout().bbs() {
            for &inst in node.insts().keys() {
                println!("generating value data {:#?}", inst);
                // 对于每个指令进行代码生成,注意,value无须递归;
                // 及联关系已经体现在IR中 例如 %1 = 1 + 1 ， %2 = 1 + 1%
                // 先生成1%,再生成2%;
                let value_data = self.dfg().value(inst);
                ctx.cur_value = Some(inst);
                value_data.generate(file, ctx)?;
            }
        }
        Ok(())
    }
}
impl GenerateAsm for koopa::ir::values::Alloc {
    type Out = ();
    fn generate(&self, file: &mut File, ctx: &mut Context) -> Result<Self::Out> {
        Ok(())
        //do nothing
    }
}
impl GenerateAsm for koopa::ir::entities::ValueData {
    type Out = ();
    fn generate(&self, file: &mut File, ctx: &mut Context) -> Result<Self::Out> {
        match self.kind() {
            ValueKind::Integer(_) => {
                Ok(())
                //
            }

            ValueKind::Return(ret) => {
                if let Some(value) = ret.value() {
                    let res_val = value.generate(file, ctx)?;
                    match res_val {
                        InsData::Int(i) => {
                            writeln!(file, "  li    a0, {}", i);
                        }
                        InsData::StackSlot(offset) => {
                            writeln!(file, "  lw    a0, {}(sp)", offset);
                        }
                    }
                }
                // write epilogue at ext point
                writeln!(file, "  addi  sp, {}", ctx.cur_fuc_stack_allocation.unwrap());
                writeln!(file, "  ret");
                Ok(())
            }

            ValueKind::Alloc(_) => {
                // nothing happens,allocation on stack at compileTime
                Ok(())
            }

            ValueKind::Store(store) => {
                let left_reg: String = match store.value().generate(file, ctx)? {
                    InsData::Int(i) => {
                        if i != 0 {
                            writeln!(file, "  li    t0, {}", i);
                            "t0".into()
                        } else {
                            "x0".into()
                        }
                    }
                    InsData::StackSlot(offset) => {
                        writeln!(file, "  lw    t0, {}(sp)", offset);
                        "t0".into()
                    }
                };

                if let InsData::StackSlot(offset) = store.dest().generate(file, ctx)? {
                    writeln!(file, "  sw    {}, {}(sp)", left_reg, offset);
                }
                Ok(())
            }

            ValueKind::Load(load) => {
                if let InsData::StackSlot(offset) = load.src().generate(file, ctx)? {
                    writeln!(file, "  lw    t0, {}(sp)", offset);
                }

                if
                    let InsData::StackSlot(self_offset) = ctx.cur_value
                        .unwrap()
                        .generate(file, ctx)?
                {
                    writeln!(file, "  sw    t0, {}(sp)", self_offset);
                }
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
                    InsData::StackSlot(offset) => {
                        writeln!(file, "  lw    t0, {}(sp)", offset);
                        "t0".into()
                    }
                };

                let right_reg = match right {
                    InsData::Int(i) => {
                        writeln!(file, "  li    t1, {}", i);
                        "t1".into()
                    }
                    InsData::StackSlot(offset) => {
                        writeln!(file, "  lw    t1, {}(sp)", offset);
                        "t1".into()
                    }
                };
                let result = "t0".into();
                generate_op_asm(file, binary.op(), &left_reg, &right_reg, &result);
                if let InsData::StackSlot(offset) = ctx.cur_value.unwrap().generate(file, ctx)? {
                    writeln!(file, "  sw    t0, {}(sp)", offset);
                }
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
            _ => Ok(InsData::StackSlot(ctx.find_value_stack_offset(*self)?)),
        }
    }
}

pub fn generate_op_asm(
    file: &mut File,
    binary_op: BinaryOp,
    left: &String,
    right: &String,
    result: &String
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
    // 扫描函数的所有指令,按需在栈上分配内存
    fn alloc_on_stack(&mut self, func_data: &FunctionData) {
        let mut offset = 0;
        for (_, bbd) in func_data.layout().bbs().iter() {
            for (&val, _) in bbd.insts() {
                // 本条指令需要分配内存,则为返回值
                if self.need_alloc(func_data.dfg().value(val)) {
                    self.value_2_stack_offset.insert(val, offset);
                    println!("alloc value {:#?} at {}", val, offset);
                    offset += 4;
                } else {
                    let var1 = func_data.dfg().value(val);
                    println!("no alloc for value {:#?} ", val);
                }
            }
        }
        self.cur_fuc_stack_allocation = Some((((offset as f32) / 16.0).ceil() * 16.0) as i32);
    }

    // todo 记一下
    fn need_alloc(&self, value_data: &ValueData) -> bool {
        !value_data.ty().is_unit() || matches!(value_data.kind(), ValueKind::Alloc(_))
    }

    fn find_value_stack_offset(&self, value: Value) -> Result<i32> {
        println!("look ip value {:#?}", value.clone());
        self.value_2_stack_offset.get(&value).ok_or(Error::SysError).cloned()
    }
}
