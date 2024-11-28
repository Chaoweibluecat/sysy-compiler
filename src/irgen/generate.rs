use core::borrow;

use super::{ eval::Eval, ASTValue, Error };
use crate::{ ast::*, irgen::{ Context, Result } };
use koopa::ir::{
    builder::{ BasicBlockBuilder, GlobalInstBuilder, LocalInstBuilder, ValueBuilder },
    entities::ValueData,
    layout::BasicBlockNode,
    BasicBlock,
    BinaryOp,
    FunctionData,
    Program,
    Type,
    TypeKind,
    Value,
};
pub trait GenerateProgram {
    type Out;
    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out>;
}

impl GenerateProgram for CompUnit {
    type Out = ();

    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        add_sysy_lib_func(program, ctx);
        for item in &self.items {
            item.generate(program, ctx)?;
        }
        Ok(())
    }
}

impl GenerateProgram for GlobalItem {
    type Out = ();

    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            GlobalItem::FuncDef(func_def) => func_def.generate(program, ctx),
            GlobalItem::Decl(decl) => decl.generate(program, ctx),
        }
    }
}

impl GenerateProgram for FuncFParam {
    type Out = (Option<String>, Type);
    fn generate(&self, _: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        let mut p_type = match self.b_type {
            BType::Int => Type::get_i32(),
        };
        if let Some(array_indices) = &self.array_indices {
            let len = array_indices.len();
            for i in 0..len {
                let cur_len_exp = &array_indices[len - 1 - i];
                let cur_len = cur_len_exp.eval(ctx)?;
                p_type = Type::get_array(p_type, cur_len as usize);
            }
            p_type = Type::get_pointer(p_type);
            println!("p_type {:?}", p_type);
        }
        Ok((Some(format!("@{}", self.name).into()), p_type))
    }
}

impl GenerateProgram for FuncDef {
    type Out = ();

    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        if let Some(_) = ctx.scopes.look_up_func(&self.ident) {
            return Err(Error::DuplicateDecl);
        }
        let func_ret_type = match self.func_type {
            FuncType::Int => Type::get_i32(),
            FuncType::Void => Type::get_unit(),
        };
        let func_params = self.params
            .iter()
            .map(|param| param.generate(program, ctx).unwrap())
            .collect();

        let func = program.new_func(
            FunctionData::with_param_names(
                format!("@{}", self.ident).into(),
                func_params,
                func_ret_type
            )
        );
        ctx.curr_fuc = Some(func);
        ctx.scopes.register_function(&self.ident, func);
        let main = program.func_mut(func);
        let entry1 = main.dfg_mut().new_bb().basic_block(Some("%entry".to_string()));
        push_block(program, ctx, entry1)?;
        ctx.new_scope();
        for i in 0..self.params.len() {
            let val = cur_func_mut(program, ctx).params()[i].clone();
            let param_type = value_data_in_cur_func(program, ctx, val).ty().clone();
            let alloc = cur_func_mut(program, ctx).dfg_mut().new_value().alloc(param_type);
            let store = cur_func_mut(program, ctx).dfg_mut().new_value().store(val, alloc);
            push_back_values_as_ins(program, ctx, vec![alloc, store]);
            ctx.insert_symbol(&self.params[i].name, ASTValue::Variable(alloc));
        }
        self.block.generate(program, ctx)?;

        // 为void类型末尾补充ret指令, sysy的标准ret后必须跟exp;所以void函数都不会以ret结尾
        if let FuncType::Void = self.func_type {
            let ret = cur_func_mut(program, ctx).dfg_mut().new_value().ret(None);
            push_back_value_as_ins(program, ctx, ret)?;
        }
        remove_useless_block(program, ctx);
        ctx.leave_scope();
        ctx.curr_fuc = None;
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
pub trait GenType {
    fn gen_type(&self, ctx: &mut Context) -> Type;
}

impl GenType for Vec<ConstExp> {
    fn gen_type(&self, ctx: &mut Context) -> Type {
        let mut ty = Type::get_i32();
        for i in 0..self.len() {
            let len_i = &self[self.len() - 1 - i];
            let parsed_len = len_i.eval(ctx).unwrap();
            ty = Type::get_array(ty, parsed_len as usize);
        }
        ty
    }
}
impl GenerateProgram for VarDef {
    type Out = ();
    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            VarDef::IdOnly(id, len) => {
                let prev_def = ctx.look_up_in_curr_scope(id);
                match prev_def {
                    Some(_) => Err(Error::DuplicateDecl),
                    None => {
                        let alloc = if ctx.in_global_scope() {
                            let ty = len.gen_type(ctx);
                            let init = program.new_value().zero_init(ty);
                            let alloc = program.new_value().global_alloc(init);
                            program.set_value_name(alloc, Some(format!("@{}", id)));
                            alloc
                        } else {
                            let func_data = program.func_mut(ctx.curr_fuc.unwrap());
                            let ty = len.gen_type(ctx);
                            let alloc = func_data.dfg_mut().new_value().alloc(ty);
                            func_data.dfg_mut().set_value_name(alloc, Some(format!("@{}", id)));
                            push_back_value_as_ins(program, ctx, alloc)?;
                            alloc
                        };
                        ctx.insert_symbol(&id, ASTValue::Variable(alloc));
                        Ok(())
                    }
                }
            }

