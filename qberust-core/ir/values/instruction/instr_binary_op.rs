use crate::ir::types::PrimitiveType;
use crate::ir::values::Value;

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

    /// Integer Equal
    ICmpEq,

    /// Integer Not Equal
    ICmpNe,

    /// Integer Unsigned Greater Than
    ICmpUgt,

    /// Integer Unsigned Greater Than or Equal
    ICmpUge,

    /// Integer Unsigned Less Than
    ICmpUlt,

    /// Integer Unsigned Less Than or Equal
    ICmpUle,

    /// Integer Signed Greater Than
    ICmpSgt,

    /// Integer Signed Greater Than or Equal
    ICmpSge,

    /// Integer Signed Less Than
    ICmpSlt,

    /// Integer Signed Less Than or Equal
    ICmpSle,

    /// Floating Ordered Equal
    FCmpOeq,

    /// Floating Ordered Greater Than
    FCmpOgt,

    /// Floating Ordered Greater Than or Equal
    FCmpOge,

    /// Floating Ordered Less Than
    FCmpOlt,

    /// Floating Ordered Less Than or Equal
    FCmpOle,

    /// Floating Ordered and Operands Unequal.
    FCmpOne,

    /// Floating Ordered
    FCmpOrd,

    /// Floating Unordered
    FCmpUno,

    /// Floating Unordered Equal
    FCmpUeq,

    /// Floating Unordered Greater Than
    FCmpUgt,

    /// Floating Unordered Greater Than or Equal
    FCmpUge,

    /// Floating Unordered Less Than
    FCmpUlt,

    /// Floating Unordered Less Than or Equal
    FCmpUle,

    /// Floating Unordered Not Equal
    FCmpUne,
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

        lhs.get_type().get_primitive_type().unwrap().clone()
    }

    pub fn get_rhs(&self) -> &Value {
        &self.rhs
    }

    pub fn get_lhs(&self) -> &Value {
        &self.rhs
    }

    pub fn is_op_arithmetic(&self) -> bool {
        todo!();
    }

    pub fn is_op_cmp(&self) -> bool {
        todo!();
    }
}
