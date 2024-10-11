use std::collections::HashMap;

use crate::ast::CompUnit;
use generate::GenerateProgram;
use koopa::ir::{ BasicBlock, Function, Program, Value };

mod eval;
mod generate;
#[derive(Debug)]
pub enum Error {
    SysError,
    UnknownSymbol,
    DuplicateDecl,
    VariableEvalAtCompileTime,
    RedefineConstValue,
}

pub struct Context {
    pub curr_fuc: Option<Function>,
    pub curr_block: Option<BasicBlock>,
    pub symbol_table: HashMap<String, ASTValue>,
    // pub scopes: Vec<HashMap<String, ASTValue>>,
}

// for each symbol,store parsedVal for const, store value for variable
#[derive(Debug, Clone, Copy)]
pub enum ASTValue {
    Const(i32),
    Variable(Value),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Context {
    pub fn new() -> Self {
        Context {
            curr_fuc: None,
            curr_block: None,
            symbol_table: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: &String, value: ASTValue) {
        self.symbol_table.insert(name.clone(), value);
    }

    pub fn look_up_symbol(&self, name: &String) -> Option<&ASTValue> {
        self.symbol_table.get(name)
    }
}
/// Generates Koopa IR program for the given compile unit (ASTs).
pub fn generate_program(comp_unit: &CompUnit) -> Result<Program> {
    let mut program = Program::new();
    let mut ctx = Context::new();
    comp_unit.generate(&mut program, &mut ctx)?;
    Ok(program)
}
