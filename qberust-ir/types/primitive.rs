use crate::alignment::Alignment;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveType {
    Void,     // Void Type
    Byte,     // A Byte
    HalfWord, // 2 Bytes Integer
    Word,     // 4 Bytes Integer
    Long,     // 8 Bytes Integer
    Single,   // 4 Bytes Floating Point
    Double,   // 8 Bytes Floating Point
}
