use crate::abi::Alignment;
use crate::ir::types::Type;
use crate::ir::symbol::{Symbol, Symbolic};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AggregateTypeKind {
    Regular(Vec<Type>),
    Opaque { size: usize },
    Pointer { size: usize, base_ty: Box<Type> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AggregateType {
    pub(crate) symbol: Symbol,
    pub(crate) align: Alignment,
    pub(crate) inner_types: AggregateTypeKind,
}

impl Symbolic for AggregateType {
    fn get_symbol(&self) -> Symbol {
        self.symbol.clone()
    }
}
