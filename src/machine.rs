use std::collections::HashMap;

use crate::exit_err;
use crate::value::Value;

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
    Begin,
    End,
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
    Concatenate,
    Length,
    Substring,
    IndexOf,
    Split,
    Join,
    ToLower,
    ToUpper,
    FormatTime,
    FormatNumber,
    SinFn,
    CosFn,
    TanFn,
    AsinFn,
    AcosFn,
    AtanFn,
    LogFn,
    Log10Fn,
    ExpFn,
    SqrtFn,
    IntFn,
    SubstrFn,
    SprintfFn,
    MatchFn,
    SubFn,
    GsubFn,
    RindexFn,
    SrandFn,
    RandFn,
    AndFn,
    Next,
    NextFile,
    Exit,
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

    pub fn execute_rem(&mut self) {
        // TODO: Implement boilerplate for REM
    }

    pub fn execute_exp(&mut self) {
        // TODO: Implement boilerplate for EXP
    }

    pub fn execute_shr(&mut self) {
        // TODO: Implement boilerplate for SHR
    }

    pub fn execute_shl(&mut self) {
        // TODO: Implement boilerplate for SHL
    }

    pub fn execute_eq(&mut self) {
        // TODO: Implement boilerplate for EQ
    }

    pub fn execute_ne(&mut self) {
        // TODO: Implement boilerplate for NE
    }

    pub fn execute_gt(&mut self) {
        // TODO: Implement boilerplate for GT
    }

    pub fn execute_ge(&mut self) {
        // TODO: Implement boilerplate for GE
    }

    pub fn execute_lt(&mut self) {
        // TODO: Implement boilerplate for LT
    }

    pub fn execute_le(&mut self) {
        // TODO: Implement boilerplate for LE
    }

    pub fn execute_and(&mut self) {
        // TODO: Implement boilerplate for AND
    }

    pub fn execute_or(&mut self) {
        // TODO: Implement boilerplate for OR
    }

    pub fn execute_incr(&mut self) {
        // TODO: Implement boilerplate for INCR
    }

    pub fn execute_decr(&mut self) {
        // TODO: Implement boilerplate for DECR
    }

    pub fn execute_pos(&mut self) {
        // TODO: Implement boilerplate for POS
    }

    pub fn execute_neg(&mut self) {
        // TODO: Implement boilerplate for NEG
    }

    pub fn execute_begin(&mut self) {
        // TODO: Implement boilerplate for BEGIN
    }

    pub fn execute_end(&mut self) {
        // TODO: Implement boilerplate for END
    }

    pub fn execute_exit(&mut self) {
        // TODO: Implement boilerplate for EXIT
    }

    pub fn exec_jump_if_false(&mut self) {
        if let Some(Value::Instruction(target)) = self.stack.pop() {
            if let Some(Value::Bool(false)) = self.stack.pop() {
                self.sp = target;
            }
        }
    }

    pub fn exec_jump_if_true(&mut self) {
        if let Some(Value::Instruction(target)) = self.stack.pop() {
            if let Some(Value::Bool(true)) = self.stack.pop() {
                self.sp = target;
            }
        }
    }

    pub fn exec_jump(&mut self) {
        if let Some(Value::Instruction(target)) = self.stack.pop() {
            self.sp = target;
        }
    }

    pub fn exec_load_variable(&mut self) {
        if let Some(Value::Identifier(variable_name)) = self.stack.pop() {
            if let Some(value) = self.environ.get(&variable_name) {
                self.stack.push(value.as_ref().unwrap().clone());
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
                self.stack.push(value.as_ref().unwrap().clone());
            } else {
                exit_err!(
                    "Error: either array `{}` or index `{}` don't exit, array_id",
                    array_id,
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
