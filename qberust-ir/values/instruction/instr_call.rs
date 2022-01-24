use crate::abi::ABI;
use crate::symbol::Symbol;
use crate::types::Type;

pub struct Call {
    symbol: Symbol,

    // types
    ret_ty: Type,
    arg_ty: Vec<Type>,

    // metadata
    has_variadic: bool,
    has_export: bool,
    calling_conv: ABI,
}
