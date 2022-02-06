use std::hash::Hash;
use std::sync::Arc;

use parking_lot::Mutex;

/// Symbolic trait
/// object that has Symbolic trait always have to own unique Symbol.
pub trait Symbolic {
    fn get_symbol(&self) -> Symbol;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SymbolInner {
    /// Global symbol
    /// this symbol is resolved when linking
    Global { global_ident: String },

    /// File-Level scoped symbol
    /// Module symbols are always can be accessed anywhere.
    Module { id: usize },

    /// Function scoped symbol.
    /// BasicBlock has own SymbolTable.
    Function { func_sym: Symbol, id: usize },

    /// Unlinked from SymbolTable
    /// symbol resolvation will always failed if symbol is on unlinked state. you must link symbol to other SymbolTable.
    Unlinked,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    inner: Arc<Mutex<SymbolInner>>,
}

impl Eq for Symbol {}
impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.inner.data_ptr() == other.inner.data_ptr()
    }
}

impl Hash for Symbol {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.data_ptr().hash(state);
    }
}
