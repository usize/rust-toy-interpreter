use value::Value;

#[derive(Debug, Clone)]
pub enum OpCode {
    Val(Value), // stack.push(Value)
    Add,    // stack.pop() + stack.pop()
    Sub,    // stack.pop() - stack.pop()
    Mul,    // stack.pop() * stack.pop()
    Div,    // stack.pop() / stack.pop()
    EqEq,   // stack.pop() == stack.pop()
    NotEq,  // stack.pop() != stack.pop()
    Lt,     // <
    LtEq,   // <=
    Gt,     // >
    GtEq,   // >=
    Def,    // scopes[stack.pop()] = stack.pop()
    Ret,
    Call,   // stack.pop()(...)
    JumpIfNot(i32),   // if !stack.pop() -> pc += jump
    JumpIf(i32),      // if stack.pop() -> pc += jump
    Jump(i32),        // pc += jump
    GetName(String),  // stack.push(scopes[String])
}
