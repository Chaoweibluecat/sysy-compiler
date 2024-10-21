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
    PushBlockFailed,
    PushInstructionFailed,
}

pub struct Context {
    pub curr_fuc: Option<Function>,
    pub curr_block: Option<BasicBlock>,
    pub scopes: Scopes,
    pub break_continue_dst: LinkedList<(BasicBlock, BasicBlock)>,
    // Vardecl生成时,通过上下文判断是否在全局scope
}

pub struct Scopes {
    // sysy标准:
    pub values: LinkedList<HashMap<String, ASTValue>>,
    pub global_values: HashMap<String, ASTValue>,
    pub func: HashMap<String, Function>,
}

impl Scopes {
    pub fn new() -> Self {
        Scopes {
            values: LinkedList::new(),
            global_values: HashMap::new(),
            func: HashMap::new(),
        }
    }
    pub fn register_function(&mut self, name: &String, func: Function) {
        self.func.insert(name.clone(), func);
    }

    pub fn look_up_func(&mut self, name: &String) -> Option<&Function> {
        self.func.get(name)
    }

    pub fn insert_global_symbol(&mut self, name: &String, value: ASTValue) {
        self.global_values.insert(name.clone(), value);
    }

    pub fn look_up_global_symbol(&mut self, name: &String) -> Option<&ASTValue> {
        self.global_values.get(name)
    }

    pub fn insert_symbol(&mut self, name: &String, value: ASTValue) {
        self.values.front_mut().unwrap().insert(name.clone(), value);
    }

    fn look_up_symbol(&self, name: &str) -> Option<&ASTValue> {
        self.values
            .iter()
            .find_map(|symbol_table| symbol_table.get(name))
            .or_else(|| self.global_values.get(name))
    }

    fn look_up_in_curr_scope(&self, name: &str) -> Option<&ASTValue> {
        self.values
            .front()
            .and_then(|symbol_table| symbol_table.get(name))
            .or_else(|| self.global_values.get(name))
    }

    fn new_scope(&mut self) {
        self.values.push_front(HashMap::new());
    }
    fn leave_scope(&mut self) {
        self.values.pop_front();
    }
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
            scopes: Scopes::new(),
            // while程序跳转目标地址;由于while可嵌套,所以应该是个栈
            break_continue_dst: LinkedList::new(),
        }
    }

    pub fn in_global_scope(&self) -> bool {
        matches!(self.curr_fuc, None)
    }

    pub fn insert_symbol(&mut self, name: &String, value: ASTValue) {
        if self.in_global_scope() {
            self.scopes.insert_global_symbol(name, value);
        } else {
            self.scopes.insert_symbol(name, value);
        }
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
        self.scopes.look_up_symbol(name)
    }

    fn look_up_in_curr_scope(&self, name: &str) -> Option<&ASTValue> {
        self.scopes.look_up_in_curr_scope(name)
    }

    fn new_scope(&mut self) {
        self.scopes.new_scope();
    }
    fn leave_scope(&mut self) {
        self.scopes.leave_scope();
    }
}

/// Generates Koopa IR program for the given compile unit (ASTs).
pub fn generate_program(comp_unit: &CompUnit) -> Result<Program> {
    let mut program = Program::new();
    let mut ctx = Context::new();
    comp_unit.generate(&mut program, &mut ctx)?;
    Ok(program)
}
