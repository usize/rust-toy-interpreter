use std::env;
use std::process;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

use value::Value;
use compiler::*;
use parser::*;
use vm::*;

mod compiler;
mod parser;
mod opcode;
mod lexer;
mod value;
mod ast;
mod vm;

extern crate readline;
extern crate cell_gc;

const VERSION: &'static str = "0.0.0";

fn main() {
    let mut parser = Parser::new();

    let args: Vec<String> = env::args().collect();

    // If there are args we're running a script.
    if args.len() > 1 {
        let filename = &args[1];
        let mut f =  match File::open(filename) {
            Ok(f) => f,
            Err(_) => panic!("Failed to open {}", filename)
        };
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        match parser.parse_lines(buf) {
            Err(msg) => println!("{}", msg),
            Ok(statements) => {
                let script = compile_script(statements);
                let mut vm = VM::new();
                let result = vm.run(&script);
                match result {
                    Ok(_) => (),
                    Err(msg) => panic!("Error: {}", msg)
                }
            }
        }
        process::exit(0);
    }

    println!("Harvey {} (github.com/mrrrgn/harvey)", VERSION);

    loop {
        let input = readline::readline("Harvey> ").unwrap();
        readline::add_history(&input);
        match parser.parse_lines(input.clone()) {
            Err(msg) => println!("{}", msg),
            Ok(statements) => {
                let script = compile_script(statements);
                let mut vm = VM::new();
                let result = vm.run(&script);
                match result {
                    Ok(Some(value)) => println!("{}", value),
                    Ok(None) => (),
                    Err(msg) => println!("Error: {}", msg)
                }
            }
        }
    }
}

macro_rules! assert_ok {
    ($e: expr) => (
        match $e {
            Ok(x) => x,
            Err(err) => panic!("{:?}", err)
        }
    )
}

#[cfg(test)]
fn eval(code: &str) -> Value {
    let mut parser = Parser::new();
    let ast = assert_ok!(parser.parse_lines(code.to_string()));
    let script = compile_script(ast);

    let mut vm = VM::new();

    assert_ok!(vm.run(&script)).expect("script did not produce a value")
}

#[test]
fn it_works() {
    /* binops */
    assert_eq!(eval("2 + 2"), Value::Number(4.0));
    assert_eq!(eval("0 / 0"), Value::Number(std::f64::NAN));
    assert_eq!(eval("\"hello\" == \"world\""), Value::Bool(false));
    assert_eq!(eval("\"hello\" != \"world\""), Value::Bool(true));
    assert_eq!(eval("\"a\" > \"b\""), Value::Bool(false));
    assert_eq!(eval("\"a\" < \"b\""), Value::Bool(true));
    assert_eq!(eval("\"a\" == 97"), Value::Bool(false));
    assert_eq!(eval("4 * 4 - 2 * 2 > 5/10 + 3"), Value::Bool(true));
    assert_eq!(eval("4 * 4 - 2 * 2 >= 5/10 + 3"), Value::Bool(true));
    assert_eq!(eval("1 >= 1"), Value::Bool(true));
    assert_eq!(eval("1 > 1"), Value::Bool(false));
    assert_eq!(eval("1 < 1"), Value::Bool(false));
    assert_eq!(eval("1 <= 1"), Value::Bool(true));
    assert_eq!(eval(r#""hello" + "world""#), Value::Str("helloworld".to_string()));
    /* whitespace */
    assert_eq!(eval("  1  "), Value::Number(1.0));
}
