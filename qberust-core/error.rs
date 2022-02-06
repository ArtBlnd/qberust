use thiserror::Error;

use crate::ir::types::Type;

#[derive(Error, Debug)]
enum Error {
    #[error("Failed to resolve symbol! \"{id}\"")]
    SymbolNotFound { id: usize },

    #[error("Symbol has conflict! there is two symbol exists with same name. \"{name}\"")]
    SymbolNameConflict { name: &'static str },

    #[error("Bad symbol match! symbol should be \"{a}\". not \"{b}\"")]
    SymbolMatchFailed { a: &'static str, b: &'static str },

    #[error("Bad type casting! \"{before}\" cannot be casted into \"{after}\"")]
    BadTypeCast { before: Type, after: Type },
}
