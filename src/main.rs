use koopa::back::KoopaGenerator;
use koopa::ir::builder::BasicBlockBuilder;
use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fs::read_to_string;
use std::io::Result;
mod ast;
use koopa::front::ast::{ AstBox };
use koopa::ir::{ FunctionData, Program, Type };

// 引用 lalrpop 生成的解析器
// 因为我们刚刚创建了 sysy.lalrpop, 所以模块名是 sysy
lalrpop_mod!(sysy);

fn main() -> Result<()> {
    // 解析命令行参数
    let mut args = args();
    args.next();
    let mode = args.next().unwrap();
    let input = args.next().unwrap();
    args.next();
    let output = args.next().unwrap();

    // 读取输入文件
    let input = read_to_string(input)?;

    // parse input file
    let comp_unit = sysy::CompUnitParser::new().parse(&input).unwrap();

    let mut prog = Program::new();

    let mut main = FunctionData::new(
        format!("@{}", comp_unit.func_def.ident).to_string(),
        vec![],
        Type::get_i32()
    );
    main.dfg_mut().new_bb().basic_block(Some("%entry".to_string()));

    prog.new_func(main);
    KoopaGenerator::from_path(output).unwrap().generate_on(&prog).unwrap();
    // prog.
    println!("{:#?}", comp_unit);

    Ok(())
}
