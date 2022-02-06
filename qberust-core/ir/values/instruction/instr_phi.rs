use crate::ir::control::Block;
use crate::ir::types::Type;
use crate::ir::values::Value;
use crate::symbol::{Symbol, Symbolic};

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
