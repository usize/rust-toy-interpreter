use opcode::OpCode;

#[derive(Debug, Clone)]
pub enum Object {
    Function{body: Vec<OpCode>, args: Vec<String>}
}
