use crate::symbol::Symbol;
use crate::types::{PrimitiveType, Type};

pub enum Constant {
    Integer32(u32),
    Float32(f32),
    Integer64(u64),
    Float64(f64),
    GlobalSymbol(Symbol),
}

impl Constant {
    pub fn get_type(&self) -> Type {
        Type::Primitive(match self {
            Constant::Integer32(_) => PrimitiveType::Word,
            Constant::Float32(_) => PrimitiveType::Single,
            Constant::Integer64(_) => PrimitiveType::Long,
            Constant::Float64(_) => PrimitiveType::Double,
            Constant::GlobalSymbol(_) => unimplemented!("cannot get type from global symbol!"),
        })
    }
}