            VarDef::Assign(id, len, init_val) => {
                let prev_def = ctx.look_up_in_curr_scope(id);
                match prev_def {
                    Some(_) => Err(Error::DuplicateDecl),
                    None => {
                        let dims_result: Result<Vec<i32>> = len
                            .iter()
                            .map(|exp| exp.eval(ctx))
                            .collect();
                        let mut dim_vec = dims_result?;

                        if ctx.in_global_scope() {
                            let parsed_init_val = init_val.generate_init_val(
                                program,
                                ctx,
                                &dim_vec,
                                0
                            )?;
                            let init = if len.is_empty() {
                                if let InitValResult::Value(val) = parsed_init_val {
                                    val
                                } else {
                                    unreachable!();
                                }
                            } else {
                                // 初始化list, folden flattened list
                                if let InitValResult::List(list) = parsed_init_val {
                                    let mut part_list = list;
                                    let mut cur_agg;
                                    let mut cur_idx = dim_vec.len() - 1;
                                    while cur_idx > 0 {
                                        let mut res_list = vec![];
                                        cur_agg = vec![];
                                        for i in 0..part_list.len() {
                                            cur_agg.push(part_list[i]);
                                            if
                                                (i as i32) % dim_vec[cur_idx] ==
                                                dim_vec[cur_idx] - 1
                                            {
                                                res_list.push(
                                                    program.new_value().aggregate(cur_agg)
                                                );
                                                cur_agg = vec![];
                                            }
                                        }
                                        cur_idx = cur_idx - 1;
                                        part_list = res_list;
                                    }
                                    program.new_value().aggregate(part_list)
                                } else {
                                    unreachable!();
                                }
                            };
                            let alloc = program.new_value().global_alloc(init);
                            program.set_value_name(alloc, Some(format!("@{}", id)));
                            ctx.insert_symbol(
                                &format!("{}", id).to_owned(),
                                ASTValue::Variable(alloc)
                            );
                            return Ok(());
                        }

                        let init_res = init_val.generate_init_val(program, ctx, &dim_vec, 0)?;
                        // 处理普通变量赋值
                        if len.is_empty() {
                            let alloc = cur_func_mut(program, ctx)
                                .dfg_mut()
                                .new_value()
                                .alloc(Type::get_i32());

                            if let InitValResult::Value(val) = init_res {
                                let store = cur_func_mut(program, ctx)
                                    .dfg_mut()
                                    .new_value()
                                    .store(val, alloc);
                                push_back_values_as_ins(program, ctx, vec![alloc, store]);
                                cur_func_mut(program, ctx)
                                    .dfg_mut()
                                    .set_value_name(alloc, Some(format!("@{}", id)));

                                ctx.insert_symbol(
                                    &format!("{}", id).to_owned(),
                                    ASTValue::Variable(alloc)
                                );
                            } else {
                                unreachable!();
                            }
                            return Ok(());
                        }

                        // 处理多维数组赋值
                        // let mut ptr_array = vec![];
                        let ty = len.gen_type(ctx);
                        let alloc = cur_func_mut(program, ctx).dfg_mut().new_value().alloc(ty);

                        push_back_value_as_ins(program, ctx, alloc)?;
                        cur_func_mut(program, ctx)
                            .dfg_mut()
                            .set_value_name(alloc, Some(format!("@{}", id)));

                        let mut cur_idx_array: Vec<i32> = len
                            .iter()
                            .map(|_| 0)
                            .collect();
                        // 每次inc一个多维数组的idx;直到赋值完所有
                        while cur_idx_array[0] < dim_vec[0] {
                            let mut ptr = alloc;
                            let mut ptrs = vec![];
                            for i in 0..cur_idx_array.len() {
                                let idx = cur_func_mut(program, ctx)
                                    .dfg_mut()
                                    .new_value()
                                    .integer(cur_idx_array[i]);
                                ptr = cur_func_mut(program, ctx)
                                    .dfg_mut()
                                    .new_value()
                                    .get_elem_ptr(ptr, idx);
                                ptrs.push(ptr);
                            }
                            let mut idx = cur_idx_array[0];
                            for i in 1..cur_idx_array.len() {
                                idx = idx * dim_vec[i] + cur_idx_array[i];
                            }
                            let store = cur_func_mut(program, ctx)
                                .dfg_mut()
                                .new_value()
                                .store(init_res.get_index(idx as usize).clone(), ptr);
                            ptrs.push(store);
                            push_back_values_as_ins(program, ctx, ptrs);
                            inc(&mut dim_vec, &mut cur_idx_array);
                        }
                        ctx.insert_symbol(&format!("{}", id).to_owned(), ASTValue::Variable(alloc));

                        Ok(())
                    }
                }
            }
        }
    }
}

fn inc(len: &mut Vec<i32>, idx: &mut Vec<i32>) -> bool {
    let mut cur_idx = idx.len() - 1;
    loop {
        idx[cur_idx] = idx[cur_idx] + 1;
        if idx[cur_idx] == len[cur_idx] {
            if cur_idx == 0 {
                return true;
            } else {
                idx[cur_idx] = 0;
                cur_idx = cur_idx - 1;
            }
        } else {
            return false;
        }
    }
}
#[derive(Debug)]
pub enum InitValResult {
    Value(Value),
    List(Vec<Value>),
}

