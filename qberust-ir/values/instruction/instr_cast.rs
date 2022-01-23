use crate::types::PrimitiveType;
use crate::values::Value;

enum Cast {
    ZeroExtend {
        target: Box<Value>,
        cast_to: PrimitiveType,
    },
    SignedExtend {
        target: Box<Value>,
        cast_to: PrimitiveType,
    },
}

impl Cast {
    pub fn get_type(&self) -> PrimitiveType {
        match self {
            Cast::ZeroExtend { cast_to, .. } | Cast::SignedExtend { cast_to, .. } => {
                cast_to.clone()
            }
        }
    }
}
