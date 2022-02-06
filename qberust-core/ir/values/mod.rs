mod constant;
pub use constant::*;
mod data;
pub use data::*;
mod value_table;
pub use value_table::*;
mod value_ref;
pub use value_ref::*;

pub mod instruction;
use instruction::Instruction;

use crate::ir::types::Type;

use std::ops::{Deref, DerefMut};

pub enum Value {
    Constant(Constant),
    Data(Data),
    Instruction(Instruction),
}

impl Value {
    pub fn get_type(&self) -> Type {
        match self {
            Value::Constant(v) => v.get_type(),
            Value::Data(v) => todo!(),
            Value::Instruction(v) => v.get_type(),
        }
    }
}
