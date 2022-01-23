use crate::control::Block;
use crate::symbol::{Symbol, Symbolic};
use crate::types::Type;
use crate::values::Value;

use std::collections::HashMap;

pub struct Phi {
    symbol: Symbol,
    ty: Type,
    pub(crate) incomings: HashMap<Block, Value>,
}

impl Phi {
    pub fn get_type(&self) -> Type {
        self.ty.clone()
    }
}

impl Symbolic for Phi {
    fn get_symbol(&self) -> Symbol {
        self.symbol.clone()
    }
}
