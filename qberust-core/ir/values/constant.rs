use crate::ir::types::{PrimitiveType, Type};
use crate::ir::symbol::Symbol;

pub enum Constant {
    I64(i64),
    U64(u64),
    F64(f64),
    I32(i32),
    U32(u32),
    F32(f32),
    GlobalSymbol(Symbol),
}

impl Constant {
    pub fn get_type(&self) -> Type {
        Type::Primitive(match self {
            Constant::I64(_) => PrimitiveType::I64,
            Constant::U64(_) => PrimitiveType::U64,
            Constant::F64(_) => PrimitiveType::F64,
            Constant::I32(_) => PrimitiveType::I32,
            Constant::U32(_) => PrimitiveType::U32,
            Constant::F32(_) => PrimitiveType::F32,
            Constant::GlobalSymbol(_) => unimplemented!("cannot get type from global symbol!"),
        })
    }
}
