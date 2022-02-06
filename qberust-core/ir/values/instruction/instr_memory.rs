use crate::ir::types::{PrimitiveType, Type};
use crate::ir::values::{Value, ValueRef};

pub enum MemoryOp {
    Store { src: ValueRef, dst: ValueRef },
    Load { ty: PrimitiveType, src: ValueRef },
    StackAlloc { ty: PrimitiveType },
    Assign { src: Box<Value> },
}

impl MemoryOp {
    pub fn get_type(&self) -> Type {
        Type::Primitive(match self {
            MemoryOp::Store { .. } => PrimitiveType::Void,
            MemoryOp::Load { ty, .. } | MemoryOp::StackAlloc { ty } => ty.clone(),
            MemoryOp::Assign { src } => return src.get_type(),
        })
    }
}
