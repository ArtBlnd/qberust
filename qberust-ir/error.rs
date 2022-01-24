use thiserror::Error;

use crate::types::Type;

#[derive(Error, Debug)]
enum Error {
    #[error("Failed to resolve symbol! {id}")]
    SymbolNotFound { id: usize },

    #[error("Bad type casting! {before} cannot be casted into {after}")]
    BadTypeCast { before: Type, after: Type }
}
