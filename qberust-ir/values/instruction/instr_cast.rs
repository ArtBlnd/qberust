use crate::types::PrimitiveType;
use crate::values::Value;

enum CastKind {
    /// Zero Extend
    ZExt,

    // Signed Extend
    SExt,

    /// Integer Casting
    Integer,

    /// Floating Point Casting
    Fp,

    /// Truncate
    Trunc,
}

pub struct Cast {
    target: Box<Value>,

    cast_kind: CastKind,
    cast_type: PrimitiveType,
}

impl Cast {
    pub fn get_type(&self) -> PrimitiveType {
        self.cast_type.clone()
    }
}
