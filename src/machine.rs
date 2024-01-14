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
    Mod,
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
    stack: Vec<Option<Value>>,
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

    pub fn exec_add(&mut self) {
        if self.stack.len() < 2 {
            exit_err!("Not enough operands on the stack for ADD");
        }

        let (left, right) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
        self.stack.push(left + right);
    }

    pub fn exec_sub(&mut self) {
        if self.stack.len() < 2 {
            exit_err!("Not enough operands on the stack for SUB");
        }

        let (left, right) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
        self.stack.push(left - right);
    }

    pub fn exec_mul(&mut self) {
        if self.stack.len() < 2 {
            exit_err!("Not enough operands on the stack for MUL");
        }

        let (left, right) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
        self.stack.push(left * right);
    }

    pub fn execute_div(&mut self) {
        if self.stack.len() < 2 {
            exit_err!("Not enough operands on the stack for DIV");
        }

        let (left, right) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());

        // Ensure that division by zero is handled
        if right == 0 {
            exit_err!("Division by zero");
        }

        self.stack.push(left / right);
    }

    pub fn execute_mod(&mut self) {
        if self.stack.len() < 2 {
            exit_err!("Not enough operands on the stack for MOD");
        }

        let (left, right) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());

        if right == 0 {
            exit_err!("Modulo by zero");
        }

        self.stack.push(left % right);
    }

    pub fn execute_exp(&mut self) {
        if self.stack.len() < 2 {
            exit_err!("Not enough operands on the stack for EXP");
        }

        let (base, exponent) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
        self.stack.push(base.pow(exponent));
    }

    pub fn execute_shr(&mut self) {
        if self.stack.len() < 2 {
            exit_err!("Not enough operands on the stack for SHR");
        }

        let (value, shift) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
        self.stack.push(value >> shift);
    }

    pub fn execute_shl(&mut self) {
        if self.stack.len() < 2 {
            exit_err!("Not enough operands on the stack for SHL");
        }

        let (value, shift) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
        self.stack.push(value << shift);
    }

    pub fn execute_eq(&mut self) {
        if self.stack.len() < 2 {
            exit_err!("Not enough operands on the stack for EQ");
        }

        let (left, right) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
        self.stack.push(Value::Bool(left == right));
    }

    pub fn execute_ne(&mut self) {
        if self.stack.len() < 2 {
            exit_err!("Not enough operands on the stack for NE");
        }

        let (left, right) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
        self.stack.push(Value::Bool(left != right));
    }

    pub fn execute_gt(&mut self) {
        if self.stack.len() < 2 {
            exit_err!("Not enough operands on the stack for GT");
        }

        let (left, right) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
        self.stack.push(Value::Bool(left > right));
    }

    pub fn execute_ge(&mut self) {
        if self.stack.len() < 2 {
            exit_err!("Not enough operands on the stack for GE");
        }

        let (left, right) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
        self.stack.push(Value::Bool(left >= right));
    }

    pub fn execute_lt(&mut self) {
        if self.stack.len() < 2 {
            exit_err!("Not enough operands on the stack for LT");
        }

        let (left, right) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
        self.stack.push(Value::Bool(left < right));
    }

    pub fn execute_le(&mut self) {
        if self.stack.len() < 2 {
            exit_err!("Not enough operands on the stack for LE");
        }

        let (left, right) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
        self.stack.push(Value::Bool(left <= right));
    }

    pub fn execute_and(&mut self) {
        if self.stack.len() < 2 {
            exit_err!("Not enough operands on the stack for AND");
        }

        let (left, right) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
        self.stack.push(Value::Bool(left & right));
    }

    pub fn execute_or(&mut self) {
        if self.stack.len() < 2 {
            exit_err!("Not enough operands on the stack for OR");
        }

        let (left, right) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
        self.stack.push(Value::Bool(left | right));
    }

    pub fn execute_incr(&mut self) {
        if self.stack.len() < 1 {
            exit_err!("Not enough operands on the stack for INCR");
        }

        let operand = self.stack.pop();
        self.stack.push(operand.increment());
    }

    pub fn execute_decr(&mut self) {
        if self.stack.len() < 1 {
            exit_err!("Not enough operands on the stack for INCR");
        }

        let operand = self.stack.pop();
        self.stack.push(operand.decerement());
    }

    pub fn execute_pos(&mut self) {
        if self.stack.len() < 1 {
            exit_err!("Not enough operands on the stack for INCR");
        }

        let operand = self.stack.pop();
        self.stack.push(operand);

    }

    pub fn execute_neg(&mut self) {
        if self.stack.len() < 1 {
            exit_err!("Not enough operands on the stack for INCR");
        }

        let operand = self.stack.pop();
        self.stack.push(-operand);


    }

    pub fn execute_begin(&mut self) {
        // TODO: Implement BEGIN
    }

    pub fn execute_end(&mut self) {
        // TODO: Implement END
    }

    pub fn execute_exit(&mut self) {
        if self.stack.len() > 0 {
            let exit_reason = self.stack.pop().unwrap();
            exit_reason.exit();
        } else {
            std::process::exit(1);
        }
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
            (self.stack.pop().unwrap(), self.stack.pop().unwrap())
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
            (self.stack.pop().unwrap(), self.stack.pop().unwrap())
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
