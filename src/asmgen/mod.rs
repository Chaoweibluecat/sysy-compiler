use std::{collections::HashMap, fs::File};

use asmgen::GenerateAsm;
use koopa::ir::{Function, Program, Value};
mod asmgen;
use crate::irgen::Result;

pub struct Context<'a> {
    prog: &'a Program,
    func: Option<Function>,
    curr_reg: i32,
    value_2_regs: HashMap<Value, i32>,
}

pub enum InsData {
    Int(i32),
    TempResult(String),
}
/// Generates the given Koopa IR program to RISC-V assembly.
pub fn generate_asm(program: &Program, path: &str) -> Result<()> {
    program.generate(
        &mut File::create(path).unwrap(),
        &mut (Context {
            prog: program,
            func: None,
            curr_reg: 1,
            value_2_regs: HashMap::new(),
        }),
    )
}
