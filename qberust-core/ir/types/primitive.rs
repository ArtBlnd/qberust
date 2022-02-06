#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    Void, // Void Type

    /// Unsigned Values
    U8, // A Byte
    U16, // 2 Bytes Integer
    U32, // 4 Bytes Integer
    U64, // 8 Bytes Integer

    /// Signed Values
    I8, // A Bytes
    I16, // 2 Bytes Integer
    I32, // 4 Bytes Integer
    I64, // 8 Bytes Integer

    F32, // 4 Bytes Floating Point
    F64, // 8 Bytes Floating Point
}

impl PrimitiveType {
    pub fn is_integer(&self) -> bool {
        match self {
            Self::I8
            | Self::I16
            | Self::I32
            | Self::I64
            | Self::U8
            | Self::U16
            | Self::U32
            | Self::U64 => true,
            _ => false,
        }
    }

    pub fn is_floating(&self) -> bool {
        match self {
            Self::F32 | Self::F64 => true,
            _ => false,
        }
    }

    pub fn is_unsigned(&self) -> bool {
        match self {
            Self::U8 | Self::U16 | Self::U32 | Self::U64 => true,
            _ => false,
        }
    }
}