impl InitValResult {
    fn get_index(&self, idx: usize) -> Value {
        if let InitValResult::List(list) = self { list[idx] } else { unreachable!() }
    }
}

impl ConstInitVal {
    fn generate_init_val(
        &self,
        program: &mut Program,
        ctx: &mut Context,
        dims: &[i32],
        cur_idx: i32
    ) -> Result<InitValResult> {
        let mut idx = cur_idx;
        match self {
            ConstInitVal::Single(exp) => {
                let res = exp.eval(ctx)?;
                if ctx.in_global_scope() {
                    Ok(InitValResult::Value(program.new_value().integer(res)))
                } else {
                    Ok(
                        InitValResult::Value(
                            cur_func_mut(program, ctx).dfg_mut().new_value().integer(res)
                        )
                    )
                }
            }
            ConstInitVal::List(list) => {
                let size: i32 = dims.iter().fold(1, |acc, &x| acc * x);
                if idx % dims[dims.len() - 1] != 0 || idx >= size {
                    panic!("FuckedUp");
                }
                let mut res: Vec<Value> = vec![];
                for sub in list {
                    let mut remain = idx;
                    // 从数组最后一列开始,向前整除,看看当前状态下,如果生成list,那么对齐的是哪一维
                    // int[2][3][4] {1, 2, 3, 4, {5}} ,idx = 4,对应int[4]
                    // int[2][3][4] {1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, {5}}, idx=12,对应 int[3][4]
                    // int[2][3][4] {{5}} ,idx = 0,对应 int[3][4]
                    let mut current = dims.len() - 1;
                    while remain % dims[current] == 0 && current >= 1 {
                        current = current - 1;
                        remain = remain / dims[current];
                    }
                    current = current + 1;
                    let sub_res = sub.generate_init_val(program, ctx, &dims[current..], cur_idx)?;
                    match sub_res {
                        InitValResult::List(mut sub_list) => {
                            res.append(&mut sub_list);
                            idx = idx + (sub_list.len() as i32);
                        }
                        InitValResult::Value(val) => {
                            res.push(val);
                            idx = idx + 1;
                        }
                    }
                }
                // 补零
                let zero = if ctx.in_global_scope() {
                    program.new_value().integer(0)
                } else {
                    cur_func_mut(program, ctx).dfg_mut().new_value().integer(0)
                };
                while res.len() < (size as usize) {
                    res.push(zero);
                }
                Ok(InitValResult::List(res))
            }
        }
    }
}

