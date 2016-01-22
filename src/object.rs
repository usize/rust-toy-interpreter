use std::fmt;

use opcode::OpCode;
use value::Value;

#[derive(Clone)]
pub enum Native {
    Function(fn(Vec<Value>) -> Value)
}

impl fmt::Debug for Native {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Native]")
    }
}

#[derive(Debug, Clone)]
pub enum Object {
    Function{args: Vec<String>, body: Vec<OpCode>},
    Native(Native),
}


