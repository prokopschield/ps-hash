mod constants;
mod digest;
mod encode;
mod error;
mod hash;
mod inner;

pub use constants::{
    DIGEST_SIZE, HASH_SIZE, HASH_SIZE_BIN, HASH_SIZE_COMPACT, MIN_RECOVERABLE, MIN_RECOVERABLE_BIN,
    PARITY, PARITY_OFFSET, PARITY_SIZE, SIZE_SIZE,
};
pub use digest::{blake3, sha256};
pub use encode::hash_encoded;
pub use error::{HashError, HashValidationError};
pub use hash::{hash, Hash, RS};
pub use inner::hash_inner;

#[cfg(test)]
mod tests {
    use super::{
        blake3, hash, hash_encoded, hash_inner, sha256, Hash, HashValidationError, DIGEST_SIZE,
        HASH_SIZE, HASH_SIZE_BIN, HASH_SIZE_COMPACT, PARITY_SIZE,
    };

    #[test]
    fn public_api_exports_work() {
        let data = b"core api";

        assert_eq!(sha256(data).len(), DIGEST_SIZE);
        assert_eq!(blake3(data).as_bytes().len(), DIGEST_SIZE);
        assert_eq!(
            hash_inner(data).expect("hash_inner should work").len(),
            HASH_SIZE_BIN
        );
        assert_eq!(
            hash_encoded(data).expect("hash_encoded should work").len(),
            HASH_SIZE
        );
    }

    #[test]
    fn moved_hash_type_exports_work() {
        let hash = hash(b"core hash").expect("hash should work");

        assert_eq!(hash.to_string().len(), HASH_SIZE);
        assert_eq!(hash.compact().len(), HASH_SIZE_COMPACT);
        assert_eq!(hash.digest().len(), DIGEST_SIZE);
        assert_eq!(hash.parity().len(), PARITY_SIZE);
    }

    #[test]
    fn moved_hash_validate_reports_invalid_length() {
        assert_eq!(
            Hash::validate("short"),
            Err(HashValidationError::InvalidLength(5))
        );
    }
}
