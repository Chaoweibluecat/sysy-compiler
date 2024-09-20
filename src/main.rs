use irgen::generate_program;
use koopa::back::KoopaGenerator;
use koopa::ir::builder::{ BasicBlockBuilder, LocalInstBuilder, ValueBuilder };
use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fmt::format;
use std::fs::read_to_string;
use std::io::Result;
mod ast;
mod irgen;
use koopa::ir::{ FunctionData, Program, Type, Value };

// 引用 lalrpop 生成的解析器
// 因为我们刚刚创建了 sysy.lalrpop, 所以模块名是 sysy
lalrpop_mod!(sysy);

fn main() -> Result<()> {
    // 解析命令行参数
    let mut args = args();
    // args.next();
    // let mode = args.next().unwrap();
    let input = "hello.c";
    // args.next();
    let output = "hello.koopa";

    // 读取输入文件
    let input = read_to_string(input)?;

    // parse input file
    let comp_unit = sysy::CompUnitParser::new().parse(&input).unwrap();

    let prog = generate_program(&comp_unit).unwrap();
    // let main_handle = prog.new_func(
    //     FunctionData::with_param_names(
    //         format!("@{}", comp_unit.func_def.ident).into(),
    //         vec![],
    //         Type::get_i32()
    //     )
    // );
    // let main = prog.func_mut(main_handle);

    // let entry = main.dfg_mut().new_bb().basic_block(Some("%entry".to_string()));
    // main.layout_mut().bbs_mut().push_key_back(entry);

    // let res_val = main.dfg_mut().new_value().integer(comp_unit.func_def.block.stmt.num);
    // let ret = main.dfg_mut().new_value().ret(Some(res_val));
    // main.layout_mut().bb_mut(entry).insts_mut().push_key_back(ret);

    KoopaGenerator::from_path(output).unwrap().generate_on(&prog).unwrap();
    // prog.
    println!("{:#?}", comp_unit);

    Ok(())
}
