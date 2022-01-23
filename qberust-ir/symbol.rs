use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SymbolInner {
    Global { global_ident: String },
    Module { id: usize },
    LocalBlock { id: usize },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    inner: SymbolInner,
}

pub trait Symbolic {
    fn get_symbol(&self) -> Symbol;
}
