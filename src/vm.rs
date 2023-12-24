use std::collections::HashMap;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, BufWrite, BufWriter};
use std::fs::{OpenOptions, File};
use std::process::{exit, Command, Stdio};

#[derive(Debug, Clone)]
enum Value {
    Number(i32),
    Float(f64),
    Instruction(usize),
    Identifier(String),
    AssociativeIdentifier(String, String),
    StringLiteral(String),
    RegexPattern(String),
    Bool(bool),
    Commmand(String, Vec<String>),
    ArrayLiteral(HashMap<String, Box<Value>>),
    FilePath(String),
    BufferedReader(BufReader),
    BufferedWriter(BufWriter),
}

macro_rules! exit_err {
    ($reason:expr) => {
        eprintln!(expr);
        eprintln!("This caused RustyAWK to exit with status 1");
        exit(1);
    },

    ($reason:expr,*) {
        eprintln!(expr, *);
        eprintln!("This caused RustyAWK to exit with status 1");
        exit(1);
    }
}

impl Value {
    pub fn add(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a + b)),
            (Value::Float(a), Value::Float(b)) => Some(Value::Float(a + b)),
            (Value::StringLiteral(ref a), Value::StringLiteral(ref b)) => {
                let mut concatenated = a.clone();
                concatenated.push_str(b);
                Some(Value::StringLiteral(concatenated))
            }
            (Value::ArrayLiteral(ref a), Value::ArrayLiteral(ref b)) => {
                let mut concatenated = a.clone();
                concatenated.extend(b);
                Some(Value::ArrayLiteral(concatenated))
            }
            _ => None,
        }
    }

    pub fn subtract(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a - b)),
            (Value::Float(a), Value::Float(b)) => Some(Value::Float(a - b)),
            _ => None,
        }
    }

    pub fn multiply(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a * b)),
            (Value::Float(a), Value::Float(b)) => Some(Value::Float(a * b)),
            _ => None,
        }
    }

    pub fn divide(&self, other: &Value) -> Option<Value> {
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

    pub fn exponentiate(&self, other: &Value) -> Option<Value> {
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

    pub fn equals(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::StringLiteral(a), Value::StringLiteral(b)) => a == b,
            _ => false,
        }
    }

    pub fn not_equals(&self, other: &Value) -> bool {
        !self.equals(other)
    }

    pub fn greater_than(&self, other: &Value) -> Option<bool> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(a > b),
            (Value::Float(a), Value::Float(b)) => Some(a > b),
            _ => None,
        }
    }

    pub fn greater_than_equals(&self, other: &Value) -> Option<bool> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(a >= b),
            (Value::Float(a), Value::Float(b)) => Some(a >= b),
            _ => None,
        }
    }

    pub fn less_than(&self, other: &Value) -> Option<bool> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(a < b),
            (Value::Float(a), Value::Float(b)) => Some(a < b),
            _ => None,
        }
    }

    pub fn less_than_equals(&self, other: &Value) -> Option<bool> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(a <= b),
            (Value::Float(a), Value::Float(b)) => Some(a <= b),
            _ => None,
        }
    }

    pub fn ere_match(&self, pattern: &Value) -> Option<bool> {
        match (self, pattern) {
            (Value::StringLiteral(input), Value::RegexPattern(regex)) => {
                let regex = regex::Regex::new(regex).ok()?;
                Some(regex.is_match(input))
            }
            _ => None,
        }
    }

    pub fn ere_non_match(&self, pattern: &Value) -> Option<bool> {
        match (self, pattern) {
            (Value::StringLiteral(input), Value::RegexPattern(regex)) => {
                let regex = regex::Regex::new(regex).ok()?;
                Some(!regex.is_match(input))
            }
            _ => None,
        }
    }

    pub fn increment(&mut self) -> Option<()> {
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

    pub fn decrement(&mut self) -> Option<()> {
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

    pub fn bitwise_not(&mut self) -> Option<Value> {
        match self {
            Value::Number(ref mut n) => Some(Value::Number(!(*n))),
            _ => None,
        }
    }

    pub fn make_negative(&mut self) -> Option<()> {
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

    pub fn bitwise_and(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a & b)),
            _ => None,
        }
    }

    pub fn bitwise_or(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a | b)),
            _ => None,
        }
    }

    pub fn bitwise_xor(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a ^ b)),
            _ => None,
        }
    }

    pub fn logical_or(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => {
                Some(Value::Number(if a != &0 || b != &0 { 1 } else { 0 }))
            }
            _ => None,
        }
    }

    pub fn logical_and(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => {
                Some(Value::Number(if a != &0 && b != &0 { 1 } else { 0 }))
            }
            _ => None,
        }
    }

    pub fn as_instruction(self) -> usize {
        if Self::Instruction(instruction) = self {
            instruction
        } else {
            panic!("Value is not an instruction");
        }
    }

    pub fn exec_command(self) -> (String, i32) {
        if let Self::Command(command, args) = self {
            let output = Command::new(cmd).args(args).stdout(Stdio::piped()).spawn();

            match output {
                Ok(mut child) => {
                    let mut buffer = String::new();
                    child
                        .stdout
                        .take()
                        .unwrap()
                        .read_to_string(&mut buffer)
                        .unwrap();

                    let status = child.wait().unwrap();

                    (buffer, status)
                }
                Err(e) => {
                    exit_err!("Unexpected error: {}", e);
                }
            }
        } else {
            panic!("Value is not a command");
        }
    }
    
    pub fn open_file_for_read(self) -> Value {
        if let Self::FilePath(file_path) = self {
            let file = OpenOptions::new()
                 .open(file_path);
            let mut buff_reader = BufReader::new(file);
            Self::BufferedReader(buff_reader)
         } else {
            panic!("Value is not a file path");
         }
   }

    pub fn open_file_for_write(self) -> Value {
        if let Self::FilePath(file_path) = self {
            let file = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(file_path);
            let mut buff_writer = BufWriter::new(file);
            Self::BufferedWriter(buff_writer)
        }
    }

    pub fn read_line_from_file(self) -> Value {
        if let Self::BufferedReader(buff_reader) = self {
            let read_line = buff_reader
                .read_line()
                .unwrap_or(String::new());
            Value::String(read_line)
        } else {
            panic("Value is not a buffered reader");
        }
    }

    pub fn read_all_from_file(self) -> Value {
        if let Self::BufferedReader(buff_reader) = self {
            let read_text = buff_reader
                .read_all()
                .unwrap_or(String::new());
            Value::String(read_text)
        } else {
            panic!("Value is not a buffered reader");
        }
    }

    pub fn write_to_file(self, text: String) {
        if let Self::BuffereWriter(buff_writer) = self {
            buff_writer.write_all(text);
        } else {
            panic!("Value is not a buffered writer");
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
    LoadAssociativeArrayValue,
    StoreAssociativeArrayValue,
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
    OutputToFile,
    AppendToFile,
    Getline,
    OpenPipe,
    System,
    CloseStream,
    AppendOut,
    RegexMatch,
}

#[derive(Debug, Clone)]
struct StackVM {
    stack: Vec<Value>,
    program: Vec<Instruction>,
    environ: HashMap<String, Value>,
    pc: usize,
    sp: usize,
}

impl StackVM {
    pub fn new(program: Vec<Instruction>) -> Self {
        StackVM {
            stack: Vec::new(),
            program,
            pc: 0,
            sp: 0,
            environ: HashMap::new(),
        }
    }

    pub fn exec_jump_if_false(&mut self) {
        if let Some(Value::Instruction(target)) = self.stack.pop() {
            if let Some(Value::Bool(false)) = self.stack.pop() {
                self.sp = target as usize;
            }
        }
    }

    pub fn exec_jump_if_true(&mut self) {
        if let Some(Value::Instruction(target)) = self.stack.pop() {
            if let Some(Value::Bool(true)) = self.stack.pop() {
                self.sp = target as usize;
            }
        }
    }

    pub fn exec_jump(&mut self) {
        if let Some(Value::Instruction(target)) = self.stack.pop() {
            self.sp = target as usize;
        }
    }

    pub fn exec_return(&mut self) {
        self.sp = self
            .stack
            .pop()
            .and_then(|val| val.as_instruction())
            .unwrap_or(0) as usize;
    }

    pub fn exec_open_pipe(&mut self) {
        let command = self.stack.pop();
        let (result, status) = command.exec_command();

        self.environ.insert("STATUS", status);
        self.stack.push(Value::String(result));
    }

    pub fn exec_load_variable(&mut self) {
        if let Some(Value::Identifier(variable_name)) = self.stack.pop() {
            if let Some(value) = self.environ.get(&variable_name) {
                self.stack.push(value.clone());
            } else {
                exit_err!("Error: variable `{}` not found", variable_name);
                exit();
            }
        } else {
            panic!("Invalid operand type for LoadVariable");
        }
    }

    pub fn execute_store_variable(&mut self) {
        if self.stack.len() < 2 {
            panic!("Not enough operands on the stack for STORE_VARIABLE");
        }

        if let (Some(Value::Identifier(variable_name)), Some(value_to_store)) =
            (self.stack.pop(), self.stack.pop())
        {
            self.environ.insert(variable_name, value_to_store);
        } else {
            panic!("Invalid operand types for STORE_VARIABLE");
        }
    }

    pub fn execute_load_associative_array_value(&mut self) {
        if self.stack.is_empty() {
            panic!("Not enough operands on the stack for LOAD_ASSOCIATIVE_ARRAY_VALUE");
        }

        if let Some(Value::AssociativeIdentifier(ref array_id, ref idx)) = self.stack.pop() {
            let mut key = array_id.clone();
            key.push_str(idx);

            if let Some(value) = self.environ.get(&key) {
                self.stack.push(value.clone());
            } else {
                exit_err!(
                    "Error: either array `{}` or index `{}` don't exit, array_id",
                    idx
                );
            }
        } else {
            panic!("Invalid operand type for LOAD_ASSOCIATIVE_ARRAY_VALUE");
        }

        self.sp += 1;
    }

    pub fn execute_store_associative_array_value(&mut self) {
        if self.stack.len() < 2 {
            panic!("Not enough operands on the stack for STORE_ASSOCIATIVE_ARRAY_VALUE");
        }

        if let (Some(Value::AssociativeIdentifier(ref array_id, ref idx)), value_to_store) =
            (self.stack.pop(), self.stack.pop())
        {
            let mut key = array_id.clone();
            key.push_str(idx);

            self.environ.insert(key.clone(), value_to_store);
        } else {
            panic!("Invalid operand types for STORE_ASSOCIATIVE_ARRAY_VALUE");
        }
    }

    pub fn exec_swap(&mut self) {
        if self.stack.len() < 2 {
            panic!("Not enough operands on the stack for SWAP");
        }

        let top = self.stack.pop().unwrap();
        let second = self.stack.pop().unwrap();

        self.stack.push(top);
        self.stack.push(second);
    }

    pub fn exec_duplicate(&mut self) {
        if let Some(top) = self.stack.last().cloned() {
            self.stack.push(top);
        } else {
            panic!("Cannot duplicate an empty stack");
        }
    }
}
