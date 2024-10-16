use crate::{
    ast::*,
    irgen::{Context, Result},
};

use super::ASTValue;

// 编译期间使用的表达式解析工具

// 注意在处理 AST中的左值（这里是广义上的LVal,实际是symbol), 时, 由于只有 const的值可以在编译期间确定（通过查符号表直接获得）
// 变量对应的值在编译期间无法确定; 所以在PrimaryExp.eval中会拒绝求值;
// 编译期间表达式求值仅依赖符号表,不依赖Program;
fn cast_int_to_bool(int_val: i32) -> bool {
    if int_val == 0 {
        false
    } else {
        true
    }
}

pub trait Eval {
    type Out;
    fn eval(&self, ctx: &mut Context) -> Result<Self::Out>;
}

impl Eval for ConstInitVal {
    type Out = i32;
    fn eval(&self, ctx: &mut Context) -> Result<Self::Out> {
        self.exp.eval(ctx)
    }
}

impl Eval for ConstExp {
    type Out = i32;
    fn eval(&self, ctx: &mut Context) -> Result<Self::Out> {
        self.exp.eval(ctx)
    }
}

impl Eval for Exp {
    type Out = i32;
    fn eval(&self, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            Exp::LOrExp(exp) => exp.eval(ctx),
        }
    }
}

impl Eval for LOrExp {
    type Out = i32;
    fn eval(&self, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            LOrExp::LAndExp(and_exp) => and_exp.eval(ctx),
            LOrExp::LOrExp(left, _, right) => {
                let left = left.eval(ctx)?;
                let right = right.eval(ctx)?;
                Ok((left != 0 || cast_int_to_bool(right)) as i32)
            }
        }
    }
}

impl Eval for LAndExp {
    type Out = i32;
    fn eval(&self, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            LAndExp::EqExp(eq_exp) => eq_exp.eval(ctx),
            LAndExp::LAndExp(left, _, right) => {
                let left = left.eval(ctx)?;
                let right = right.eval(ctx)?;
                Ok((cast_int_to_bool(left) && cast_int_to_bool(right)) as i32)
            }
        }
    }
}

impl Eval for EqExp {
    type Out = i32;
    fn eval(&self, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            EqExp::RelExp(exp) => exp.eval(ctx),
            EqExp::EqExp(left, op, right) => {
                let left = left.eval(ctx)?;
                let right = right.eval(ctx)?;
                match op {
                    EqOp::Eq => Ok((left == right) as i32),
                    EqOp::Ne => Ok((left != right) as i32),
                }
            }
        }
    }
}

impl Eval for RelExp {
    type Out = i32;
    fn eval(&self, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            RelExp::AddExp(exp) => exp.eval(ctx),
            RelExp::RelExp(left, op, right) => {
                let left = left.eval(ctx)?;
                let right = right.eval(ctx)?;
                match op {
                    RelOp::Ge => Ok((left >= right) as i32),
                    RelOp::Gt => Ok((left > right) as i32),
                    RelOp::Le => Ok((left <= right) as i32),
                    RelOp::Lt => Ok((left < right) as i32),
                }
            }
        }
    }
}

impl Eval for AddExp {
    type Out = i32;
    fn eval(&self, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            AddExp::MulExp(exp) => exp.eval(ctx),
            AddExp::AddExp(left, op, right) => {
                let left = left.eval(ctx)?;
                let right = right.eval(ctx)?;
                match op {
                    AddOp::Add => Ok(left + right),
                    AddOp::Minus => Ok(left - right),
                }
            }
        }
    }
}

impl Eval for MulExp {
    type Out = i32;
    fn eval(&self, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            MulExp::UnaryExp(exp) => exp.eval(ctx),
            MulExp::MulExp(left, op, right) => {
                let left = left.eval(ctx)?;
                let right = right.eval(ctx)?;
                match op {
                    MulOp::Multi => Ok(left * right),
                    MulOp::Divide => Ok(left / right),
                    MulOp::Mod => Ok(left / right),
                }
            }
        }
    }
}

impl Eval for UnaryExp {
    type Out = i32;
    fn eval(&self, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            UnaryExp::PrimaryExp(exp) => exp.eval(ctx),
            UnaryExp::UnaryExp(op, left) => {
                let left = left.eval(ctx)?;
                match op {
                    UnaryOp::NEGATIVE => Ok(-left),
                    UnaryOp::POSITIVE => Ok(left),
                    UnaryOp::NOT => Ok(!left),
                }
            }
            _ => unimplemented!(),
        }
    }
}

impl Eval for PrimaryExp {
    type Out = i32;
    fn eval(&self, ctx: &mut Context) -> Result<Self::Out> {
        match self {
            PrimaryExp::Exp(exp) => exp.eval(ctx),
            PrimaryExp::Number(num) => Ok(*num),
            PrimaryExp::LVal(lval) => {
                let val_op = ctx.look_up_symbol(&lval.id);
                match val_op {
                    None => Err(super::Error::UnknownSymbol),
                    Some(ASTValue::Const(val)) => Ok(*val),
                    _ => Err(super::Error::VariableEvalAtCompileTime),
                }
            }
        }
    }
}
