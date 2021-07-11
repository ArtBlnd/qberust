use crate::{Instruction};
use qberust_core::IRError;

use std::convert::TryInto;

pub enum ValueKind {
    Instruction(Instruction),
    Constant()
}

pub struct Value {
    pub(crate) span: &'static str,
    pub(crate) kind: ValueKind
}

impl TryInto<Instruction> for Value {
    type Error = IRError;

    fn try_into(self) -> Result<Instruction, Self::Error> {
        if let ValueKind::Instruction(instr) = self.kind {
            return Ok(instr);
        }

        return Err(IRError::BadTranformType(self.span, "instruction"))
    }
}

