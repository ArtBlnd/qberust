use crate::alignment::Alignment;

mod primitive;
pub use primitive::*;
mod aggregate;
pub use aggregate::*;

use std::fmt::Result as FmtResult;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Primitive(PrimitiveType),
    Aggregate(AggregateType),
}

impl Type {
    fn size(&self) -> usize {
        match self {
            Type::Primitive(ty) => match ty {
                PrimitiveType::Void => 0,

                PrimitiveType::U8 | PrimitiveType::I8 => 1,
                PrimitiveType::U16 | PrimitiveType::I16 => 2,
                PrimitiveType::U32 | PrimitiveType::I32 => 4,
                PrimitiveType::U64 | PrimitiveType::I64 => 8,

                PrimitiveType::F32 => 4,
                PrimitiveType::F64 => 8,
            },

            Type::Aggregate(ty) => match &ty.inner_types {
                AggregateTypeKind::Regular(ty) => {
                    // Append all size of inner types.
                    ty.iter().map(|v| v.size()).sum()
                }
                // Opaque has no base type
                // it only has opaque size hints.
                AggregateTypeKind::Opaque { size } => *size,
                AggregateTypeKind::Pointer { size, .. } => *size,
            },
        }
    }

    pub fn is_primitive(&self) -> bool {
        self.get_primitive_type().is_some()
    }

    pub fn is_aggregate(&self) -> bool {
        self.get_aggregate_type().is_some()
    }

    pub fn alignment(&self) -> Alignment {
        match self {
            Type::Primitive(_) => Alignment::Inherited,
            Type::Aggregate(ty) => ty.align,
        }
    }

    pub fn get_primitive_type(&self) -> Option<&PrimitiveType> {
        if let Self::Primitive(ty) = self {
            Some(ty)
        } else {
            None
        }
    }

    pub fn get_aggregate_type(&self) -> Option<&AggregateType> {
        if let Self::Aggregate(ty) = self {
            Some(ty)
        } else {
            None
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        todo!()
    }
}
