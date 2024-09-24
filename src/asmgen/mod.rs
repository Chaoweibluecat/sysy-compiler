use std::fs::File;

use koopa::ir::{ Function, Program };
use asmgen::GenerateAsm;
mod asmgen;
use crate::irgen::Result;

struct Context<'a> {
    prog: &'a Program,
    func: Option<Function>,
}

/// Generates the given Koopa IR program to RISC-V assembly.
pub fn generate_asm(program: &Program, path: &str) -> Result<()> {
    program.generate(&mut File::create(path).unwrap(), &mut (Context { prog: program, func: None }))
}
