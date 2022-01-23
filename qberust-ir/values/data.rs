use crate::symbol::{Symbol, Symbolic};
use crate::types::Type;
use crate::values::Constant;
use crate::visibility::Visibility;

use crate::types::PrimitiveType;

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
