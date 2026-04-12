use ps_base64::sized_decode;
use ps_ecc::ReedSolomon;

use crate::{
    HashValidationError, HASH_SIZE, HASH_SIZE_BIN, MIN_RECOVERABLE, MIN_RECOVERABLE_BIN,
    PARITY_OFFSET,
};

use super::super::Hash;

impl Hash {
    pub fn validate(bytes: impl AsRef<[u8]>) -> Result<Self, HashValidationError> {
        let bytes = bytes.as_ref();

        let mut hash = Self {
            inner: match bytes.len() {
                MIN_RECOVERABLE_BIN..=HASH_SIZE_BIN => {
                    let mut inner = [0xF4; HASH_SIZE_BIN];
                    inner[..bytes.len()].copy_from_slice(bytes);
                    inner
                }
                MIN_RECOVERABLE..=HASH_SIZE => sized_decode::<HASH_SIZE_BIN>(bytes),
                len => Err(HashValidationError::InvalidLength(len))?,
            },
        };

        let (data, parity) = hash.inner.split_at_mut(PARITY_OFFSET);
        ReedSolomon::correct_detached_in_place(parity, data)?;

        Ok(hash)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Hash, HashValidationError, HASH_SIZE, HASH_SIZE_BIN, MIN_RECOVERABLE, MIN_RECOVERABLE_BIN,
    };

    #[test]
    fn validate_uncorrupted_base64() {
        let original = Hash::hash(b"test").unwrap();
        let validated = Hash::validate(original.to_string()).unwrap();
        assert_eq!(original, validated);
    }

    #[test]
    fn validate_uncorrupted_binary() {
        let original = Hash::hash(b"test").unwrap();
        let validated = Hash::validate(&original.inner).unwrap();
        assert_eq!(original, validated);
    }

    #[test]
    fn validate_single_bit_corruption_base64() {
        let original = Hash::hash(b"corruption").unwrap();
        let mut corrupted = original.to_string().into_bytes();
        corrupted[5] ^= 0x01;
        let corrupted_str = String::from_utf8(corrupted).unwrap();
        let recovered = Hash::validate(&corrupted_str).unwrap();
        assert_eq!(original, recovered);
    }

    #[test]
    fn validate_single_byte_corruption_binary() {
        let original = Hash::hash(b"binary corruption").unwrap();
        let mut corrupted = original.inner;
        corrupted[10] ^= 0xFF;
        let recovered = Hash::validate(&corrupted).unwrap();
        assert_eq!(original, recovered);
    }

    #[test]
    fn validate_multi_byte_corruption_recoverable() {
        let original = Hash::hash(b"multi").unwrap();
        let mut corrupted = original.inner;
        corrupted[0] ^= 0xFF;
        corrupted[1] ^= 0xFF;
        corrupted[2] ^= 0xFF;
        let recovered = Hash::validate(&corrupted).unwrap();
        assert_eq!(original, recovered);
    }

    #[test]
    fn validate_unrecoverable_corruption() {
        let original = Hash::hash(b"unrecoverable").unwrap();
        let mut corrupted = original.to_string().into_bytes();
        for byte in corrupted.iter_mut().take(20) {
            *byte ^= 0xFF;
        }
        let corrupted_str = String::from_utf8_lossy(&corrupted);
        let result = Hash::validate(corrupted_str.as_ref());
        assert!(result.is_err());
    }

    #[test]
    fn validate_too_short_returns_invalid_length() {
        let result = Hash::validate("short");
        assert_eq!(result, Err(HashValidationError::InvalidLength(5)));
    }

    #[test]
    fn validate_empty_returns_invalid_length() {
        let result = Hash::validate("");
        assert_eq!(result, Err(HashValidationError::InvalidLength(0)));
    }

    #[test]
    fn validate_min_recoverable_length_base64() {
        let original = Hash::hash(b"min recoverable").unwrap();
        let truncated = &original.to_string()[..MIN_RECOVERABLE];
        let result = Hash::validate(truncated);
        assert!(result.is_ok());
    }

    #[test]
    fn validate_below_min_recoverable_fails() {
        let original = Hash::hash(b"below min").unwrap();
        let truncated = &original.to_string()[..MIN_RECOVERABLE - 1];
        let result = Hash::validate(truncated);
        assert!(result.is_err());
    }

    #[test]
    fn validate_min_recoverable_length_binary() {
        let original = Hash::hash(b"min binary").unwrap();
        let truncated = &original.inner[..MIN_RECOVERABLE_BIN];
        let result = Hash::validate(truncated);
        assert!(result.is_ok());
    }

    #[test]
    fn validate_full_length_base64() {
        let original = Hash::hash(b"full").unwrap();
        let s = original.to_string();
        assert_eq!(s.len(), HASH_SIZE);
        let validated = Hash::validate(&s).unwrap();
        assert_eq!(original, validated);
    }

    #[test]
    fn validate_full_length_binary() {
        let original = Hash::hash(b"full binary").unwrap();
        assert_eq!(original.inner.len(), HASH_SIZE_BIN);
        let validated = Hash::validate(&original.inner).unwrap();
        assert_eq!(original, validated);
    }

    #[test]
    fn validate_accepts_str() {
        let original = Hash::hash(b"str").unwrap();
        let s = original.to_string();
        assert!(Hash::validate(s.as_str()).is_ok());
    }

    #[test]
    fn validate_accepts_string() {
        let original = Hash::hash(b"string").unwrap();
        let s = original.to_string();
        assert!(Hash::validate(&s).is_ok());
    }

    #[test]
    fn validate_accepts_slice() {
        let original = Hash::hash(b"slice").unwrap();
        let bytes = original.to_string().into_bytes();
        assert!(Hash::validate(bytes.as_slice()).is_ok());
    }

    #[test]
    fn validate_accepts_vec() {
        let original = Hash::hash(b"vec").unwrap();
        let bytes = original.to_string().into_bytes();
        assert!(Hash::validate(&bytes).is_ok());
    }

    #[test]
    fn validate_idempotent() {
        let original = Hash::hash(b"idempotent").unwrap();
        let v1 = Hash::validate(original.to_string()).unwrap();
        let v2 = Hash::validate(v1.to_string()).unwrap();
        let v3 = Hash::validate(v2.to_string()).unwrap();
        assert_eq!(original, v1);
        assert_eq!(v1, v2);
        assert_eq!(v2, v3);
    }
}
