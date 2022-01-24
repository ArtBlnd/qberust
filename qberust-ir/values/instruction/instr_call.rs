use crate::abi::ABI;
use crate::symbol::Symbol;
use crate::types::Type;
use crate::values::ValueRef;

pub struct Call {
    fn_symbol: Symbol,
    arguments: Vec<ValueRef>,

    abi: ABI,
}
