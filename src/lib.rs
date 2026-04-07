#![allow(clippy::missing_errors_doc)]
pub mod error;
pub use error::*;
pub use ps_hash_core::{
    hash, Hash, DIGEST_SIZE, HASH_SIZE, HASH_SIZE_BIN, HASH_SIZE_COMPACT, MIN_RECOVERABLE,
    MIN_RECOVERABLE_BIN, PARITY, PARITY_OFFSET, PARITY_SIZE, RS, SIZE_SIZE,
};

#[cfg(test)]
pub mod tests;
