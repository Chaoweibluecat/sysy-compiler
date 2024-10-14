use asmgen::generate_asm;
use irgen::generate_program;
use koopa::back::KoopaGenerator;
use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fs::read_to_string;
use std::io::Result;
mod asmgen;
mod ast;
mod irgen;

// 引用 lalrpop 生成的解析器
// 因为我们刚刚创建了 sysy.lalrpop, 所以模块名是 sysy
lalrpop_mod!(sysy);

fn main() -> Result<()> {
    let input = "hello.c";
    let mode = "-123".to_owned();
    let output = "hello.koopa";
    let output_2 = "hello.asm";

    // let mut args = args();
    // args.next();
    // let mode = args.next().unwrap();
    // let input = args.next().unwrap();
    // args.next();
    // let output = args.next().unwrap();
    let input = read_to_string(input)?;

    // parse input file
    let comp_unit: ast::CompUnit = sysy::CompUnitParser::new().parse(&input).unwrap();

    let prog = generate_program(&comp_unit).unwrap();

    if mode.as_str() == "-koopa" {
        KoopaGenerator::from_path(output)
            .unwrap()
            .generate_on(&prog)
            .unwrap();
    } else if mode.as_str() == "-riscv" {
        generate_asm(&prog, &output).expect("failed to generate asm");
    } else {
        KoopaGenerator::from_path(output)
            .unwrap()
            .generate_on(&prog)
            .unwrap();
        generate_asm(&prog, &output_2).expect("failed to generate asm");
    }

    // prog.
    println!("{:#?}", comp_unit);

    Ok(())
}