impl InitVal {
    fn generate_init_val(
        &self,
        program: &mut Program,
        ctx: &mut Context,
        dims: &[i32],
        cur_idx: i32
    ) -> Result<InitValResult> {
        let mut idx = cur_idx;
        match self {
            InitVal::Single(exp) =>
                Ok(
                    if ctx.in_global_scope() {
                        let res = exp.eval(ctx)?;
                        InitValResult::Value(program.new_value().integer(res))
                    } else {
                        InitValResult::Value(exp.generate(program, ctx)?)
                    }
                ),
            InitVal::List(list) => {
                let size: i32 = dims.iter().fold(1, |acc, &x| acc * x);
                if idx % dims[dims.len() - 1] != 0 || idx >= size {
                    panic!("FuckedUp");
                }
                let mut res: Vec<Value> = vec![];
                for sub in list {
                    let mut remain = idx;
                    // 从数组最后一列开始,向前整除,看看当前状态下,如果生成list,那么对齐的是哪一维
                    // int[2][3][4] {1, 2, 3, 4, {5}} ,idx = 4,对应int[4]
                    // int[2][3][4] {1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, {5}}, idx=12,对应 int[3][4]
                    // int[2][3][4] {{5}} ,idx = 0,对应 int[3][4]
                    let mut current = dims.len() - 1;
                    while remain % dims[current] == 0 && current >= 1 {
                        current = current - 1;
                        remain = remain / dims[current];
                    }
                    current = current + 1;
                    let sub_res = sub.generate_init_val(program, ctx, &dims[current..], cur_idx)?;
                    match sub_res {
                        InitValResult::List(mut sub_list) => {
                            res.append(&mut sub_list);
                            idx = idx + (sub_list.len() as i32);
                        }
                        InitValResult::Value(val) => {
                            res.push(val);
                            idx = idx + 1;
                        }
                    }
                }
                // 补零
                let zero = if ctx.in_global_scope() {
                    program.new_value().integer(0)
                } else {
                    cur_func_mut(program, ctx).dfg_mut().new_value().integer(0)
                };
                while res.len() < (size as usize) {
                    res.push(zero);
                }
                Ok(InitValResult::List(res))
            }
        }
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
        match &self.dims.len() {
            0 => {
                let eval_val = self.init_val.eval(ctx)?;
                ctx.insert_symbol(&self.id.clone(), ASTValue::Const(eval_val));
            }
            _ => {
                let dims_result: Result<Vec<i32>> = self.dims
                    .iter()
                    .map(|exp| exp.eval(ctx))
                    .collect();
                let mut dim_vec = dims_result?;
                let parsed_init_val = self.init_val.generate_init_val(program, ctx, &dim_vec, 0)?;
                // 处理多维数组赋值
                // let mut ptr_array = vec![];

                let ty = self.dims.gen_type(ctx);
                if ctx.in_global_scope() {
                    if let InitValResult::List(list) = parsed_init_val {
                        // 初始化list, folden flattened list
                        let mut part_list = list;
                        let mut cur_agg;
                        let mut cur_idx = dim_vec.len() - 1;
                        while cur_idx > 0 {
                            let mut res_list = vec![];
                            cur_agg = vec![];
                            for i in 0..part_list.len() {
                                cur_agg.push(part_list[i]);
                                if (i as i32) % dim_vec[cur_idx] == dim_vec[cur_idx] - 1 {
                                    res_list.push(program.new_value().aggregate(cur_agg));
                                    cur_agg = vec![];
                                }
                            }
                            cur_idx = cur_idx - 1;
                            part_list = res_list;
                        }
                        let init = program.new_value().aggregate(part_list);

                        let alloc = program.new_value().global_alloc(init);
                        program.set_value_name(alloc, Some(format!("@{}", self.id)));
                        ctx.insert_symbol(
                            &format!("{}", self.id).to_owned(),
                            ASTValue::Variable(alloc)
                        );
                        return Ok(());
                    } else {
                        unreachable!();
                    }
                }

                let alloc = cur_func_mut(program, ctx).dfg_mut().new_value().alloc(ty);

                push_back_value_as_ins(program, ctx, alloc)?;
                let mut cur_idx_array: Vec<i32> = self.dims
                    .iter()
                    .map(|_| 0)
                    .collect();
                // 每次inc一个多维数组的idx;直到赋值完所有
                while cur_idx_array[0] < dim_vec[0] {
                    let mut ptr = alloc;
                    let mut ptrs = vec![];
                    for i in 0..cur_idx_array.len() {
                        let idx = cur_func_mut(program, ctx)
                            .dfg_mut()
                            .new_value()
                            .integer(cur_idx_array[i]);
                        ptr = cur_func_mut(program, ctx)
                            .dfg_mut()
                            .new_value()
                            .get_elem_ptr(ptr, idx);
                        ptrs.push(ptr);
                    }
                    let mut idx = cur_idx_array[0];
                    for i in 1..cur_idx_array.len() {
                        idx = idx * dim_vec[i] + cur_idx_array[i];
                    }
                    let store = cur_func_mut(program, ctx)
                        .dfg_mut()
                        .new_value()
                        .store(parsed_init_val.get_index(idx as usize).clone(), ptr);
                    ptrs.push(store);
                    push_back_values_as_ins(program, ctx, ptrs);
                    inc(&mut dim_vec, &mut cur_idx_array);
                }
                ctx.insert_symbol(&format!("{}", self.id).to_owned(), ASTValue::Variable(alloc));
            }
        }
        Ok(())
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
        match self {
            Stmt::Ret(exp) => {
                // todo 优化: 基本块的出口是唯一的,
                // 翻译完return后可以在ctx中关闭基本块, 这样一些递归后序操作（比如if-else的尾部跳转指令）就不用加进去
                let res_val = exp.generate(program, ctx)?;
                let ret = cur_func_mut(program, ctx).dfg_mut().new_value().ret(Some(res_val));
                push_back_value_as_ins(program, ctx, ret)?;
                next_bb(program, ctx)?;
                Ok(())
            }
            Stmt::IfStmt(if_stmt) => if_stmt.generate(program, ctx),

            Stmt::Assign(lval, exp) => {
                let sym_val = ctx.look_up_symbol(&lval.id);
                match sym_val {
                    Some(ASTValue::Const(_)) => Err(Error::RedefineConstValue),
                    None => Err(Error::UnknownSymbol),
                    Some(ASTValue::Variable(var)) => {
                        let mut dst = var.clone();
                        let mut is_ptr_ptr = false;
                        if !dst.is_global() {
                            if
                                let TypeKind::Pointer(base) = value_data_in_cur_func(
                                    program,
                                    ctx,
                                    dst
                                )
                                    .ty()
                                    .kind()
                            {
                                if let TypeKind::Pointer(_) = base.kind() {
                                    is_ptr_ptr = true;
                                    dst = cur_func_mut(program, ctx)
                                        .dfg_mut()
                                        .new_value()
                                        .load(dst);
                                    push_back_value_as_ins(program, ctx, dst)?;
                                }
                            }
                        }

                        for i in 0..lval.indices.len() {
                            let kind = if dst.is_global() {
                                program.borrow_value(dst).ty().kind().clone()
                            } else {
                                value_data_in_cur_func(program, ctx, dst).ty().kind().clone()
                            };
                            let idx = lval.indices[i].generate(program, ctx)?;
                            dst = match kind {
                                TypeKind::Pointer(base) =>
                                    match base.kind() {
                                        TypeKind::Int32 => {
                                            if !is_ptr_ptr {
                                                unreachable!("deref an int");
                                            }
                                            let dst = cur_func_mut(program, ctx)
                                                .dfg_mut()
                                                .new_value()
                                                .get_ptr(dst, idx);
                                            push_back_value_as_ins(program, ctx, dst)?;
                                            dst
                                        }
                                        TypeKind::Array(_, _) => {
                                            let dst = cur_func_mut(program, ctx)
                                                .dfg_mut()
                                                .new_value()
                                                .get_elem_ptr(dst, idx);
                                            push_back_value_as_ins(program, ctx, dst)?;
                                            dst
                                        }
                                        _ => unreachable!(),
                                    }
                                _ => unreachable!(),
                            };
                        }
                        let rval = exp.generate(program, ctx)?;

                        let store_ins: Value = cur_func_mut(program, ctx)
                            .dfg_mut()
                            .new_value()
                            .store(rval, dst);
                        push_back_value_as_ins(program, ctx, store_ins)?;
                        Ok(())
                    }
                }
            }
            Stmt::While(while_stmt) => while_stmt.generate(program, ctx),
            Stmt::Exp(exp) => {
                exp.as_ref().map(|e| e.generate(program, ctx));
                Ok(())
            }
            Stmt::Block(block) => block.generate(program, ctx),
            Stmt::Break(break_stmt) => break_stmt.generate(program, ctx),
            Stmt::Continue(cont) => cont.generate(program, ctx),
        }
    }
}

