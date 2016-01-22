use value::Value;

#[derive(Debug, Clone)]
pub enum OpCode {
    Val(Value), // stack.push(Value)
    Add,    // stack.pop() + stack.pop()
    Sub,    // stack.pop() - stack.pop()
    Mul,    // stack.pop() * stack.pop()
    Div,    // stack.pop() / stack.pop()
    Def,    // scopes[stack.pop()] = stack.pop()
    Ret,
    Call,   // stack.pop()(...)
    JumpIfNot(usize), // if !stack.pop() -> pc += jump
    JumpIf(usize),    // if stack.pop() -> pc += jump
    GetName(String),  // stack.push(scopes[String])
}
