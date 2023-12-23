use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Value {
    Number(i32),
    Float(f64),
    Instruction(usize),
    StringLiteral(String),
    RegexPattern(String),
    Bool(bool),
}

impl Value {
    fn add(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a + b)),
            (Value::Float(a), Value::Float(b)) => Some(Value::Float(a + b)),
            (Value::StringLiteral(ref a), Value::StringLiteral(ref b)) => {
                let mut concatenated = a.clone();
                concatenated.push_str(b);
                Some(Value::StringLiteral(concatenated))
            }
            _ => None,
        }
    }

    fn subtract(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a - b)),
            (Value::Float(a), Value::Float(b)) => Some(Value::Float(a - b)),
            _ => None,
        }
    }

    fn multiply(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a * b)),
            (Value::Float(a), Value::Float(b)) => Some(Value::Float(a * b)),
            _ => None,
        }
    }

    fn divide(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => {
                if b != 0 {
                    Some(Value::Number(a / b))
                } else {
                    None
                }
            }
            (Value::Float(a), Value::Float(b)) => {
                if b != 0.0 {
                    Some(Value::Float(a / b))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn exponentiate(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(base), Value::Number(exponent)) => {
                Some(Value::Number(base.pow(*exponent as u32)))
            }
            (Value::Float(base), Value::Float(exponent)) => {
                Some(Value::Float(base.powf(*exponent)))
            }
            _ => None,
        }
    }

    fn equals(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::StringLiteral(a), Value::StringLiteral(b)) => a == b,
            _ => false,
        }
    }

    fn not_equals(&self, other: &Value) -> bool {
        !self.equals(other)
    }

    fn greater_than(&self, other: &Value) -> Option<bool> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(a > b),
            (Value::Float(a), Value::Float(b)) => Some(a > b),
            _ => None,
        }
    }

    fn greater_than_equals(&self, other: &Value) -> Option<bool> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(a >= b),
            (Value::Float(a), Value::Float(b)) => Some(a >= b),
            _ => None,
        }
    }

    fn less_than(&self, other: &Value) -> Option<bool> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(a < b),
            (Value::Float(a), Value::Float(b)) => Some(a < b),
            _ => None,
        }
    }

    fn less_than_equals(&self, other: &Value) -> Option<bool> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(a <= b),
            (Value::Float(a), Value::Float(b)) => Some(a <= b),
            _ => None,
        }
    }

    fn ere_match(&self, pattern: &Value) -> Option<bool> {
        match (self, pattern) {
            (Value::StringLiteral(input), Value::RegexPattern(regex)) => {
                let regex = regex::Regex::new(regex).ok()?;
                Some(regex.is_match(input))
            }
            _ => None,
        }
    }

    fn ere_non_match(&self, pattern: &Value) -> Option<bool> {
        match (self, pattern) {
            (Value::StringLiteral(input), Value::RegexPattern(regex)) => {
                let regex = regex::Regex::new(regex).ok()?;
                Some(!regex.is_match(input))
            }
            _ => None,
        }
    }

    fn increment(&mut self) -> Option<()> {
        match self {
            Value::Number(ref mut n) => {
                *n = n.checked_add(1)?;
                Some(())
            }
            Value::Float(ref mut f) => {
                *f += 1.0;
                Some(())
            }
            _ => None,
        }
    }

    fn decrement(&mut self) -> Option<()> {
        match self {
            Value::Number(ref mut n) => {
                *n = n.checked_sub(1)?;
                Some(())
            }
            Value::Float(ref mut f) => {
                *f -= 1.0;
                Some(())
            }
            _ => None,
        }
    }

    fn bitwise_not(&mut self) -> Option<Value> {
        match self {
            Value::Number(ref mut n) => Some(Value::Number(!(*n))),
            _ => None,
        }
    }

    fn make_negative(&mut self) -> Option<()> {
        match self {
            Value::Number(ref mut n) => {
                *n = -(*n);
                Some(())
            }
            Value::Float(ref mut f) => {
                *f = -(*f);
                Some(())
            }
            _ => None,
        }
    }

    fn bitwise_and(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a & b)),
            _ => None,
        }
    }

    fn bitwise_or(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a | b)),
            _ => None,
        }
    }

    fn bitwise_xor(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a ^ b)),
            _ => None,
        }
    }

    fn logical_or(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => {
                Some(Value::Number(if a != &0 || b != &0 { 1 } else { 0 }))
            }
            _ => None,
        }
    }

    fn logical_and(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => {
                Some(Value::Number(if a != &0 && b != &0 { 1 } else { 0 }))
            }
            _ => None,
        }
    }

    fn as_instruction(self) -> usize {
        if Self::Instruction(instruction) = self {
            instruction
        } else {
            panic!("Value is not an instruction");
        }
    }
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
    Exp,
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
    Incr,
    Decr,
    Pos,
    Neg,
    EreMatch,
    EreNonMatch,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
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

    fn jump_if_false(&mut self) {
        if let Some(Value::Instruction(target)) = self.stack.pop() {
            if let Some(Value::Bool(false)) = self.stack.pop() {
                self.pc = target as usize;
            }
        }
    }

    fn jump_if_true(&mut self) {
        if let Some(Value::Instruction(target)) = self.stack.pop() {
            if let Some(Value::Bool(true)) = self.stack.pop() {
                self.pc = target as usize;
            }
        }
    }

    fn jump(&mut self) {
        if let Some(Value::Instruction(target)) = self.stack.pop() {
            self.pc = target as usize;
        }
    }

    fn return_instruction(&mut self) {
        self.pc = self
            .stack
            .pop()
            .and_then(|val| val.as_instruction())
            .unwrap_or(0) as usize;
    }
}