impl GenerateProgram for Break {
    type Out = ();

    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        match ctx.peek_break_dst() {
            None => Err(Error::InvalidBreak),
            Some(bb) => {
                let jump = cur_func_mut(program, ctx).dfg_mut().new_value().jump(bb);
                push_back_value_as_ins(program, ctx, jump)?;
                next_bb(program, ctx)?;
                Ok(())
            }
        }
    }
}

impl GenerateProgram for Continue {
    type Out = ();

    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        match ctx.peek_cont_dst() {
            None => Err(Error::InvalidContinue),
            Some(bb) => {
                let jump = cur_func_mut(program, ctx).dfg_mut().new_value().jump(bb);
                push_back_value_as_ins(program, ctx, jump)?;
                next_bb(program, ctx)?;
                Ok(())
            }
        }
    }
}
impl GenerateProgram for While {
    type Out = ();

    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        let while_entry = cur_func_mut(program, ctx)
            .dfg_mut()
            .new_bb()
            .basic_block(Some("%while_entry".to_owned()));
        let while_body = cur_func_mut(program, ctx)
            .dfg_mut()
            .new_bb()
            .basic_block(Some("%while_body".to_owned()));
        let while_end = cur_func_mut(program, ctx)
            .dfg_mut()
            .new_bb()
            .basic_block(Some("%while_end".to_owned()));

        // begin entry jump
        let begin_while_entry_jump = cur_func_mut(program, ctx)
            .dfg_mut()
            .new_value()
            .jump(while_entry);
        push_back_value_as_ins(program, ctx, begin_while_entry_jump)?;

        push_block(program, ctx, while_entry)?;
        let cond = self.cond.generate(program, ctx)?;
        let loop_cond = cur_func_mut(program, ctx)
            .dfg_mut()
            .new_value()
            .branch(cond, while_body, while_end);
        push_back_value_as_ins(program, ctx, loop_cond)?;
        ctx.push_break_and_continue_dst(while_end, while_entry);
        push_block(program, ctx, while_body)?;
        self.body.generate(program, ctx)?;
        let jump_back_to_cond = cur_func_mut(program, ctx).dfg_mut().new_value().jump(while_entry);
        push_back_value_as_ins(program, ctx, jump_back_to_cond)?;
        ctx.pop_break_and_continue_dst();
        push_block(program, ctx, while_end)?;
        Ok(())
    }
}
impl GenerateProgram for IfStmt {
    type Out = ();

    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        // 1.在当前块下计算condition
        let cond_value = self.cond.generate(program, ctx)?;
        // 初始化then else和end块
        let then_block = cur_func_mut(program, ctx)
            .dfg_mut()
            .new_bb()
            .basic_block(Some("%then".into()));
        let else_block = cur_func_mut(program, ctx)
            .dfg_mut()
            .new_bb()
            .basic_block(Some("%else".into()));
        let end_block = cur_func_mut(program, ctx)
            .dfg_mut()
            .new_bb()
            .basic_block(Some("%ifend".into()));
        // 使用condition计算结果，增加branch指令
        let br_ins = cur_func_mut(program, ctx)
            .dfg_mut()
            .new_value()
            .branch(cond_value, then_block, else_block);

        push_back_value_as_ins(program, ctx, br_ins)?;

        // 生产then_block的语句。主体stmt + jump end指令
        push_block(program, ctx, then_block)?;
        self.then.generate(program, ctx)?;
        // 注意经过stmt生成,当前块可能已经不是then_block了!(可能是别的控制流的end_block)
        // 我们这里添加jump到end块需要在当前block而不是在then_block中
        let then_jump = cur_func_mut(program, ctx).dfg_mut().new_value().jump(end_block);
        push_back_value_as_ins(program, ctx, then_jump)?;

        // 生产else_block的语句。主体stmt + jump end指令
        push_block(program, ctx, else_block)?;
        self.else_stmt.as_ref().map(|stmt| stmt.generate(program, ctx));
        let else_jump = cur_func_mut(program, ctx).dfg_mut().new_value().jump(end_block);
        push_back_value_as_ins(program, ctx, else_jump)?;

