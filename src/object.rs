use opcode::OpCode;

#[derive(Debug, Clone)]
pub enum Object {
    Function{args: Vec<String>, body: Vec<OpCode>}
}
