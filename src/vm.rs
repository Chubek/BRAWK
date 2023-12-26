use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::process::{exit, Command, Stdio};

use rand::SeedableRng;
use regex::Regex;

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
    Command(String, Vec<String>),
    ExecResult(String, i32),
    ArrayLiteral(HashMap<String, Box<Value>>),
    FilePath(String),
    BufferedReader(BufReader<Box<File>>),
    BufferedWriter(BufWriter<Box<File>>),
    PrintExpr(Vec<Box<Value>>),
    PrintFormattedExpr(String, Vec<Box<Value>>),
}

macro_rules! exit_err {
    ($reason:expr) => {
        eprintln!(expr);
        eprintln!("This caused RustyAWK to exit with status 1");
        exit(1)
    },

    ($reason:expr,*) {
        eprintln!(expr, *);
        eprintln!("This caused RustyAWK to exit with status 1");
        exit(1)
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
        unreachable!()
    }

    pub fn subtract(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a - b)),
            (Value::Float(a), Value::Float(b)) => Some(Value::Float(a - b)),
            _ => None,
        }
        unreachable!()    
    }

    pub fn multiply(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a * b)),
            (Value::Float(a), Value::Float(b)) => Some(Value::Float(a * b)),
            _ => None,
        }
        unreachable!()
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
        unreachable!()
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
        unreachable!()
    }

    pub fn equals(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Bool(a == b)),
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::StringLiteral(a), Value::StringLiteral(b)) => Some(Value::Bool(a == b)),
            _ => Some(Value::Bool(false)),
        }
        unreachable!()
    }

    pub fn not_equals(&self, other: &Value) -> Option<Value> {
        Value::Bool(!self.equals(other))
    }

    pub fn greater_than(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Bool(a > b)),
            (Value::Float(a), Value::Float(b)) => Some(Value::Bool(a > b)),
            _ => Some(Value::Bool(false)),
        }
        unreachable!()
    }

    pub fn greater_than_equals(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Bool(a >= b)),
            (Value::Float(a), Value::Float(b)) => Some(Value::Bool(a >= b)),
            _ => Some(Value::Bool(false)),
        }
        unreachable!()
    }

    pub fn less_than(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Bool(a < b)),
            (Value::Float(a), Value::Float(b)) => Some(Value::Bool(a < b)),
            _ => Some(Value::Bool(false)),
        }
        unreachable!()
    }

    pub fn less_than_equals(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Bool(a <= b)),
            (Value::Float(a), Value::Float(b)) => Some(Value::Bool(a <= b)),
            _ => Some(Value::Bool(false)),
        }
    }

    pub fn ere_match(&self, pattern: &Value) -> Option<Value> {
        match (self, pattern) {
            (Value::StringLiteral(input), Value::RegexPattern(regex)) => {
                let regex = regex::Regex::new(regex).ok()?;
                Some(Value::Bool(regex.is_match(input)))
            }
            _ => Some(Value::Bool(false)),
        }
        unreachable!()
    }

    pub fn ere_non_match(&self, pattern: &Value) -> Option<Value> {
        match (self, pattern) {
            (Value::StringLiteral(input), Value::RegexPattern(regex)) => {
                let regex = regex::Regex::new(regex).ok()?;
                Some(Value::Bool(!regex.is_match(input)))
            }
            _ => Some(Value::Bool(false)),
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
        unreachable!()
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
        unreachable!()
    }

    pub fn bitwise_not(&mut self) -> Option<Value> {
        match self {
            Some(Value::Number(ref mut n)) => Some(Value::Number(!(*n))),
            _ => None,
        }
        unreachable!()
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
        unreachable!()
    }

    pub fn bitwise_and(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a & b)),
            _ => None,
        }
        unreachable!()
    }

    pub fn bitwise_or(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a | b)),
            _ => None,
        }
        unreachable!()
    }

    pub fn bitwise_xor(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a ^ b)),
            _ => None,
        }
        unreachable!()
    }

    pub fn logical_or(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => {
                Some(Value::Number(if a != &0 || b != &0 { 1 } else { 0 }))
            }
            _ => None,
        }
        unreachable!()
    }

    pub fn logical_and(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => {
                Some(Value::Number(if a != &0 && b != &0 { 1 } else { 0 }))
            }
            _ => None,
        }
        unreachable!()
    }

    pub fn as_instruction(self) -> usize {
        if let Self::Instruction(instruction) = self {
            instruction
        } else {
            exit_err!("Value is not an instruction");
        }
        unreachable!()
    }

    pub fn exec_command(self) -> Option<Value> {
        if let Self::Command(command, args) = self {
            let output = Command::new(command).args(args).stdout(Stdio::piped()).spawn();

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

                    Some(Value::ExecResult(buffer, status))
                }
                Err(e) => {
                    exit_err!("Unexpected error: {}", e);
                }
            }
        } else {
            exit_err!("Value is not a command");
        }
        unreachable!()
    }

    pub fn open_file_for_read(self) -> Option<Value> {
        if let Self::FilePath(file_path) = self {
            let file = OpenOptions::new().open(file_path);
            let mut buff_reader = BufReader::new(file);
            Some(Self::BufferedReader(buff_reader))
        } else {
            exit_err!("Value is not a file path");
        }
        unreachable!()
    }

    pub fn open_file_for_write(self) -> Option<Value> {
        if let Self::FilePath(file_path) = self {
            let file = OpenOptions::new().create(true).append(true).open(file_path);
            let mut buff_writer = BufWriter::new(file);
            Some(Self::BufferedWriter(buff_writer))
        } else {
            exit_err!("Value is not a file path");
        }
        unreachable!()
    }

    pub fn read_line_from_file(self) -> Option<Value> {
        if let Self::BufferedReader(buff_reader) = self {
            let read_line = buff_reader.read_line().unwrap_or(String::new());
            Some(Value::String(read_line))
        } else {
            exit_err!("Value is not a buffered reader");
        }
        unreachable!()
    }

    pub fn read_all_from_file(self) -> Option<Value> {
        if let Self::BufferedReader(buff_reader) = self {
            let read_text = buff_reader.read_all().unwrap_or(String::new());
            Some(Value::String(read_text))
        } else {
            exit_err!("Value is not a buffered reader");
        }
        unreachable!()
    }

    pub fn write_to_file(self, text: String) {
        if let Self::BuffereWriter(buff_writer) = self {
            buff_writer.write_all(text);
        } else {
            exit_err!("Value is not a buffered writer");
        }
        unreachable!()
    }

    pub fn r#match(&self, pattern: &Value) -> Option<Value> {
        match (self, pattern) {
            (Value::StringLiteral(input), Value::StringLiteral(regex_str)) => {
                let regex = Regex::new(regex_str).ok()?;
                Some(Value::Bool(regex.is_match(input)))
            }
            _ => None,
        }
        unreachable!()
    }

    pub fn not_match(&self, pattern: &Value) -> Option<Value> {
        Some(Value::Bool(!self.r#match(pattern).unwrap_or(false)))
    }

    pub fn substitute(&mut self, regex: &Value, replacement: &Value) -> Option<()> {
        match (self, regex, replacement) {
            (
                Value::StringLiteral(input),
                Value::StringLiteral(regex_str),
                Value::StringLiteral(replacement_str),
            ) => {
                let regex = Regex::new(regex_str).ok()?;
                *input = regex.replace_all(input, replacement_str).to_string();
                Some(())
            }
            _ => None,
        }
        unreachable!()
    }

    pub fn gsub(&mut self, regex: &Value, replacement: &Value) -> Option<()> {
        match (self, regex, replacement) {
            (
                Value::StringLiteral(input),
                Value::StringLiteral(regex_str),
                Value::StringLiteral(replacement_str),
            ) => {
                let regex = Regex::new(regex_str).ok()?;
                *input = regex.replace_all(input, replacement_str).to_string();
                Some(())
            }
            _ => None,
        }
        unreachable!()
    }

    pub fn match_array(&self, regex: &Value, array: &Value) -> Option<Value> {
        match (self, regex, array) {
            (
                Value::StringLiteral(input),
                Value::StringLiteral(regex_str),
                Value::ArrayLiteral(array_map),
            ) => {
                let regex = Regex::new(regex_str).ok()?;
                let matches: Vec<_> = array_map
                    .values()
                    .filter_map(|value| match value {
                        Value::StringLiteral(s) => Some(s),
                        _ => None,
                    })
                    .filter(|s| regex.is_match(s))
                    .collect();

                Some(Value::Bool(!matches.is_empty()))
            }
            _ => None,
        }
        unreachable!()
    }

    pub fn non_match_array(&self, regex: &Value, array: &Value) -> Option<Value> {
        Some(Value::Bool(!self.match_array(regex, array).unwrap_or(false)))
    }

    pub fn pipe(&self, command: &Value) -> Option<Value> {
        match (self, command) {
            (Value::StringLiteral(input), Value::Command(cmd, args)) => {
                let output = Command::new(cmd)
                    .args(args)
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn();

                match output {
                    Ok(mut child) => {
                        let mut child_stdin = child.stdin.take().unwrap();
                        child_stdin.write_all(input.as_bytes()).unwrap();

                        let mut buffer = String::new();
                        child
                            .stdout
                            .take()
                            .unwrap()
                            .read_to_string(&mut buffer)
                            .unwrap();

                        let status = child.wait().unwrap();

                        Some(Value::StringLiteral(buffer))
                    }
                    Err(e) => {
                        exit_err!("Unexpected error: {}", e);
                    }
                }
            }
            _ => {
                exit_err!("Invalid usage of pipe operator");
            }
        }
        unreachable!()
    }


    pub fn rand(&self) -> Option<Value> {
        match self {
            Value::Number(seed) => {
                let mut rng = rand::thread_rng();
                Some(Value::Float(rng.gen_range(0.0..1.0)))
            }
            Value::Float(seed) => {
                let mut rng = rand::thread_rng();
                Some(Value::Float(rng.gen_range(0.0..1.0)))
            }
            _ => {
                exit_err!("Invalid usage of rand function");
            }
        }
        unreachable!()
    }

    pub fn srand(&self, seed: i32) -> Option<Value> {
        match self {
            Value::Number(_) => {
                let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64);
                Some(Value::Float(rng.gen_range(0.0..1.0)))
            }
            Value::Float(_) => {
                let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64);
                Some(Value::Float(rng.gen_range(0.0..1.0)))
            }
            _ => {
                exit_err!("Invalid usage of srand function");
            }
        }
        unreachable!()
    }

    pub fn index(&self, target: &Value) -> Option<Value> {
        match (self, target) {
            (Value::StringLiteral(source), Value::StringLiteral(pattern)) => {
                if let Some(position) = source.find(pattern) {
                    Some(Value::Number(position as i32 + 1))
                } else {
                    Some(Value::Number(0))
                }
            }
            _ => {
                exit_err!("Invalid usage of index function");
            }
        }
        unreachable!()
    }

    pub fn length(&self) -> Option<Value> {
        match self {
            Value::StringLiteral(s) => Some(Value::Number(s.len() as i32)),
            _ => {
                exit_err!("Invalid usage of length function");
            }
        }
        unreachable!()
    }

    pub fn split(&self, regex: &Value, array: &Value) -> Option<Value> {
        match (self, regex, array) {
            (
                Value::StringLiteral(input),
                Value::StringLiteral(regex_str),
                Value::ArrayLiteral(array_map),
            ) => {
                if let Ok(regex) = regex::Regex::new(regex_str) {
                    let split_values: Vec<_> = regex.split(input).map(|s| s.to_string()).collect();

                    
                    for (index, value) in split_values.into_iter().enumerate() {
                        array_map.insert(index.to_string(), Box::new(Value::StringLiteral(value)));
                    }

                    Some(Value::Number(split_values.len() as i32))
                } else {
                    exit_err!("Invalid regular expression in split function");
                }
            }
            _ => {
                exit_err!("Invalid usage of split function");
            }
        }
        unreachable!()
    }

    pub fn sub(&mut self, regex: &Value, replacement: &Value) -> Option<Value> {
        match (self, regex, replacement) {
            (
                Value::StringLiteral(input),
                Value::StringLiteral(regex_str),
                Value::StringLiteral(replacement_str),
            ) => {
                if let Ok(regex) = regex::Regex::new(regex_str) {
                    *input = regex.replace(input, replacement_str).to_string();
                    Some(Value::Number(1)) 
                } else {
                    exit_err!("Invalid regular expression in sub function");
                }
            }
            _ => {
                exit_err!("Invalid usage of sub function");
            }
        }
        unreachable!()
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
    environ: HashMap<String, Option<Value>>,
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
            .unwrap_or(Value::Instruction(0))
            .as_instruction();
    }

    pub fn exec_load_variable(&mut self) {
        if let Some(Value::Identifier(variable_name)) = self.stack.pop() {
            if let Some(value) = self.environ.get(&variable_name) {
                self.stack.push(value.unwrap().clone());
            } else {
                exit_err!("Error: variable `{}` not found", variable_name);
            }
        } else {
            exit_err!("Invalid operand type for LoadVariable");
        }
    }

    pub fn execute_store_variable(&mut self) {
        if self.stack.len() < 2 {
            exit_err!("Not enough operands on the stack for STORE_VARIABLE");
        }

        if let (Some(Value::Identifier(variable_name)), Some(value_to_store)) =
            (self.stack.pop(), self.stack.pop())
        {
            self.environ.insert(variable_name, Some(value_to_store));
        } else {
            exit_err!("Invalid operand types for STORE_VARIABLE");
        }
    }

    pub fn execute_load_associative_array_value(&mut self) {
        if self.stack.is_empty() {
            exit_err!("Not enough operands on the stack for LOAD_ASSOCIATIVE_ARRAY_VALUE");
        }

        if let Some(Value::AssociativeIdentifier(ref array_id, ref idx)) = self.stack.pop() {
            let mut key = array_id.clone();
            key.push_str(idx);

            if let Some(value) = self.environ.get(&key) {
                self.stack.push(value.unwrap().clone());
            } else {
                exit_err!(
                    "Error: either array `{}` or index `{}` don't exit, array_id",
                    idx
                );
            }
        } else {
            exit_err!("Invalid operand type for LOAD_ASSOCIATIVE_ARRAY_VALUE");
        }

        self.sp += 1;
    }

    pub fn execute_store_associative_array_value(&mut self) {
        if self.stack.len() < 2 {
            exit_err!("Not enough operands on the stack for STORE_ASSOCIATIVE_ARRAY_VALUE");
        }

        if let (Some(Value::AssociativeIdentifier(ref array_id, ref idx)), Some(value_to_store)) =
            (self.stack.pop(), self.stack.pop())
        {
            let mut key = array_id.clone();
            key.push_str(idx);

            self.environ.insert(key.clone(), Some(value_to_store));
        } else {
            exit_err!("Invalid operand types for STORE_ASSOCIATIVE_ARRAY_VALUE");
        }
    }

    pub fn exec_swap(&mut self) {
        if self.stack.len() < 2 {
            exit_err!("Not enough operands on the stack for SWAP");
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
            exit_err!("Cannot duplicate an empty stack");
        }
    }
}
