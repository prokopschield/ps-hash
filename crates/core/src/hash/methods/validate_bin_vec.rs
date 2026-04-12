use ps_ecc::ReedSolomon;

use crate::{HashValidationError, HASH_SIZE_BIN, PARITY_OFFSET};

use super::super::Hash;

impl Hash {
    pub fn validate_bin_vec(hash: &mut Vec<u8>) -> Result<Self, HashValidationError> {
        hash.resize(HASH_SIZE_BIN, 0xF4);

        let (data, parity) = hash.split_at_mut(PARITY_OFFSET);
        ReedSolomon::correct_detached_in_place(parity, data)?;

        let mut inner = [0u8; HASH_SIZE_BIN];
        inner.copy_from_slice(hash);

        Ok(Self { inner })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Hash, HASH_SIZE_BIN};

    #[test]
    fn validate_bin_vec_uncorrupted() {
        let original = Hash::hash(b"bin vec").unwrap();
        let mut vec = original.inner.to_vec();
        let validated = Hash::validate_bin_vec(&mut vec).unwrap();
        assert_eq!(original, validated);
    }

    #[test]
    fn validate_bin_vec_single_byte_corruption() {
        let original = Hash::hash(b"corruption").unwrap();
        let mut vec = original.inner.to_vec();
        vec[5] ^= 0xFF;
        let recovered = Hash::validate_bin_vec(&mut vec).unwrap();
        assert_eq!(original, recovered);
    }

    #[test]
    fn validate_bin_vec_multi_byte_corruption() {
        let original = Hash::hash(b"multi byte").unwrap();
        let mut vec = original.inner.to_vec();
        vec[0] ^= 0xFF;
        vec[1] ^= 0xFF;
        vec[2] ^= 0xFF;
        let recovered = Hash::validate_bin_vec(&mut vec).unwrap();
        assert_eq!(original, recovered);
    }

    #[test]
    fn validate_bin_vec_unrecoverable() {
        let original = Hash::hash(b"unrecoverable").unwrap();
        let mut vec = original.inner.to_vec();
        for byte in vec.iter_mut().take(20) {
            *byte ^= 0xFF;
        }
        let result = Hash::validate_bin_vec(&mut vec);
        assert!(result.is_err());
    }

    #[test]
    fn validate_bin_vec_resizes_short_input() {
        let original = Hash::hash(b"short input").unwrap();
        let mut vec = original.compact().to_vec();
        let original_len = vec.len();
        assert!(original_len < HASH_SIZE_BIN);
        let _ = Hash::validate_bin_vec(&mut vec);
        assert_eq!(vec.len(), HASH_SIZE_BIN);
    }

    #[test]
    fn validate_bin_vec_from_compact() {
        let original = Hash::hash(b"compact").unwrap();
        let mut vec = original.compact().to_vec();
        let recovered = Hash::validate_bin_vec(&mut vec).unwrap();
        assert_eq!(original, recovered);
    }

    #[test]
    fn validate_bin_vec_mutates_input() {
        let original = Hash::hash(b"mutates").unwrap();
        let mut vec = original.compact().to_vec();
        let before_len = vec.len();
        let _ = Hash::validate_bin_vec(&mut vec);
        assert_ne!(before_len, vec.len());
    }

    #[test]
    fn validate_bin_vec_corrects_input_in_place() {
        let original = Hash::hash(b"in place").unwrap();
        let mut vec = original.inner.to_vec();
        vec[0] ^= 0x01;
        let corrupted_byte = vec[0];
        let _ = Hash::validate_bin_vec(&mut vec);
        assert_ne!(vec[0], corrupted_byte);
        assert_eq!(vec[0], original.inner[0]);
    }

    #[test]
    fn validate_bin_vec_empty_input() {
        let mut vec = Vec::new();
        let result = Hash::validate_bin_vec(&mut vec);
        assert!(result.is_err());
    }

    #[test]
    fn validate_bin_vec_idempotent() {
        let original = Hash::hash(b"idempotent").unwrap();
        let mut vec = original.inner.to_vec();
        let v1 = Hash::validate_bin_vec(&mut vec).unwrap();
        let mut vec2 = v1.inner.to_vec();
        let v2 = Hash::validate_bin_vec(&mut vec2).unwrap();
        assert_eq!(original, v1);
        assert_eq!(v1, v2);
    }
}
