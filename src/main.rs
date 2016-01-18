use std::collections::HashMap;
use std::io::{self, Write};

use compiler::*;
use parser::*;
use vm::*;

mod compiler;
mod parser;
mod opcode;
mod object;
mod lexer;
mod value;
mod vm;

macro_rules! weak_try {
    ($func:expr) => {
        match $func {
            Ok(_) => (),
            Err(err) => println!("{:?}", err)
        };
    }
}

const VERSION: &'static str = "0.0.0";

fn main() {
    println!("Harvey {} (github.com/mrrrgn/harvey)", VERSION);

    let mut vm = VM::new();
    let mut parser = Parser::new();
    let mut scopes = HashMap::new();

    loop {
        let mut buffer = String::new();
        weak_try!(io::stdout().write(b"Harvey> "));
        weak_try!(io::stdout().flush());

        for _ in io::stdin().read_line(&mut buffer) {
            match parser.parse_lines(buffer.clone()) {
                Err(msg) => println!("{}", msg),
                Ok(statements) => {
                    println!("Parser: {:?}", &statements);
                    let script = compile_script(statements);
                    vm.load(script);
                    vm.run(&mut scopes);
                    println!("VM: stack: {:?}, program: {:?}", vm.stack(), vm.program());
                }
            }
        }
    }
}
