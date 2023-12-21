use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Value {
    Variable(String),
    Number(i32),
    Float(f64),
    StringLiteral(String),
    RegexPattern(String),
}

#[derive(Debug, Clone)]
enum Instruction {
    PushValue,
    FunctionCall,
    JumpIfFalse,
    JumpIfTrue,
    Jump,
    Return,
    LoadVariable,
    StoreVariable,
    Pop,
    Duplicate,
    Swap,
    Add,
    Sub,
    Mul,
    Rem,
    Div,
    Shr,
    Shl,
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
    And,
    Or,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    Print,
    Printf,
    Getline,
    ExecShell,
    RedirOut,
    AppendOut,
    RegexMatch,
}


#[derive(Debug, Clone)]
struct StackVM {
    stack: Vec<Value>,
    program: Vec<Instruction>,
    environ: HashMap<String, Value>,
    pc: usize,
}

impl StackVM {
    fn new(program: Vec<Instruction>) -> Self {
        StackVM {
            stack: Vec::new(),
            program,
            pc: 0,
            environ: HashMap::new(),
        }
    }

}
