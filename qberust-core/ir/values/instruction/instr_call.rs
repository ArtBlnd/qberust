use crate::abi::CallingConv;
use crate::ir::types::Type;
use crate::ir::values::ValueRef;
use crate::ir::symbol::Symbol;

pub struct Call {
    fn_symbol: Symbol,
    arguments: Vec<ValueRef>,

    abi: CallingConv,
}