        // 设置当前块为end_block,作为if结束后后续指令所在的块
        push_block(program, ctx, end_block)?;
        Ok(())
    }
}
impl GenerateProgram for Exp {
    type Out = Value;

    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            Exp::LOrExp(lor_exp) => lor_exp.generate(program, ctx),
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
                //  当前块 1.申请一个临时变量 2.val left条件 3.branch
                // then_block 此分支说明短路, 那么直接往临时变量上写结果
                // else_block 此分支说明未短路，eval right,结果写到变量上
                // end_block  读取临时变量,作为此表达式的返回结果
                let left_value: Value = left.generate(program, ctx)?;
                let func_data = program.func_mut(ctx.curr_fuc.unwrap());
                let zero = func_data.dfg_mut().new_value().integer(0);
                let res: Value = func_data.dfg_mut().new_value().alloc(Type::get_i32());
                let left_false = register_binary(program, ctx, left_value, zero, BinaryOp::Eq)?;
                let then_block = cur_func_mut(program, ctx)
                    .dfg_mut()
                    .new_bb()
                    .basic_block(Some("%then_block".into()));
                let else_block = cur_func_mut(program, ctx)
                    .dfg_mut()
                    .new_bb()
                    .basic_block(Some("%else".into()));
                let end_block = cur_func_mut(program, ctx)
                    .dfg_mut()
                    .new_bb()
                    .basic_block(Some("%ifend".into()));
                let branch = cur_func_mut(program, ctx)
                    .dfg_mut()
                    .new_value()
                    .branch(left_false, then_block, else_block);
                push_back_values_as_ins(program, ctx, vec![res, branch]);

                push_block(program, ctx, then_block)?;
                let inst_1 = cur_func_mut(program, ctx).dfg_mut().new_value().integer(0);
                let store_res: Value = cur_func_mut(program, ctx)
                    .dfg_mut()
                    .new_value()
                    .store(inst_1, res);
                let then_jump = cur_func_mut(program, ctx).dfg_mut().new_value().jump(end_block);
                push_back_values_as_ins(program, ctx, vec![store_res, then_jump]);

                push_block(program, ctx, else_block)?;

                let right_value = right.generate(program, ctx)?;
                let right_bool = register_binary(program, ctx, right_value, zero, BinaryOp::NotEq)?;
                let store_res: Value = cur_func_mut(program, ctx)
                    .dfg_mut()
                    .new_value()
                    .store(right_bool, res);
                let then_jump = cur_func_mut(program, ctx).dfg_mut().new_value().jump(end_block);
                push_back_values_as_ins(program, ctx, vec![store_res, then_jump]);

                push_block(program, ctx, end_block)?;
                let load = cur_func_mut(program, ctx).dfg_mut().new_value().load(res);
                push_back_value_as_ins(program, ctx, load)?;

                Ok(load)
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
                let func_data = program.func_mut(ctx.curr_fuc.unwrap());
                let zero = func_data.dfg_mut().new_value().integer(0);
                let res: Value = func_data.dfg_mut().new_value().alloc(Type::get_i32());
                let left_bool = register_binary(program, ctx, left_value, zero, BinaryOp::NotEq)?;
                let then_block = cur_func_mut(program, ctx)
                    .dfg_mut()
                    .new_bb()
                    .basic_block(Some("%then_block".into()));
                let else_block = cur_func_mut(program, ctx)
                    .dfg_mut()
                    .new_bb()
                    .basic_block(Some("%else".into()));
                let end_block = cur_func_mut(program, ctx)
                    .dfg_mut()
                    .new_bb()
                    .basic_block(Some("%ifend".into()));
                let branch = cur_func_mut(program, ctx)
                    .dfg_mut()
                    .new_value()
                    .branch(left_bool, then_block, else_block);

                push_back_values_as_ins(program, ctx, vec![res, branch]);
                push_block(program, ctx, then_block)?;
                let inst_1 = cur_func_mut(program, ctx).dfg_mut().new_value().integer(1);
                let store_res: Value = cur_func_mut(program, ctx)
                    .dfg_mut()
                    .new_value()
                    .store(inst_1, res);
                let then_jump = cur_func_mut(program, ctx).dfg_mut().new_value().jump(end_block);
                push_back_values_as_ins(program, ctx, vec![store_res, then_jump]);

                push_block(program, ctx, else_block)?;

                let right_value = right.generate(program, ctx)?;
                let right_bool = register_binary(program, ctx, right_value, zero, BinaryOp::NotEq)?;
                let store_res: Value = cur_func_mut(program, ctx)
                    .dfg_mut()
                    .new_value()
                    .store(right_bool, res);
                let then_jump = cur_func_mut(program, ctx).dfg_mut().new_value().jump(end_block);
                push_back_values_as_ins(program, ctx, vec![store_res, then_jump]);

