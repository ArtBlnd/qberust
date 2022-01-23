use crate::alignment::Alignment;

mod primitive;
pub use primitive::*;
mod aggregate;
pub use aggregate::*;

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
                PrimitiveType::Byte => 1,
                PrimitiveType::HalfWord => 2,
                PrimitiveType::Word => 4,
                PrimitiveType::Long => 8,
                PrimitiveType::Single => 4,
                PrimitiveType::Double => 8,
            },

            Type::Aggregate(ty) => match &ty.inner_types {
                AggregateTypeKind::Regular(ty) => {
                    // Append all size of inner types.
                    ty.iter().map(|v| v.size()).sum()
                }
                // Opaque has no base type
                // it only has opaque size hints.
                AggregateTypeKind::Opaque { size } => *size,
            },
        }
    }

    fn alignment(&self) -> Alignment {
        match self {
            Type::Primitive(_) => Alignment::Inherited,
            Type::Aggregate(ty) => ty.align,
        }
    }

    pub fn get_primitve_type(&self) -> Option<&PrimitiveType> {
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
