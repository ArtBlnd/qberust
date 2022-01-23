use crate::types::PrimitiveType;
use crate::values::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOpKind {
    Add,
    Sub,
    Div,
    UnsignedDiv,
    Mul,
    Mod,
    UnsignedMod,
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
    Sar,
    Shr,
    Shl,
}

pub struct BinaryOp {
    kind: BinaryOpKind,
    lhs: Box<Value>,
    rhs: Box<Value>,
}

impl BinaryOp {
    pub fn get_kind(&self) -> BinaryOpKind {
        self.kind
    }

    pub fn get_type(&self) -> PrimitiveType {
        let lhs = self.get_lhs();
        let rhs = self.get_rhs();
        assert!(lhs.get_type() == rhs.get_type());

        todo!();
    }

    pub fn get_rhs(&self) -> &Value {
        &self.rhs
    }

    pub fn get_lhs(&self) -> &Value {
        &self.rhs
    }
}