                push_block(program, ctx, end_block)?;
                let load = cur_func_mut(program, ctx).dfg_mut().new_value().load(res);
                push_back_value_as_ins(program, ctx, load)?;
                Ok(load)
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
                        let is_array = !lval.indices.is_empty();
                        // lval作为表达式（即出现在等号右边时),此时需要求值
                        // 1. name => 内存位置 (查env//符号表)
                        // 2. 内存位置 => 内存值 (koopa中的load, 读出val作为指针实际指向的栈值，作为表达式的返回)
                        let value: Option<&ASTValue> = ctx.look_up_symbol(&lval.id);
                        match value {
                            None => Err(super::Error::UnknownSymbol),
                            Some(ASTValue::Const(val)) => {
                                assert!(!is_array);
                                let local_val = val.clone();
                                // 表达式中的左值,如果是常量,直接取解析结果
                                Ok(
                                    cur_func_mut(program, ctx)
                                        .dfg_mut()
                                        .new_value()
                                        .integer(local_val)
                                )
                            }
                            Some(ASTValue::Variable(var)) => {
                                let mut dst = var.clone();
                                // 存在这两种情况
                                // 1.var本身是一个本地栈上alloc的array,此时var是一个pointer(array) (at see alloc)
                                // 2.函数的形参为一个数组,此时函数形参本身是一个pointer(i32),
                                // 但是scope中存的name对应的var 是本地栈上alloc出的一个栈帧,逻辑上类型是一个pointer(pointer(i32)) (at see funcParam.generate)
                                //
                                let mut is_ptr_ptr = false;
                                // 处理下函数形参为数组的情况,此时符号表中存的是一个二阶指针,需要先load一次
                                // todo:stmt assign里有类似逻辑，考虑合并
                                let mut fully_deref = true;

                                if !dst.is_global() {
                                    if
                                        let TypeKind::Pointer(base) = value_data_in_cur_func(
                                            program,
                                            ctx,
                                            dst
                                        )
                                            .ty()
                                            .kind()
                                    {
                                        if let TypeKind::Pointer(_) = base.kind() {
                                            is_ptr_ptr = true;
                                            fully_deref = false;
                                            dst = cur_func_mut(program, ctx)
                                                .dfg_mut()
                                                .new_value()
                                                .load(dst);
                                            push_back_value_as_ins(program, ctx, dst)?;
                                        }
                                    }
                                }

                                for i in 0..lval.indices.len() {
                                    let kind = if dst.is_global() {
                                        program.borrow_value(dst).ty().kind().clone()
                                    } else {
                                        value_data_in_cur_func(program, ctx, dst)
                                            .ty()
                                            .kind()
                                            .clone()
                                    };
                                    let idx = lval.indices[i].generate(program, ctx)?;
                                    dst = if is_ptr_ptr && i == 0 {
                                        let dst = cur_func_mut(program, ctx)
                                            .dfg_mut()
                                            .new_value()
                                            .get_ptr(dst, idx);
                                        is_ptr_ptr = false;
                                        dst
                                    } else {
                                        let dst = cur_func_mut(program, ctx)
                                            .dfg_mut()
                                            .new_value()
                                            .get_elem_ptr(dst, idx);
                                        dst
                                    };
                                    push_back_value_as_ins(program, ctx, dst)?;
                                }

                                if dst.is_global() {
                                    let load = cur_func_mut(program, ctx)
                                        .dfg_mut()
                                        .new_value()
                                        .load(dst);
                                    push_back_value_as_ins(program, ctx, load)?;
                                    return Ok(load);
                                }
                                if is_ptr_ptr {
                                    return Ok(dst);
                                }
                                // 如果是数组指针,对应数组没有完全解引用的情况,则返回自身,由外界检查并决定使用
                                // 如果是完全解引用的（如果 a, b[1]),则直接返回load指针后得到的值
                                match value_data_in_cur_func(program, ctx, dst).ty().kind() {
                                    TypeKind::Pointer(base) =>
                                        match base.kind() {
                                            TypeKind::Int32 => {
                                                let load = cur_func_mut(program, ctx)
                                                    .dfg_mut()
                                                    .new_value()
                                                    .load(dst);
                                                push_back_value_as_ins(program, ctx, load)?;
                                                Ok(load)
                                            }
                                            TypeKind::Array(_, _) => {
                                                let zero = cur_func_mut(program, ctx)
                                                    .dfg_mut()
                                                    .new_value()
                                                    .integer(0);
                                                dst = cur_func_mut(program, ctx)
                                                    .dfg_mut()
                                                    .new_value()
                                                    .get_elem_ptr(dst, zero);
                                                push_back_value_as_ins(program, ctx, dst)?;
                                                Ok(dst)
                                            }
                                            _ => unreachable!(),
                                        }
                                    _ => unreachable!(),
                                }
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
                        push_back_value_as_ins(program, ctx, res)?;
                        Ok(res)
                    }
                }
            }
            UnaryExp::FuncCall(func_call) => func_call.generate(program, ctx),
        }
    }
}

impl GenerateProgram for FuncCall {
    type Out = Value;

