use super::{ FunctionInfo, InsData };
use crate::asmgen::Context;
use crate::irgen::{ Error, Result };
use koopa::ir::entities::ValueData;
use koopa::ir::{ BasicBlock, BinaryOp, FunctionData, TypeKind, Value, ValueKind };
use std::ops::Deref;
use std::{ fs::File, io::Write };
// koopa IR => ASM
pub trait GenerateAsm {
    fn generate(&self, file: &mut File, ctx: &mut Context) -> Result<Self::Out>;
    type Out;
}

pub trait GenerateInsData<'a> {
    fn generate(&self, ctx: &'a mut Context) -> Result<Self::Out>;
    type Out;
}

impl GenerateAsm for koopa::ir::Program {
    type Out = ();

    fn generate(&self, file: &mut File, ctx: &mut Context) -> Result<Self::Out> {
        for value in self.inst_layout() {
            writeln!(file, "  .data");
            if value.is_global() {
                if let Some(value_data) = self.borrow_values().get(value) {
                    ctx.cur_value = Some(*value);
                    if matches!(value_data.kind(), ValueKind::GlobalAlloc(_)) {
                        value_data.generate(file, ctx)?;
                    }
                }
            }
        }
        for &func in self.func_layout() {
            let func_data: &koopa::ir::FunctionData = self.func(func);
            // 函数声明内部没有block,不需要翻译为机器码,skip;
            // Koopa IR 的函数声明和普通函数的区别是: 函数声明的基本块列表是空的
            if let None = func_data.layout().entry_bb() {
                continue;
            }
            writeln!(file, "  .text");

            let name = func_data.name()[1..].to_string();
            writeln!(file, "  .global {}", name);
            ctx.func = Some(func);
            func_data.generate(file, ctx)?;
            writeln!(file);
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
        inc_reg(file, &"sp".to_string(), -ctx.cur_func_info.as_ref().unwrap().stack_allocation);
        if !ctx.cur_func_info.as_ref().unwrap().is_leaf_func {
            let offset = ctx.cur_func_info.as_ref().unwrap().stack_allocation - 4;
            write_by_offset(file, "ra", "sp", offset);
        }
        for (bb, node) in self.layout().bbs() {
            if let Some(name) = ctx.look_up_label(*bb) {
                writeln!(file, "{}:", name);
            }
            for &inst in node.insts().keys() {
                println!("generating value data {:#?}", inst);
                // 对于每个指令进行代码生成,注意,value无须递归;
                // 及联关系已经体现在IR layout中指令顺序中 例如 %1 = 1 + 1 ， %2 = 1 + 1%
                // 先生成1%,再生成2%;
                let value_data = self.dfg().value(inst);
                ctx.cur_value = Some(inst);
                value_data.generate(file, ctx)?;
            }
        }

        Ok(())
    }
}
impl GenerateAsm for koopa::ir::values::Aggregate {
    type Out = ();
    fn generate(&self, file: &mut File, ctx: &mut Context) -> Result<Self::Out> {
        for ele in self.elems() {
            let sub_value = ctx.prog.borrow_value(*ele);
            match sub_value.kind() {
                ValueKind::Integer(int) => {
                    writeln!(file, "  .word {}", int.value());
                }
                ValueKind::Aggregate(agg) => {
                    agg.generate(file, ctx);
                }
                _ => unreachable!(),
            }
        }
        Ok(())
    }
}
impl GenerateAsm for koopa::ir::entities::ValueData {
    type Out = ();
    fn generate(&self, file: &mut File, ctx: &mut Context) -> Result<Self::Out> {
        match self.kind() {
            ValueKind::GlobalAlloc(global_alloc) => {
                let value = ctx.cur_value.unwrap();
                let value_data = ctx.prog.borrow_value(value);
                if let Some(global_name) = value_data.name().as_ref() {
                    let var_name = global_name[1..].to_string();
                    writeln!(file, "  .globl {}", var_name);
                    writeln!(file, "{}:", var_name);
                    let init = ctx.prog.borrow_value(global_alloc.init());
                    match init.kind() {
                        ValueKind::ZeroInit(_) => {
                            writeln!(file, "  .zero {}", Context::size(value_data.deref()));
                        }
                        ValueKind::Integer(int) => {
                            writeln!(file, "  .word {}", int.value());
                        }
                        ValueKind::Aggregate(agg) => {
                            agg.generate(file, ctx);
                        }
                        _ => unreachable!(),
                    }
                    writeln!(file);
                    ctx.global_value_to_data_name.insert(value, var_name);
                    Ok(())
                } else {
                    Err(Error::UnknownSymbol)
                }
            }

            ValueKind::Integer(_) => {
                Ok(())
                //
            }

            // getPtr的偏移,因为数组下标可能是表达式,编译期间无法确定,所以还是需要用乘法指令算出偏移,
            // ptr计算结果(即一个绝对地址,保存在一个逻辑内存值中)
            ValueKind::GetElemPtr(ptr) => ptr.generate(file, ctx),
            ValueKind::GetPtr(ptr) => ptr.generate(file, ctx),
            ValueKind::Return(ret) => {
                if let Some(value) = ret.value() {
                    let res_val = value.generate(ctx)?;
                    res_val.write_to(file, &"a0".to_string());
                }
                if !ctx.cur_func_info.as_ref().unwrap().is_leaf_func {
                    let ra_addr_offset = ctx.cur_func_info.as_ref().unwrap().stack_allocation - 4;
                    load_by_offset(file, "ra", "sp", ra_addr_offset);
                }
                // write epilogue at ext point
                let stack_space = ctx.cur_func_info.as_ref().unwrap().stack_allocation;
                inc_reg(file, "sp", stack_space);
                writeln!(file, "  ret");
                Ok(())
            }

            ValueKind::Alloc(_) => {
                // nothing happens,allocation on stack at compileTime
                // no corresponding instruction needs to be generated
                Ok(())
            }

            ValueKind::Store(store) => {
                let left_reg = load_to_reg_with_default(file, ctx, store.value(), "t0")?;
                match store.dest().generate(ctx)? {
                    InsData::StackSlot(offset) => {
                        if ctx.is_ptr(store.dest()) {
                            load_by_offset(file, "t1", "sp", offset);
                            write_by_offset(file, left_reg, "t1".to_string(), 0);
                            // writeln!(file, "  lw    t1, {}(sp)", offset);
                            // writeln!(file, "  sw    {}, 0(t1)", left_reg);
                        } else {
                            write_by_offset(file, left_reg, "sp".to_string(), offset);
                        }
                    }
                    InsData::GlobalVar(name) => {
                        writeln!(file, "  la    t1, {}", name);
                        writeln!(file, "  sw    {}, 0(t1)", left_reg);
                    }
                    _ => unreachable!(),
                }

                Ok(())
            }

            ValueKind::Branch(if_else) => if_else.generate(file, ctx),
            ValueKind::Jump(jump) => jump.generate(file, ctx),
            // load指令,获取目标的值,并写入到本指令对应的逻辑内存位置中
            ValueKind::Load(load) => {
                load.src().generate(ctx)?.write_to(file, "t0");
                if ctx.is_ptr(load.src()) {
                    load_by_offset(file, "t0", "s0", 0);
                }

                // load.src().generate(ctx)?.write_address_to(file, is_ptr, dst_reg);
                // match load.src().generate(ctx)? {
                //     InsData::StackSlot(offset) => {
                //         if ctx.is_ptr(load.src()) {
                //             writeln!(file, "  lw    t0, {}(sp)", offset);
                //             writeln!(file, "  lw    t0, 0(t0)");
                //         } else {
                //             writeln!(file, "  lw    t0, {}(sp)", offset);
                //         }
                //     }
                //     InsData::GlobalVar(name) => {
                //         writeln!(file, "  la    t0, {}", name);
                //         writeln!(file, "  lw    t0, 0(t0)");
                //     }
                //     _ => unreachable!(),
                // }
                write_to_dst_value(file, ctx, ctx.cur_value.unwrap(), "t0")?;
                Ok(())
            }
            ValueKind::Binary(binary) => {
                let lhs = binary.lhs();
                let rhs = binary.rhs();
                let left_reg = load_to_reg_with_default(file, ctx, lhs, "t0")?;
                let right_reg = load_to_reg_with_default(file, ctx, rhs, "t1")?;
                generate_op_asm(file, binary.op(), &left_reg, &right_reg, &"t0".into());
                write_to_dst_value(file, ctx, ctx.cur_value.unwrap(), "t0")?;

                Ok(())
            }

            ValueKind::Call(func_call) => {
                for i in 0..func_call.args().len() {
                    if i < 8 {
                        let dst = format!("a{}", i).to_owned();
                        let ins_data = func_call.args()[i].generate(ctx)?;
                        ins_data.write_to(file, dst);
                    } else {
                        let dst = (i - 8) * 4;
                        let ins_data = func_call.args()[i].generate(ctx)?;
                        ins_data.write_to(file, "t0");
                        write_by_offset(file, "t0", "sp", dst as i32);
                    }
                }
                writeln!(
                    file,
                    "  call  {}",
                    ctx.prog.func(func_call.callee()).name()[1..].to_string()
                );
                // 注意funcall可能会返回void,对于unit type,此时不用写回逻辑位置
                if let Ok(_) = ctx.find_value_stack_offset(ctx.cur_value.unwrap()) {
                    write_to_dst_value(file, ctx, ctx.cur_value.unwrap(), "a0")?;
                }
                Ok(())
            }
            _ => {
                println!("{:?}", self);
                unreachable!()
            }
        }
    }
}

impl GenerateAsm for koopa::ir::values::Branch {
    type Out = ();
    fn generate(&self, file: &mut File, ctx: &mut Context) -> Result<Self::Out> {
        let value = self.cond().generate(ctx)?;
        value.write_to(file, &"t0".to_string());
        let true_bb = self.true_bb();
        let false_bb = self.false_bb();
        let mut true_block_name = ctx.cur_func().dfg().bb(true_bb).name().as_ref().unwrap().clone();
        let true_label_name = ctx.register_label(true_bb, label_name(true_block_name));
        writeln!(file, "  bnez {}, {}", "t0", true_label_name);

        let mut false_block_name = ctx
            .cur_func()
            .dfg()
            .bb(false_bb)
            .name()
            .as_ref()
            .unwrap()
            .clone();
        let false_label_name = ctx.register_label(false_bb, label_name(false_block_name));
        writeln!(file, "  j {}", false_label_name);

        Ok(())
    }
}

impl GenerateAsm for koopa::ir::values::Jump {
    type Out = ();
    fn generate(&self, file: &mut File, ctx: &mut Context) -> Result<Self::Out> {
        let func_data = ctx.cur_func();
        let label_name = if let None = ctx.look_up_label(self.target()) {
            let target_block_name: &Option<String> = func_data.dfg().bb(self.target()).name();
            ctx.register_label(
                self.target(),
                label_name(target_block_name.as_ref().unwrap().clone())
            )
        } else {
            ctx.look_up_label(self.target()).unwrap()
        };
        writeln!(file, "  j {}", label_name);
        Ok(())
    }
}
impl GenerateAsm for koopa::ir::values::GetPtr {
    type Out = ();
    fn generate(&self, file: &mut File, ctx: &mut Context) -> Result<Self::Out> {
        /*
            addi t0, sp, 4
        # 计算 getelemptr 的偏移量
         li t1, 1
        li t2, 4
        mul t1, t1, t2
        # 计算 getelemptr 的结果
         add t0, t0, t1
             */

        // 读取Src的地址,写到t0寄存器(基址)
        let is_ptr = ctx.is_ptr(self.src());
        let src_data = self.src().generate(ctx)?;
        src_data.write_address_to(file, is_ptr, "t0");

        self.index().generate(ctx)?.write_to(file, &"t1".to_string());

        let cur_value = ctx.cur_func().dfg().value(ctx.cur_value.unwrap());
        let size = match cur_value.ty().kind() {
            TypeKind::Pointer(base) => base.size(),
            _ => unreachable!("ptr op to non-pointer type"),
        };
        writeln!(file, "  li   t2 , {}", size);
        writeln!(file, "  mul t1, t1, t2");
        writeln!(file, "  add t0, t0, t1");

        write_to_dst_value(file, ctx, ctx.cur_value.unwrap(), "t0")?;
        Ok(())
    }
}
impl GenerateAsm for koopa::ir::values::GetElemPtr {
    type Out = ();
    fn generate(&self, file: &mut File, ctx: &mut Context) -> Result<Self::Out> {
        /*
            addi t0, sp, 4
        # 计算 getelemptr 的偏移量
         li t1, 1
        li t2, 4
        mul t1, t1, t2
        # 计算 getelemptr 的结果
         add t0, t0, t1
             */

        // 读取Src的地址,写到t0寄存器(基址)
        let is_ptr = ctx.is_ptr(self.src());
        let src_data = self.src().generate(ctx)?;
        src_data.write_address_to(file, is_ptr, "t0");

        self.index().generate(ctx)?.write_to(file, &"t1".to_string());

        let cur_value = ctx.cur_func().dfg().value(ctx.cur_value.unwrap());
        let size = match cur_value.ty().kind() {
            TypeKind::Pointer(base) => base.size(),
            _ => unreachable!(),
        };
        writeln!(file, "  li   t2 , {}", size);
        writeln!(file, "  mul t1, t1, t2");
        writeln!(file, "  add t0, t0, t1");

        write_to_dst_value(file, ctx, ctx.cur_value.unwrap(), "t0")?;
        Ok(())
    }
}

//返回当前value在作为别的指令的操作数时，如何生成代码
impl<'a> GenerateInsData<'a> for koopa::ir::Value {
    type Out = InsData<'a>;
    fn generate(&self, ctx: &'a mut Context) -> Result<Self::Out> {
        if ctx.is_global_value(self) {
            let value_data = ctx.prog.borrow_value(*self);
            if let ValueKind::GlobalAlloc(_) = value_data.kind() {
                let asm_name = ctx.global_value_to_data_name.get(self).unwrap();
                return Ok(InsData::GlobalVar(&asm_name));
            } else {
                unreachable!();
            }
        }
        let func_data = ctx.prog.func(ctx.func.unwrap());
        let value_data = func_data.dfg().value(*self);
        match value_data.kind() {
            ValueKind::Integer(v) => Ok(InsData::Int(v.value())),
            // 理论上来说可以先处理prologue然后这里就不用加本函数栈的偏移？再想想！
            ValueKind::FuncArgRef(func_arg) => {
                if func_arg.index() < 8 {
                    Ok(InsData::Reg(format!("a{}", func_arg.index())))
                } else {
                    Ok(
                        InsData::StackSlot(
                            4 * ((func_arg.index() - 8) as i32) +
                                ctx.cur_func_info.as_ref().unwrap().stack_allocation
                        )
                    )
                }
            }
            // global_alloc在此前分支中返回
            ValueKind::GlobalAlloc(_) => { unreachable!() }
            // 否则返回自身在栈上的偏移量
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
    fn is_ptr(&self, value: Value) -> bool {
        if self.is_global_value(&value) {
            let value_data = self.prog.borrow_value(value);
            return (
                matches!(value_data.ty().kind(), TypeKind::Pointer(_)) &&
                !matches!(value_data.kind(), ValueKind::Alloc(_))
            );
        } else {
            let value_data = self.cur_func().dfg().value(value);
            return (
                matches!(value_data.ty().kind(), TypeKind::Pointer(_)) &&
                !matches!(value_data.kind(), ValueKind::Alloc(_))
            );
        }
    }
    fn is_global_value(&self, value: &Value) -> bool {
        self.global_value_to_data_name.get(value).is_some()
    }
    // 扫描函数的所有指令,按需在栈上分配内存
    fn alloc_on_stack(&mut self, func_data: &FunctionData) {
        let mut offset = 0;
        let mut is_leaf_func = true;
        let mut longest_call_func_args = 0;

        // 处理call参数超过8个时自己的栈帧
        for (_, bbd) in func_data.layout().bbs().iter() {
            for (&val, _) in bbd.insts() {
                let value_data = func_data.dfg().value(val);
                if let ValueKind::Call(call) = value_data.kind() {
                    longest_call_func_args = if call.args().len() > longest_call_func_args {
                        call.args().len()
                    } else {
                        longest_call_func_args
                    };
                    is_leaf_func = false;
                }
            }
        }
        if longest_call_func_args > 8 {
            offset = offset + 4 * (longest_call_func_args - 8);
        }

        for (_, bbd) in func_data.layout().bbs().iter() {
            for (&val, _) in bbd.insts() {
                let value_data = func_data.dfg().value(val);
                // 本条指令需要分配内存,则为返回值
                if self.need_alloc(value_data) {
                    self.value_2_stack_offset.insert(val, offset as i32);
                    println!("alloc value {:#?} at {}", val, offset);
                    offset += Self::size(value_data);
                } else {
                    println!("no alloc for value {:#?} ", val);
                }
            }
        }
        // alloc ra
        if !is_leaf_func {
            offset = offset + 4;
        }
        let stack_allocation = (((offset as f32) / 16.0).ceil() * 16.0) as i32;
        let info = FunctionInfo {
            stack_allocation,
            is_leaf_func,
        };
        self.cur_func_info = Some(info);
    }

    // todo 记一下
    fn need_alloc(&self, value_data: &ValueData) -> bool {
        !value_data.ty().is_unit() || matches!(value_data.kind(), ValueKind::Alloc(_))
    }

    fn size(value_data: &ValueData) -> usize {
        if
            matches!(value_data.kind(), ValueKind::Alloc(_)) ||
            matches!(value_data.kind(), ValueKind::GlobalAlloc(_))
        {
            if let TypeKind::Pointer(base) = value_data.ty().kind() {
                return base.size();
            }
        }
        4
    }

    fn find_value_stack_offset(&self, value: Value) -> Result<i32> {
        println!("look ip value {:#?}", value.clone());
        self.value_2_stack_offset.get(&value).ok_or(Error::SysError).cloned()
    }

    // 我们让对functiondata的变量往往是作为临时变量存在；如果函数中一直存在这个引用，那么相当于一直有program的引用
    // borrow checker 非常烦人
    fn cur_func(&self) -> &FunctionData {
        self.prog.func(self.func.unwrap())
    }

    fn register_label(&mut self, bb: BasicBlock, mut name: String) -> &String {
        name.push_str(self.label_counter.to_string().as_str());
        self.label_counter = self.label_counter + 1;
        self.basic_block_to_label_name.insert(bb, name);
        self.basic_block_to_label_name.get(&bb).unwrap()
    }

    fn register_label_1(&mut self, bb: BasicBlock, name: Option<&String>) -> &String {
        let label_name = match name {
            None => self.label_counter.to_string(),
            Some(name_ref) => {
                let mut label_name = label_name_1(name_ref);
                label_name.push_str(self.label_counter.to_string().as_str());
                label_name
            }
        };
        self.basic_block_to_label_name.insert(bb, label_name);
        self.look_up_label(bb).unwrap()
    }

    fn look_up_label(&self, bb: BasicBlock) -> Option<&String> {
        self.basic_block_to_label_name.get(&bb)
    }
}

fn label_name(str: String) -> String {
    str[1..].to_string()
}

fn label_name_1(str: &String) -> String {
    str[1..].to_string()
}

impl<'a> InsData<'a> {
    fn write_to<T: std::fmt::Display>(&self, file: &mut File, dst_reg: T) {
        match self {
            InsData::StackSlot(offset) => load_by_offset(file, dst_reg, "sp", *offset),
            InsData::Reg(reg) => {
                writeln!(file, "  mv  {}, {}", dst_reg, reg);
            }
            InsData::GlobalVar(name) => {
                writeln!(file, "  la    {}, {}", dst_reg, name);
                writeln!(file, "  lw    {}, 0({})", dst_reg, dst_reg);
            }
            InsData::Int(int) => {
                writeln!(file, "  li    {}, {}", dst_reg, int);
            }
        }
    }

    fn write_address_to<T: std::fmt::Display>(&self, file: &mut File, is_ptr: bool, dst_reg: T) {
        match self {
            InsData::StackSlot(offset) => {
                if is_ptr {
                    load_by_offset(file, dst_reg, "sp", *offset);
                } else {
                    writeln!(file, "  li    {},  {}", dst_reg, offset);
                    writeln!(file, "  add  {} , sp , {}", dst_reg, dst_reg);
                }
            }
            InsData::GlobalVar(name) => {
                writeln!(file, "  la    {}, {}", dst_reg, name);
            }
            _ => unreachable!(),
        }
    }
}

// 目标位置的值写到dst寄存器
fn load_by_offset<T: std::fmt::Display, U: std::fmt::Display>(
    file: &mut File,
    dst: T,
    src_base_address: U,
    offset: i32
) {
    if offset >= -2048 && offset < 2048 {
        writeln!(file, "  lw   {} , {}({})", dst, offset, src_base_address);
    } else {
        writeln!(file, "  li   {} , {}", dst, offset);
        writeln!(file, "  add   {} , {}, {}", dst, src_base_address, dst);
        writeln!(file, "  lw   {} , 0({})", dst, dst);
    }
}

//从src寄存器加载值到目标位置
// 使用T3暂存位置
fn write_by_offset<T: std::fmt::Display>(file: &mut File, src: T, dst_base_addr: T, offset: i32) {
    if offset >= -2048 && offset < 2048 {
        writeln!(file, "  sw   {} , {}({})", src, offset, dst_base_addr);
    } else {
        writeln!(file, "  li   t3 , {}", offset);
        writeln!(file, "  add   t3 , t3, {}", dst_base_addr);
        writeln!(file, "  sw   {} , 0(t3)", src);
    }
}

fn inc_reg<T: std::fmt::Display>(file: &mut File, reg: T, increment: i32) {
    if increment >= -2048 && increment < 2048 {
        writeln!(file, "  addi  {}, {}, {}", reg, reg, increment);
    } else {
        writeln!(file, "  li   t3 , {}", increment);
        writeln!(file, "  add  {}, t3, {}", reg, reg);
    }
}

// 把某个value load到寄存器中,有特殊情况返回特定寄存器,否则返回入参指定的寄存器
fn load_to_reg_with_default(
    file: &mut File,
    ctx: &mut Context,
    value: Value,
    default_reg: &'static str
) -> Result<String> {
    let ins_data = value.generate(ctx)?;
    let reg: String = match ins_data {
        InsData::Int(0) => "x0".to_string(),
        InsData::Reg(reg) => reg,
        _ => {
            ins_data.write_to(file, &default_reg);
            default_reg.into()
        }
    };
    Ok(reg)
}

// 将 src_reg的值写到value处
fn write_to_dst_value(
    file: &mut File,
    ctx: &mut Context,
    value: Value,
    src_reg: &'static str
) -> Result<()> {
    let ins_data = value.generate(ctx)?;
    match ins_data {
        InsData::StackSlot(offset) => {
            write_by_offset(file, src_reg, "sp", offset);
        }
        _ => {}
    }
    Ok(())
}
