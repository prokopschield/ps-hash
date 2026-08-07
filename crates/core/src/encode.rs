use ps_ecc::RSGenerateParityError;

use crate::{encoding::crockford, hash_inner, HASH_SIZE_CROCKFORD};

/// Hashes `data` and encodes it in the canonical Crockford Base32 form.
///
/// For the base64url form, use [`Hash::to_base64`](crate::Hash::to_base64).
pub fn hash_encoded(data: &[u8]) -> Result<[u8; HASH_SIZE_CROCKFORD], RSGenerateParityError> {
    Ok(crockford::encode(&hash_inner(data)?))
}

#[allow(clippy::expect_used)]
#[cfg(test)]
mod tests {
    use super::{crockford, hash_encoded, hash_inner, HASH_SIZE_CROCKFORD};

    #[allow(clippy::expect_used)]
    #[test]
    fn hash_encoded_matches_encoded_inner() {
        let data = b"encode consistency";

        let encoded = hash_encoded(data).expect("hash_encoded should work");
        let inner = hash_inner(data).expect("hash_inner should work");

        assert_eq!(encoded, crockford::encode(&inner));
        assert_eq!(encoded.len(), HASH_SIZE_CROCKFORD);
    }

    #[test]
    fn hash_encoded_is_deterministic() {
        let data = b"determinism";

        assert_eq!(
            hash_encoded(data).expect("hash_encoded should work"),
            hash_encoded(data).expect("hash_encoded should work")
        );
    }
}
