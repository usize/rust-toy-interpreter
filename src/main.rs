#![allow(non_snake_case)]

use std::env;
use std::process;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

use value::Value;
use object::{Object, Native};
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

extern crate readline;

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
    let mut vm = VM::new();
    let mut parser = Parser::new();
    let mut scopes = HashMap::new();

    fn pr_native(args: Vec<Value>) -> Value {
        println!("{:?}", args);
        return Value::Undefined;
    }

    // insert a handy native print method
    scopes.insert("print".to_string(),
                  Value::Object(Object::Native(Native::Function(pr_native))));


    let args: Vec<String> = env::args().collect();
    // let's run a script !
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
                vm.load(script);
                let result = vm.run(&mut scopes);
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
                vm.load(script);
                let result = vm.run(&mut scopes);
                match result {
                    Ok(Some(value)) => println!("{:?}", value),
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
    vm.load(script);

    let mut scopes = HashMap::new();
    assert_ok!(vm.run(&mut scopes)).expect("script did not produce a value")
}

#[test]
fn it_works() {
    assert_eq!(eval("2 + 2"), Value::Number(4.0));
    assert_eq!(eval("0 / 0"), Value::Number(std::f64::NAN));
    assert_eq!(eval(r#""hello" + "world""#), Value::Str("helloworld".to_string()));
    assert_eq!(eval("  1  "), Value::Number(1.0));
    assert_eq!(eval("let x = 101; x;"), Value::Number(101.0));
    assert_eq!(eval("let x = 0; while (100 - x) { x = x + 1; }; x;"),
               Value::Number(100.0));
}
