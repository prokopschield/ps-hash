use crate::HashValidationError;

use super::super::Hash;

impl TryFrom<&[u8]> for Hash {
    type Error = HashValidationError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::validate(value)
    }
}

impl TryFrom<&str> for Hash {
    type Error = HashValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.as_bytes().try_into()
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use crate::{Hash, HashValidationError};

    #[test]
    fn try_from_slice_valid() {
        let original = Hash::hash(b"slice").expect("hashing should succeed");
        let bytes = original.to_string().into_bytes();
        let recovered =
            Hash::try_from(bytes.as_slice()).expect("conversion from a valid slice should succeed");
        assert_eq!(original, recovered);
    }

    #[test]
    fn try_from_slice_invalid_short() {
        let result = Hash::try_from(b"short".as_slice());
        assert!(result.is_err());
    }

    #[test]
    fn try_from_slice_invalid_length_error() {
        let result = Hash::try_from(b"short".as_slice());
        assert_eq!(result, Err(HashValidationError::InvalidLength(5)));
    }

    #[test]
    fn try_from_slice_empty() {
        let result = Hash::try_from([].as_slice());
        assert_eq!(result, Err(HashValidationError::InvalidLength(0)));
    }

    #[test]
    fn try_from_slice_corrects_corruption() {
        let original = Hash::hash(b"corruption").expect("hashing should succeed");
        let mut bytes = original.to_string().into_bytes();
        bytes[5] ^= 0x01;
        let recovered = Hash::try_from(bytes.as_slice())
            .expect("conversion should recover single-bit corruption");
        assert_eq!(original, recovered);
    }

    #[test]
    fn try_from_str_valid() {
        let original = Hash::hash(b"str").expect("hashing should succeed");
        let s = original.to_string();
        let recovered =
            Hash::try_from(s.as_str()).expect("conversion from a valid string should succeed");
        assert_eq!(original, recovered);
    }

    #[test]
    fn try_from_str_invalid_short() {
        let result = Hash::try_from("short");
        assert!(result.is_err());
    }

    #[test]
    fn try_from_str_invalid_length_error() {
        let result = Hash::try_from("short");
        assert_eq!(result, Err(HashValidationError::InvalidLength(5)));
    }

    #[test]
    fn try_from_str_empty() {
        let result = Hash::try_from("");
        assert_eq!(result, Err(HashValidationError::InvalidLength(0)));
    }

    #[test]
    fn try_from_str_corrects_corruption() {
        let original = Hash::hash(b"str corruption").expect("hashing should succeed");
        let mut bytes = original.to_string().into_bytes();
        bytes[5] ^= 0x01;
        let corrupted = String::from_utf8(bytes).expect("corrupted bytes should be valid UTF-8");
        let recovered = Hash::try_from(corrupted.as_str())
            .expect("conversion should recover single-bit corruption");
        assert_eq!(original, recovered);
    }

    #[test]
    fn try_from_error_type_is_hash_validation_error() {
        fn assert_error_type<T: TryFrom<&'static str, Error = HashValidationError>>() {}
        assert_error_type::<Hash>();
    }

    #[test]
    fn try_from_str_delegates_to_slice() {
        let original = Hash::hash(b"delegate").expect("hashing should succeed");
        let s = original.to_string();
        let from_str = Hash::try_from(s.as_str()).expect("conversion from &str should succeed");
        let from_slice =
            Hash::try_from(s.as_bytes()).expect("conversion from &[u8] should succeed");
        assert_eq!(from_str, from_slice);
    }

    #[test]
    fn try_from_binary_data() {
        let original = Hash::hash(b"binary").expect("hashing should succeed");
        let recovered = Hash::try_from(original.inner.as_slice())
            .expect("conversion from raw binary should succeed");
        assert_eq!(original, recovered);
    }

    /// `TryFrom<&[u8]>` delegates to `validate()`, which accepts both base64-encoded
    /// and raw binary representations. This test documents that the raw binary
    /// inner representation is a valid input, distinct from base64-encoded bytes.
    #[test]
    fn try_from_raw_binary_differs_from_base64_encoded() {
        let original = Hash::hash(b"encoding test").expect("hashing should succeed");

        let base64_bytes = original.to_string().into_bytes();
        let raw_binary = original.inner.as_slice();

        assert_ne!(
            base64_bytes.as_slice(),
            raw_binary,
            "base64-encoded and raw binary representations must differ"
        );

        let from_base64 = Hash::try_from(base64_bytes.as_slice())
            .expect("conversion from base64 bytes should succeed");
        let from_raw =
            Hash::try_from(raw_binary).expect("conversion from raw binary should succeed");

        assert_eq!(from_base64, original);
        assert_eq!(from_raw, original);
    }
}
