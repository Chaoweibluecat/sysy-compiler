use std::{ collections::HashMap, fs::File };

use asmgen::GenerateAsm;
use koopa::ir::{ BasicBlock, Function, Program, Value };
mod asmgen;
use crate::irgen::Result;

pub struct Context<'a> {
    prog: &'a Program,
    func: Option<Function>,
    cur_func_info: Option<FunctionInfo>,
    value_2_stack_offset: HashMap<Value, i32>,

    cur_value: Option<Value>,
    basic_block_to_label_name: HashMap<BasicBlock, String>,
    label_counter: i32,
}

pub struct FunctionInfo {
    stack_allocation: i32,
    is_leaf_func: bool,
}

pub enum InsData {
    Int(i32),
    StackSlot(i32),
    Reg(String),
}
/// Generates the given Koopa IR program to RISC-V assembly.
pub fn generate_asm(program: &Program, path: &str) -> Result<()> {
    program.generate(
        &mut File::create(path).unwrap(),
        &mut (Context {
            prog: program,
            func: None,
            value_2_stack_offset: HashMap::new(),
            cur_func_info: None,
            cur_value: None,
            basic_block_to_label_name: HashMap::new(),
            label_counter: 0,
        })
    )
}