    fn generate(&self, program: &mut Program, ctx: &mut Context) -> Result<Self::Out> {
        let mut call_params = vec![];
        for ele in &self.params {
            let mut val = ele.generate(program, ctx).unwrap();

            let type_kind = if val.is_global() {
                program.borrow_value(val).ty().kind().clone()
            } else {
                value_data_in_cur_func(program, ctx, val).ty().kind().clone()
            };
            call_params.push(val);
        }
        let func = ctx.scopes.look_up_func(&self.func_name).unwrap().clone();
        let call = cur_func_mut(program, ctx).dfg_mut().new_value().call(func, call_params);
        push_back_value_as_ins(program, ctx, call)?;
        Ok(call)
    }
}

fn register_binary(
    program: &mut Program,
    ctx: &mut Context,
    left: Value,
    right: Value,
    op: BinaryOp
) -> Result<Value> {
    let res = cur_func_mut(program, ctx).dfg_mut().new_value().binary(op, left, right);
    cur_func_mut(program, ctx)
        .layout_mut()
        .bb_mut(ctx.curr_block.unwrap())
        .insts_mut()
        .push_key_back(res)
        .map_err(|_| Error::SysError)?;
    Ok(res)
}

// 我们让对functiondata的变量往往是作为临时变量存在；如果函数中一直存在这个引用，那么相当于一直持有program的引用
// borrow checker 非常烦人
fn cur_func_mut<'a, 'b>(program: &'a mut Program, ctx: &'b mut Context) -> &'a mut FunctionData {
    program.func_mut(ctx.curr_fuc.unwrap())
}

fn remove_useless_block<'a, 'b>(program: &'a mut Program, ctx: &'b mut Context) {
    let func = cur_func_mut(program, ctx);
    let bbs = func.layout_mut().bbs_mut();

    let empty_bbs: std::collections::HashSet<_> = bbs
        .iter()
        .filter(|(_, v)| v.insts().is_empty())
        .map(|(k, _)| k.clone())
        .collect();

    for ele in empty_bbs {
        bbs.remove(&ele);
    }
}

/**
 * 创建一个匿名块并设置为当前块,一般用于Ret,break,continue等对应 基本块出口指令翻译完后
 *
 */
fn next_bb(program: &mut Program, ctx: &mut Context) -> Result<()> {
    let new_bb: koopa::ir::BasicBlock = cur_func_mut(program, ctx)
        .dfg_mut()
        .new_bb()
        .basic_block(None);
    push_block(program, ctx, new_bb)
}

/**
 * 在当前函数的基本块序列中插入某个块,并将上下文中的"当前块"设置为此块
 */
fn push_block(program: &mut Program, ctx: &mut Context, bb: BasicBlock) -> Result<()> {
    cur_func_mut(program, ctx)
        .layout_mut()
        .bbs_mut()
        .push_key_back(bb)
        .map_err(|_| Error::PushBlockFailed)?;
    ctx.curr_block = Some(bb);
    Ok(())
}

/**
 * 在当前函数的当前块的指令序列最后插入某个value
 */
fn push_back_value_as_ins(program: &mut Program, ctx: &mut Context, val: Value) -> Result<()> {
    cur_block_mut(program, ctx)
        .insts_mut()
        .push_key_back(val)
        .map_err(|_| Error::PushInstructionFailed)?;
    Ok(())
}
fn push_back_values_as_ins(program: &mut Program, ctx: &mut Context, vals: Vec<Value>) {
    cur_block_mut(program, ctx).insts_mut().extend(vals);
}

fn cur_block_mut<'a, 'b>(program: &'a mut Program, ctx: &'b mut Context) -> &'a mut BasicBlockNode {
    cur_func_mut(program, ctx).layout_mut().bb_mut(ctx.curr_block.unwrap())
}

fn value_data_in_cur_func<'a, 'b>(
    program: &'a mut Program,
    ctx: &'b mut Context,
    value: Value
) -> &'a ValueData {
    cur_func_mut(program, ctx).dfg().value(value)
}
// 为koopaIr注册sysy的库函数定义;同时在上下文中注册库函数name->func的映射,使得后文funcall能找到对应func句柄
fn add_sysy_lib_func(program: &mut Program, ctx: &mut Context) {
    let dec1 = FunctionData::new_decl("@getint".to_owned(), vec![], Type::get_i32());
    let dec2 = FunctionData::new_decl("@getch".to_owned(), vec![], Type::get_i32());
    let dec3 = FunctionData::new_decl(
        "@getarray".to_owned(),
        vec![Type::get_pointer(Type::get_i32())],
        Type::get_i32()
    );
    let dec4 = FunctionData::new_decl(
        "@putint".to_owned(),
        vec![Type::get_i32()],
        Type::get_unit()
    );
    let dec5 = FunctionData::new_decl("@putch".to_owned(), vec![Type::get_i32()], Type::get_unit());
    let dec6 = FunctionData::new_decl(
        "@putarray".to_owned(),
        vec![Type::get_i32(), Type::get_pointer(Type::get_i32())],
        Type::get_unit()
    );
    let dec7 = FunctionData::new_decl("@starttime".to_owned(), vec![], Type::get_unit());

    let dec8 = FunctionData::new_decl("@stoptime".to_owned(), vec![], Type::get_unit());
    let dec_list = vec![dec1, dec2, dec3, dec4, dec5, dec6, dec7, dec8];
    for dec in dec_list {
        let name = &dec.name()[1..].to_string();
        let func = program.new_func(dec);
        ctx.scopes.register_function(name, func);
    }
}
