mod instr_binary_op;
pub use instr_binary_op::*;
mod instr_phi;
pub use instr_phi::*;
mod instr_memory;
pub use instr_memory::*;
mod instr_cast;
pub use instr_cast::*;
mod instr_call;
pub use instr_call::*;

use crate::types::Type;

pub enum Instruction {
    Phi(Phi),
    BinaryOp(BinaryOp),
    MemoryOp(MemoryOp),
}

impl Instruction {
    pub fn get_type(&self) -> Type {
        match self {
            Instruction::Phi(v) => v.get_type(),
            Instruction::BinaryOp(v) => Type::Primitive(v.get_type()),
            Instruction::MemoryOp(v) => v.get_type(),
        }
    }
}
