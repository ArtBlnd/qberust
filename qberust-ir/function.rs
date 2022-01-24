use crate::abi::ABI;
use crate::symbol::{Symbol, Symbolic};
use crate::types::Type;

pub struct Function {
    symbol: Symbol,

    // types
    ret_ty: Type,
    arg_ty: Vec<Type>,

    // metadata
    has_variadic: bool,
    has_export: bool,
    abi: ABI,
}

impl Symbolic for Function {
    fn get_symbol(&self) -> Symbol {
        self.symbol.clone()
    }
}
