use asmgen::generate_asm;
use irgen::generate_program;
use koopa::back::KoopaGenerator;
use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fs::read_to_string;
use std::io::Result;
mod ast;
mod irgen;
mod asmgen;

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
    let output_2 = "hello.asm";

    // 读取输入文件
    let input = read_to_string(input)?;

    // parse input file
    let comp_unit = sysy::CompUnitParser::new().parse(&input).unwrap();

    let prog = generate_program(&comp_unit).unwrap();

    KoopaGenerator::from_path(output).unwrap().generate_on(&prog).unwrap();
    generate_asm(&prog, &output_2).expect("failed to generate asm");
    // prog.
    println!("{:#?}", comp_unit);

    Ok(())
}
