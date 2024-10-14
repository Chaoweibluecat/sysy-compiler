use std::collections::{ HashMap, LinkedList };

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
    InvalidBreak,
    InvalidContinue,
}

pub struct Context {
    pub curr_fuc: Option<Function>,
    pub curr_block: Option<BasicBlock>,
    pub scopes: LinkedList<HashMap<String, ASTValue>>,
    pub break_continue_dst: LinkedList<(BasicBlock, BasicBlock)>,
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
            // while程序跳转目标地址;由于while可嵌套,所以应该是个栈
            break_continue_dst: LinkedList::new(),
        }
    }

    pub fn insert_symbol(&mut self, name: &String, value: ASTValue) {
        self.scopes.front_mut().unwrap().insert(name.clone(), value);
    }

    pub fn push_break_and_continue_dst(&mut self, break_dst: BasicBlock, cont_dst: BasicBlock) {
        self.break_continue_dst.push_front((break_dst, cont_dst));
    }

    pub fn pop_break_and_continue_dst(&mut self) {
        self.break_continue_dst.pop_front();
    }
    pub fn peek_break_dst(&self) -> Option<BasicBlock> {
        self.break_continue_dst.front().map(|pair| { pair.0 })
    }

    pub fn peek_cont_dst(&self) -> Option<BasicBlock> {
        self.break_continue_dst.front().map(|pair| { pair.1 })
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
