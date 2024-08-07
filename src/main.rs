use std::{env, fs, io::{self, Write}};

use vm::{InterpretResult};

mod chunk;
mod debug;
mod value;
mod vm;
mod compile;
mod scanner;
mod object;

fn main()
{
    let vm = vm::init_vm();
    let _chunk: chunk::Chunk = chunk::init_chunk();

    let args: Vec<String> = env::args().collect();
    println!("args.len() = {}", args.len());
    if args.len() == 1 
    {
        repl(vm);
    } else if args.len() == 2
    {
        RunFile(&args[1]);
    } else {
        println!("Usage: rust_lox [path]");
    }
}

fn repl(mut vm: vm::VM)
{
    let mut line = String::new();
    loop 
    {
        print!("> ");
        io::stdout().flush();
        let _input = std::io::stdin().read_line(&mut line).unwrap();
        vm.interpret(line);
        line = String::new();
        //break;
    }
}

fn RunFile(path: &String)
{
    let _source = fs::read_to_string(path)
        .expect("Something went wrong reading the file");

    let result: u8 = InterpretResult::InterpretCompileError as u8; //interpret(source);

    if result == InterpretResult::InterpretCompileError as u8 { std::process::exit(65); }
    if result == InterpretResult::InterpretRuntimeError as u8 { std::process::exit(70); }
}
