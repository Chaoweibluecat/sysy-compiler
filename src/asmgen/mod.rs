use std::{ collections::HashMap, fs::File };

use asmgen::GenerateAsm;
use koopa::ir::{ Function, Program, Value };
mod asmgen;
use crate::irgen::Result;

pub struct Context<'a> {
    prog: &'a Program,
    func: Option<Function>,
    value_2_stack_offset: HashMap<Value, i32>,
    cur_fuc_stack_allocation: Option<i32>,
    cur_value: Option<Value>,
}

pub enum InsData {
    Int(i32),
    StackSlot(i32),
}
/// Generates the given Koopa IR program to RISC-V assembly.
pub fn generate_asm(program: &Program, path: &str) -> Result<()> {
    program.generate(
        &mut File::create(path).unwrap(),
        &mut (Context {
            prog: program,
            func: None,
            value_2_stack_offset: HashMap::new(),
            cur_fuc_stack_allocation: None,
            cur_value: None,
        })
    )
}
