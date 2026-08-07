use ps_ecc::ReedSolomon;

use crate::{HashValidationError, DIGEST_SIZE, HASH_SIZE_BIN, PARITY_OFFSET};

use super::super::Hash;

impl Hash {
    pub fn validate_bin_vec(hash: &mut Vec<u8>) -> Result<Self, HashValidationError> {
        if hash.len() > HASH_SIZE_BIN {
            return Err(HashValidationError::InvalidLength(hash.len()));
        }

        hash.resize(HASH_SIZE_BIN, 0xF4);

        let (data, parity) = hash.split_at_mut(PARITY_OFFSET);
        ReedSolomon::correct_detached_in_place(parity, data)?;

        if data[..DIGEST_SIZE] == [0; DIGEST_SIZE] {
            return Err(HashValidationError::ZeroDigest);
        }

        let mut inner = [0u8; HASH_SIZE_BIN];
        inner.copy_from_slice(hash);

        Ok(Self { inner })
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use crate::{Hash, HashValidationError, HASH_SIZE_BIN, PARITY};

    #[test]
    fn validate_bin_vec_uncorrupted() {
        let original = Hash::hash(b"bin vec").expect("hashing should succeed");
        let mut vec = original.inner.to_vec();
        let validated = Hash::validate_bin_vec(&mut vec)
            .expect("validation of an uncorrupted binary vector should succeed");
        assert_eq!(original, validated);
    }

    #[test]
    fn validate_bin_vec_single_byte_corruption() {
        let original = Hash::hash(b"corruption").expect("hashing should succeed");
        let mut vec = original.inner.to_vec();
        vec[5] ^= 0xFF;
        let recovered =
            Hash::validate_bin_vec(&mut vec).expect("single-byte corruption should be recovered");
        assert_eq!(original, recovered);
    }

    #[test]
    fn validate_bin_vec_two_byte_corruption() {
        let original = Hash::hash(b"two bytes").expect("hashing should succeed");
        let mut vec = original.inner.to_vec();
        vec[0] ^= 0xFF;
        vec[1] ^= 0xFF;
        let recovered =
            Hash::validate_bin_vec(&mut vec).expect("two-byte corruption should be recovered");
        assert_eq!(original, recovered);
    }

    #[test]
    fn validate_bin_vec_corrects_at_most_the_parity_budget() {
        let original = Hash::hash(b"budget").expect("hashing should succeed");

        let mut recoverable = original.inner.to_vec();

        for byte in recoverable.iter_mut().take(PARITY as usize) {
            *byte ^= 0xFF;
        }

        assert_eq!(
            Hash::validate_bin_vec(&mut recoverable)
                .expect("corruption within the parity budget should be recovered"),
            original
        );

        let mut unrecoverable = original.inner.to_vec();

        for byte in unrecoverable.iter_mut().take(PARITY as usize + 1) {
            *byte ^= 0xFF;
        }

        assert!(Hash::validate_bin_vec(&mut unrecoverable).is_err());
    }

    #[test]
    fn validate_bin_vec_unrecoverable() {
        let original = Hash::hash(b"unrecoverable").expect("hashing should succeed");
        let mut vec = original.inner.to_vec();
        for byte in vec.iter_mut().take(20) {
            *byte ^= 0xFF;
        }
        let result = Hash::validate_bin_vec(&mut vec);
        assert!(result.is_err());
    }

    #[test]
    fn validate_bin_vec_resizes_short_input() {
        let original = Hash::hash(b"short input").expect("hashing should succeed");
        let mut vec = original.compact().to_vec();
        let original_len = vec.len();
        assert!(original_len < HASH_SIZE_BIN);
        let _ = Hash::validate_bin_vec(&mut vec);
        assert_eq!(vec.len(), HASH_SIZE_BIN);
    }

    #[test]
    fn validate_bin_vec_from_compact() {
        let original = Hash::hash(b"compact").expect("hashing should succeed");
        let mut vec = original.compact().to_vec();
        let recovered = Hash::validate_bin_vec(&mut vec)
            .expect("recovery from the compact form should succeed");
        assert_eq!(original, recovered);
    }

    #[test]
    fn validate_bin_vec_mutates_input() {
        let original = Hash::hash(b"mutates").expect("hashing should succeed");
        let mut vec = original.compact().to_vec();
        let before_len = vec.len();
        let _ = Hash::validate_bin_vec(&mut vec);
        assert_ne!(before_len, vec.len());
    }

    #[test]
    fn validate_bin_vec_corrects_input_in_place() {
        let original = Hash::hash(b"in place").expect("hashing should succeed");
        let mut vec = original.inner.to_vec();
        vec[0] ^= 0x01;
        let corrupted_byte = vec[0];
        let _ = Hash::validate_bin_vec(&mut vec);
        assert_ne!(vec[0], corrupted_byte);
        assert_eq!(vec[0], original.inner[0]);
    }

    #[test]
    fn validate_bin_vec_rejects_the_zero_digest() {
        let mut vec = vec![0u8; HASH_SIZE_BIN];

        assert_eq!(
            Hash::validate_bin_vec(&mut vec),
            Err(HashValidationError::ZeroDigest)
        );
    }

    #[test]
    fn validate_bin_vec_rejects_oversized_input() {
        let original = Hash::hash(b"oversized").expect("hashing should succeed");
        let mut vec = original.inner.to_vec();
        vec.push(0xF4);

        assert_eq!(
            Hash::validate_bin_vec(&mut vec),
            Err(HashValidationError::InvalidLength(HASH_SIZE_BIN + 1))
        );
        assert_eq!(vec.len(), HASH_SIZE_BIN + 1);
    }

    #[test]
    fn validate_bin_vec_empty_input() {
        let mut vec = Vec::new();
        let result = Hash::validate_bin_vec(&mut vec);
        assert!(result.is_err());
    }

    #[test]
    fn validate_bin_vec_idempotent() {
        let original = Hash::hash(b"idempotent").expect("hashing should succeed");
        let mut vec = original.inner.to_vec();
        let v1 = Hash::validate_bin_vec(&mut vec).expect("validation should succeed");
        let mut vec2 = v1.inner.to_vec();
        let v2 = Hash::validate_bin_vec(&mut vec2).expect("revalidation should succeed");
        assert_eq!(original, v1);
        assert_eq!(v1, v2);
    }
}
