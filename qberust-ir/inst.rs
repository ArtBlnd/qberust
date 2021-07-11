use crate::Value;
use crate::Types;
use crate::ValueKind;

pub enum InstructionKind {
    BinaryOp(BinaryOpKind),
    UnaryOp(UnaryOpKind)
}

pub struct Instruction {
    kind: InstructionKind,
    oper: Vec<Value>,
}

impl Instruction {
    // returns instruction type of value
    pub fn instr_kind(&self) -> &InstructionKind {
        &self.kind
    }

    // Operand helpers.
    pub fn get_oper_lhs(&self) -> Option<&Value> {
        self.get_oper(0)
    }

    pub fn get_oper_rhs(&self) -> Option<&Value> {
        self.get_oper(1)
    }

    pub fn get_oper(&self, n: usize) -> Option<&Value> {
        self.oper.get(n)
    }

    pub fn get_oper_mut(&mut self, n: usize) -> Option<&mut Value> {
        self.oper.get_mut(n)
    }

    pub fn set_oper_n(&mut self, n: usize, oper: Value) -> Option<Value> {
        if self.oper.len() < n {
            Some(std::mem::replace(self.oper.get_mut(n).unwrap(), oper))
        } else {
            None
        }
    }

    pub fn pop_oper(&mut self) -> Option<Value> {
        self.oper.pop()
    }
}

impl Into<Value> for Instruction {
    fn into(self) -> Value {
        Value { 
            span: "instruction",
            kind: ValueKind::Instruction(self)
        }
    }
}

pub enum BinaryOpKind {
    Add, Sub, Mul, Div
}

pub enum UnaryOpKind {
    Inc, Dec
}

pub enum MemoryInstKind {
    Load(Types),
    Store(Types),
    Alloc4,
    Alloc8,
    Alloc16
}

pub struct MemoryInst {
    kind: MemoryInstKind,
}