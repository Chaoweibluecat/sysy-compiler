use crate::ast::CompUnit;
use generate::GenerateProgram;
use koopa::ir::{BasicBlock, Function, FunctionData, Program};

mod generate;
#[derive(Debug)]
pub enum Error {
    SysError,
}

pub struct Context {
    pub curr_fuc: Option<Function>,
    pub curr_block: Option<BasicBlock>,
}
pub type Result<T> = std::result::Result<T, Error>;

impl Context {
    pub fn new() -> Self {
        Context {
            curr_fuc: None,
            curr_block: None,
        }
    }
}
/// Generates Koopa IR program for the given compile unit (ASTs).
pub fn generate_program(comp_unit: &CompUnit) -> Result<Program> {
    let mut program = Program::new();
    let mut ctx = Context::new();
    comp_unit.generate(&mut program, &mut ctx)?;
    Ok(program)
}
