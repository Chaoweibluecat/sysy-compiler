use std::collections::{HashMap, LinkedList};

use crate::ast::CompUnit;
use generate::GenerateProgram;
use koopa::ir::{BasicBlock, Function, Program, Value};

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
    pub scopes: LinkedList<HashMap<String, ASTValue>>,
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
            scopes: LinkedList::new(),
        }
    }

    pub fn insert_symbol(&mut self, name: &String, value: ASTValue) {
        self.scopes.front_mut().unwrap().insert(name.clone(), value);
    }

    fn look_up_symbol(&self, name: &str) -> Option<&ASTValue> {
        self.scopes
            .iter()
            .filter_map(|symbol_table| symbol_table.get(name))
            .next()
    }

    fn look_up_in_curr_scope(&self, name: &str) -> Option<&ASTValue> {
        self.scopes.front().unwrap().get(name)
    }

    fn new_scope(&mut self) {
        self.scopes.push_front(HashMap::new());
    }
    fn leave_scope(&mut self) {
        self.scopes.pop_front();
    }
}
/// Generates Koopa IR program for the given compile unit (ASTs).
pub fn generate_program(comp_unit: &CompUnit) -> Result<Program> {
    let mut program = Program::new();
    let mut ctx = Context::new();
    comp_unit.generate(&mut program, &mut ctx)?;
    Ok(program)
}
