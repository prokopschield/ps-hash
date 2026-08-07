mod constants;
mod digest;
mod encode;
mod error;
mod hash;
mod inner;

pub mod encoding;

#[cfg(test)]
#[allow(clippy::expect_used)]
mod golden;

pub use constants::{
    DIGEST_SIZE, HASH_SIZE_BASE64, HASH_SIZE_BIN, HASH_SIZE_COMPACT, HASH_SIZE_CROCKFORD,
    MIN_RECOVERABLE_BASE64, MIN_RECOVERABLE_BIN, MIN_RECOVERABLE_CROCKFORD, PARITY, PARITY_OFFSET,
    PARITY_SIZE, SIZE_SIZE,
};
pub use digest::{blake3, sha256};
pub use encode::hash_encoded;
pub use error::{HashError, HashValidationError};
pub use hash::{hash, Hash, RS};
pub use inner::{hash_inner, inner_from_parts};
pub use ps_pint16::PackedInt;

#[allow(clippy::expect_used)]
#[cfg(test)]
mod tests {
    use super::{
        blake3, encoding, hash, hash_encoded, hash_inner, sha256, Hash, HashValidationError,
        DIGEST_SIZE, HASH_SIZE_BASE64, HASH_SIZE_BIN, HASH_SIZE_COMPACT, HASH_SIZE_CROCKFORD,
        PARITY_SIZE,
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
            HASH_SIZE_CROCKFORD
        );
    }

    #[test]
    fn hash_method_exports_work() {
        let hash = hash(b"core hash").expect("hash should work");

        assert_eq!(hash.to_string().len(), HASH_SIZE_CROCKFORD);
        assert_eq!(hash.to_crockford().len(), HASH_SIZE_CROCKFORD);
        assert_eq!(hash.to_base64().len(), HASH_SIZE_BASE64);
        assert_eq!(hash.compact().len(), HASH_SIZE_COMPACT);
        assert_eq!(hash.digest().len(), DIGEST_SIZE);
        assert_eq!(hash.parity().len(), PARITY_SIZE);
    }

    #[test]
    fn encoding_module_is_public() {
        let inner = hash_inner(b"public encoding").expect("hash_inner should work");

        let crockford = encoding::crockford::encode(&inner);
        let base64 = encoding::base64::encode(&inner);

        assert_eq!(encoding::crockford::decode(&crockford), inner);
        assert_eq!(encoding::base64::decode(&base64), inner);
    }

    #[test]
    fn validate_reports_invalid_length() {
        assert_eq!(
            Hash::validate("short"),
            Err(HashValidationError::InvalidLength(5))
        );
    }
}
