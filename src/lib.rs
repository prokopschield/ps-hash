#![allow(clippy::missing_errors_doc)]
pub mod error;
pub use error::*;
pub use ps_hash_core::{
    encoding, hash, Hash, PackedInt, DIGEST_SIZE, HASH_SIZE_BASE64, HASH_SIZE_BIN,
    HASH_SIZE_COMPACT, HASH_SIZE_CROCKFORD, MIN_RECOVERABLE_BASE64, MIN_RECOVERABLE_BIN,
    MIN_RECOVERABLE_CROCKFORD, PARITY, PARITY_OFFSET, PARITY_SIZE, RS, SIZE_SIZE,
};

#[cfg(test)]
pub mod tests;
