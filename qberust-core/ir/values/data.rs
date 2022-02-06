use crate::ir::types::Type;
use crate::ir::values::Constant;
use crate::ir::symbol::{Symbol, Symbolic};
use crate::abi::Visibility;

use crate::ir::types::PrimitiveType;

pub enum DataItem {
    Symbol {
        symbol: Symbol,
        offset: Option<usize>,
    },
    Bytes(Vec<u8>),
    Constant(Constant),
}

pub struct Data {
    visibility: Visibility,
    item: DataItem,
    symbol: Symbol,
}

impl Data {
    pub fn get_type(&self) -> PrimitiveType {
        todo!();
    }
}

impl Symbolic for Data {
    fn get_symbol(&self) -> Symbol {
        self.symbol.clone()
    }
}
