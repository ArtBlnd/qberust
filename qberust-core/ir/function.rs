use crate::abi::CallingConv;
use crate::ir::types::Type;
use crate::symbol::{Symbol, Symbolic};

pub struct Function {
    symbol: Symbol,

    // types
    ret_ty: Type,
    arg_ty: Vec<Type>,

    // metadata
    has_variadic: bool,
    has_export: bool,
    calling_conv: CallingConv,
}

impl Symbolic for Function {
    fn get_symbol(&self) -> Symbol {
        self.symbol.clone()
    }
}
