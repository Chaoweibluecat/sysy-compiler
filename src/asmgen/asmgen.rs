use super::{ FunctionInfo, InsData };
use crate::asmgen::Context;
use crate::irgen::{ Error, Result };
use koopa::ir::entities::ValueData;
use koopa::ir::{ BasicBlock, BinaryOp, FunctionData, TypeKind, Value, ValueKind };
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
        writeln!(file, "  addi  sp, sp, -{}", ctx.cur_func_info.as_ref().unwrap().stack_allocation);

        if !ctx.cur_func_info.as_ref().unwrap().is_leaf_func {
            writeln!(
                file,
                "    sw  ra, {}(sp)",
                ctx.cur_func_info.as_ref().unwrap().stack_allocation - 4
            );
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
                            writeln!(file, "  .zero {}", value_data.ty().size());
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
                    match res_val {
                        InsData::Int(i) => {
                            writeln!(file, "  li    a0, {}", i);
                        }
                        InsData::StackSlot(offset) => {
                            writeln!(file, "  lw    a0, {}(sp)", offset);
                        }
                        _ => unimplemented!(),
                    }
                }
                if !ctx.cur_func_info.as_ref().unwrap().is_leaf_func {
                    writeln!(
                        file,
                        "    lw  ra, {}(sp)",
                        ctx.cur_func_info.as_ref().unwrap().stack_allocation - 4
                    );
                }
                // write epilogue at ext point
                writeln!(
                    file,
                    "  addi  sp, sp,  {}",
                    ctx.cur_func_info.as_ref().unwrap().stack_allocation
                );
                writeln!(file, "  ret");
                Ok(())
            }

            ValueKind::Alloc(_) => {
                // nothing happens,allocation on stack at compileTime
                // no corresponding instruction needs to be generated
                Ok(())
            }

            ValueKind::Store(store) => {
                let left_reg: String = match store.value().generate(ctx)? {
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
                    InsData::Reg(reg) => reg,
                    _ => unimplemented!(),
                };
                match store.dest().generate(ctx)? {
                    InsData::StackSlot(offset) => {
                        if ctx.is_ptr(store.dest()) {
                            writeln!(file, "  lw    t1, {}(sp)", offset);
                            writeln!(file, "  sw    {}, 0(t1)", left_reg);
                        } else {
                            writeln!(file, "  sw    {}, {}(sp)", left_reg, offset);
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
                match load.src().generate(ctx)? {
                    InsData::StackSlot(offset) => {
                        if ctx.is_ptr(load.src()) {
                            writeln!(file, "  lw    t0, {}(sp)", offset);
                            writeln!(file, "  lw    t0, 0(t0)");
                        } else {
                            writeln!(file, "  lw    t0, {}(sp)", offset);
                        }
                    }
                    InsData::GlobalVar(name) => {
                        writeln!(file, "  la    t0, {}", name);
                        writeln!(file, "  lw    t0, 0(t0)");
                    }
                    _ => unreachable!(),
                }

                if let InsData::StackSlot(self_offset) = ctx.cur_value.unwrap().generate(ctx)? {
                    writeln!(file, "  sw    t0, {}(sp)", self_offset);
                }
                Ok(())
            }
            ValueKind::Binary(binary) => {
                let lhs = binary.lhs();
                let rhs = binary.rhs();
                let left = lhs.generate(ctx)?;

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
                    _ => unimplemented!(),
                };
                let right = rhs.generate(ctx)?;

                let right_reg = match right {
                    InsData::Int(i) => {
                        writeln!(file, "  li    t1, {}", i);
                        "t1".into()
                    }
                    InsData::StackSlot(offset) => {
                        writeln!(file, "  lw    t1, {}(sp)", offset);
                        "t1".into()
                    }

                    _ => unimplemented!(),
                };
                let result = "t0".into();
                generate_op_asm(file, binary.op(), &left_reg, &right_reg, &result);
                if let InsData::StackSlot(offset) = ctx.cur_value.unwrap().generate(ctx)? {
                    writeln!(file, "  sw    t0, {}(sp)", offset);
                }
                Ok(())
            }

            ValueKind::Call(func_call) => {
                for i in 0..func_call.args().len() {
                    if i < 8 {
                        let dst = format!("a{}", i).to_owned();
                        match func_call.args()[i].generate(ctx)? {
                            InsData::Int(int) => {
                                writeln!(file, "  li    {},  {}", dst, int);
                            }
                            InsData::StackSlot(offset) => {
                                writeln!(file, "  lw    {}, {}(sp)", dst, offset);
                            }
                            _ => unimplemented!(),
                        };
                    } else {
                        let dst = (i - 8) * 4;
                        match func_call.args()[i].generate(ctx)? {
                            InsData::Int(int) => {
                                writeln!(file, "  li    t0,  {}", int);
                                writeln!(file, "  sw    t0,  {}(sp)", dst);
                            }
                            InsData::StackSlot(offset) => {
                                writeln!(file, "  lw    t0, {}(sp)", offset);
                                writeln!(file, "  sw    t0, {}(sp)", dst);
                            }
                            _ => unimplemented!(),
                        };
                    }
                }
                writeln!(
                    file,
                    "  call  {}",
                    ctx.prog.func(func_call.callee()).name()[1..].to_string()
                );
                if let Ok(offset) = ctx.find_value_stack_offset(ctx.cur_value.unwrap()) {
                    writeln!(file, "  sw    a0, {}(sp)", offset);
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
        match value {
            InsData::StackSlot(offset) => {
                writeln!(file, "  lw    t0, {}(sp)", offset);
            }
            InsData::Int(inst_num) => {
                writeln!(file, "  li    t0, {}", inst_num);
            }
            _ => unimplemented!(),
        }
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
        let src_address = self.src().generate(ctx)?;
        if let InsData::StackSlot(offset) = src_address {
            if ctx.is_ptr(self.src()) {
                writeln!(file, "  lw   t0 , {}(sp)", offset);
            } else {
                writeln!(file, "  addi t0, sp, {}", offset);
            }
        } else if let InsData::GlobalVar(name) = src_address {
            writeln!(file, "  la    t0, {}", name);
        }

        self.index().generate(ctx)?.write_to(file, &"t1".to_string());

        let cur_value = ctx.cur_func().dfg().value(ctx.cur_value.unwrap());
        let size = match cur_value.ty().kind() {
            TypeKind::Pointer(base) => base.size(),
            _ => unreachable!("ptr op to non-pointer type"),
        };
        writeln!(file, "  li   t2 , {}", size);
        writeln!(file, "  mul t1, t1, t2");
        writeln!(file, "  add t0, t0, t1");

        let dst = ctx.find_value_stack_offset(ctx.cur_value.unwrap())?;
        writeln!(file, "  sw  t0, {}(sp)", dst);
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
        let src_address = self.src().generate(ctx)?;
        if let InsData::StackSlot(offset) = src_address {
            if ctx.is_ptr(self.src()) {
                writeln!(file, "  lw   t0 , {}(sp)", offset);
            } else {
                writeln!(file, "  addi t0, sp, {}", offset);
            }
        } else if let InsData::GlobalVar(name) = src_address {
            writeln!(file, "  la    t0, {}", name);
        }

        self.index().generate(ctx)?.write_to(file, &"t1".to_string());

        let cur_value = ctx.cur_func().dfg().value(ctx.cur_value.unwrap());
        let size = match cur_value.ty().kind() {
            TypeKind::Pointer(base) => base.size(),
            _ => unreachable!(),
        };
        writeln!(file, "  li   t2 , {}", size);
        writeln!(file, "  mul t1, t1, t2");
        writeln!(file, "  add t0, t0, t1");

        let dst = ctx.find_value_stack_offset(ctx.cur_value.unwrap())?;
        writeln!(file, "  sw  t0, {}(sp)", dst);
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

    fn size(valueData: &ValueData) -> usize {
        if let ValueKind::Alloc(_) = valueData.kind() {
            if let TypeKind::Pointer(base) = valueData.ty().kind() {
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
    fn write_to(&self, file: &mut File, dst_reg: &String) {
        match self {
            InsData::StackSlot(offset) => {
                writeln!(file, "  lw   {} , {}(sp)", dst_reg, offset);
            }
            InsData::Reg(reg) => {
                if reg != dst_reg {
                    writeln!(file, "  lw  {}, {}", dst_reg, reg);
                }
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
}
