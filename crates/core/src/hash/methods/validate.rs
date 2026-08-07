use ps_ecc::ReedSolomon;

use crate::{
    encoding::{base64, crockford},
    HashValidationError, DIGEST_SIZE, HASH_SIZE_BASE64, HASH_SIZE_BIN, HASH_SIZE_CROCKFORD,
    MIN_RECOVERABLE_BASE64, MIN_RECOVERABLE_BIN, MIN_RECOVERABLE_CROCKFORD, PARITY_OFFSET,
};

use super::super::Hash;

impl Hash {
    /// Validates and, where necessary, repairs a hash in any of its
    /// representations.
    ///
    /// The representation is selected by input length, since the three
    /// accepted ranges are disjoint:
    ///
    /// | length  | representation            |
    /// |---------|---------------------------|
    /// | 41..=48 | binary, including compact |
    /// | 55..=64 | base64url                 |
    /// | 66..=77 | Crockford Base32          |
    ///
    /// Inputs shorter than the full size are treated as truncated and are
    /// restored by the Reed-Solomon codec, which corrects up to
    /// [`PARITY`](crate::PARITY) byte errors.
    ///
    /// # Errors
    ///
    /// - [`HashValidationError::InvalidLength`] if the length matches no
    ///   representation.
    /// - [`HashValidationError::RSDecodeError`] if the damage exceeds what the
    ///   Reed-Solomon codec can correct.
    /// - [`HashValidationError::ZeroDigest`] if the corrected digest is all
    ///   zeros. No hashed input produces the zero digest, but the all-zero
    ///   buffer is a valid Reed-Solomon codeword, so it must be rejected
    ///   explicitly.
    pub fn validate(bytes: impl AsRef<[u8]>) -> Result<Self, HashValidationError> {
        let bytes = bytes.as_ref();

        let mut hash = Self {
            // Bytes the input does not determine are filled with 0xF4. The
            // constant is chosen arbitrarily, but it must not be 0x00: the
            // all-zero buffer is a valid Reed-Solomon codeword, so relying on
            // the decoders' zero-fill would turn any input whose bytes they
            // all skip into Ok(AAA...AAA).
            inner: match bytes.len() {
                MIN_RECOVERABLE_BIN..=HASH_SIZE_BIN => {
                    let mut inner = [0xF4; HASH_SIZE_BIN];
                    inner[..bytes.len()].copy_from_slice(bytes);
                    inner
                }
                MIN_RECOVERABLE_BASE64..=HASH_SIZE_BASE64 => {
                    let mut inner = base64::decode(bytes);
                    inner[base64::decoded_len(bytes)..].fill(0xF4);
                    inner
                }
                MIN_RECOVERABLE_CROCKFORD..=HASH_SIZE_CROCKFORD => {
                    let mut inner = crockford::decode(bytes);
                    inner[crockford::decoded_len(bytes)..].fill(0xF4);
                    inner
                }
                len => Err(HashValidationError::InvalidLength(len))?,
            },
        };

        let (data, parity) = hash.inner.split_at_mut(PARITY_OFFSET);

        ReedSolomon::correct_detached_in_place(parity, data)?;

        if hash.inner[..DIGEST_SIZE] == [0; DIGEST_SIZE] {
            return Err(HashValidationError::ZeroDigest);
        }

        Ok(hash)
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use crate::{
        Hash, HashValidationError, HASH_SIZE_BASE64, HASH_SIZE_BIN, HASH_SIZE_COMPACT,
        HASH_SIZE_CROCKFORD, MIN_RECOVERABLE_BASE64, MIN_RECOVERABLE_BIN,
        MIN_RECOVERABLE_CROCKFORD,
    };

    /// Replaces the character at `index` with a different one that is valid in
    /// both alphabets, so that corruption tests stay inside the encoded
    /// character set.
    fn corrupt(bytes: &mut [u8], index: usize) {
        bytes[index] = if bytes[index] == b'A' { b'B' } else { b'A' };
    }

    #[test]
    fn validate_uncorrupted_crockford() {
        let original = Hash::hash(b"test").expect("hashing should succeed");

        assert_eq!(
            Hash::validate(original.to_crockford())
                .expect("validation of an uncorrupted hash should succeed"),
            original
        );
    }

    #[test]
    fn validate_uncorrupted_base64() {
        let original = Hash::hash(b"test").expect("hashing should succeed");

        assert_eq!(
            Hash::validate(original.to_base64())
                .expect("validation of an uncorrupted hash should succeed"),
            original
        );
    }

    #[test]
    fn validate_uncorrupted_binary() {
        let original = Hash::hash(b"test").expect("hashing should succeed");

        assert_eq!(
            Hash::validate(original.inner)
                .expect("validation of an uncorrupted binary hash should succeed"),
            original
        );
    }

    #[test]
    fn validate_uncorrupted_compact() {
        let original = Hash::hash(b"test").expect("hashing should succeed");

        assert_eq!(
            Hash::validate(original.compact())
                .expect("validation of the compact form should succeed"),
            original
        );
    }

    #[test]
    fn validate_is_case_insensitive_for_crockford() {
        let original = Hash::hash(b"case").expect("hashing should succeed");
        let lowercase = original.to_crockford().to_lowercase();

        assert_eq!(
            Hash::validate(lowercase).expect("validation of a lowercase hash should succeed"),
            original
        );
    }

    #[test]
    fn validate_recovers_corrupt_crockford_characters() {
        let original = Hash::hash(b"crockford corruption").expect("hashing should succeed");
        let mut corrupted = original.to_crockford().into_bytes();

        for index in [3, 20, 50] {
            corrupt(&mut corrupted, index);
        }

        assert_eq!(
            Hash::validate(corrupted).expect("corrupted characters should be recovered"),
            original
        );
    }

    #[test]
    fn validate_recovers_corrupt_base64_characters() {
        let original = Hash::hash(b"base64 corruption").expect("hashing should succeed");
        let mut corrupted = original.to_base64().into_bytes();

        for index in [3, 20, 50] {
            corrupt(&mut corrupted, index);
        }

        assert_eq!(
            Hash::validate(corrupted).expect("corrupted characters should be recovered"),
            original
        );
    }

    #[test]
    fn validate_recovers_corrupt_binary_bytes() {
        let original = Hash::hash(b"binary corruption").expect("hashing should succeed");
        let mut corrupted = original.inner;

        for byte in corrupted.iter_mut().take(6) {
            *byte ^= 0xFF;
        }

        assert_eq!(
            Hash::validate(corrupted).expect("corrupted bytes should be recovered"),
            original
        );
    }

    #[test]
    fn validate_rejects_unrecoverable_corruption() {
        let original = Hash::hash(b"unrecoverable").expect("hashing should succeed");
        let mut corrupted = original.to_crockford().into_bytes();

        for index in 0..30 {
            corrupt(&mut corrupted, index);
        }

        assert!(Hash::validate(corrupted).is_err());
    }

    #[test]
    fn validate_accepts_minimum_recoverable_lengths() {
        let original = Hash::hash(b"min recoverable").expect("hashing should succeed");

        let crockford = original.to_crockford();
        let base64 = original.to_base64();

        assert_eq!(
            Hash::validate(&crockford[..MIN_RECOVERABLE_CROCKFORD])
                .expect("validation of a minimum-length hash should succeed"),
            original
        );
        assert_eq!(
            Hash::validate(&base64[..MIN_RECOVERABLE_BASE64])
                .expect("validation of a minimum-length hash should succeed"),
            original
        );
        assert_eq!(
            Hash::validate(&original.inner[..MIN_RECOVERABLE_BIN])
                .expect("validation of a minimum-length hash should succeed"),
            original
        );
    }

    #[test]
    fn validate_accepts_every_length_in_every_range() {
        let original = Hash::hash(b"every length").expect("hashing should succeed");

        let crockford = original.to_crockford();
        let base64 = original.to_base64();

        for len in MIN_RECOVERABLE_CROCKFORD..=HASH_SIZE_CROCKFORD {
            assert_eq!(
                Hash::validate(&crockford[..len])
                    .expect("validation of a truncated hash should succeed"),
                original
            );
        }

        for len in MIN_RECOVERABLE_BASE64..=HASH_SIZE_BASE64 {
            assert_eq!(
                Hash::validate(&base64[..len])
                    .expect("validation of a truncated hash should succeed"),
                original
            );
        }

        for len in MIN_RECOVERABLE_BIN..=HASH_SIZE_BIN {
            assert_eq!(
                Hash::validate(&original.inner[..len])
                    .expect("validation of a truncated hash should succeed"),
                original
            );
        }
    }

    #[test]
    fn validate_rejects_lengths_between_the_ranges() {
        for len in [0, 1, 40, 49, 54, 65, 78, 100] {
            assert_eq!(
                Hash::validate(vec![b'A'; len]),
                Err(HashValidationError::InvalidLength(len))
            );
        }
    }

    #[test]
    fn validate_rejects_inputs_whose_bytes_the_decoders_all_skip() {
        for junk in [
            vec![b'!'; HASH_SIZE_CROCKFORD],
            vec![b'@'; HASH_SIZE_CROCKFORD],
            vec![b' '; HASH_SIZE_CROCKFORD],
            vec![b'-'; MIN_RECOVERABLE_CROCKFORD],
            vec![b' '; HASH_SIZE_BASE64],
            vec![b'='; HASH_SIZE_BASE64],
        ] {
            assert!(
                Hash::validate(&junk).is_err(),
                "accepted junk input: {:?}",
                String::from_utf8_lossy(&junk)
            );
        }
    }

    #[test]
    fn validate_rejects_the_zero_digest_in_every_representation() {
        for input in [
            vec![0u8; HASH_SIZE_BIN],
            vec![b'A'; HASH_SIZE_BASE64],
            vec![b'0'; HASH_SIZE_CROCKFORD],
        ] {
            assert_eq!(
                Hash::validate(&input),
                Err(HashValidationError::ZeroDigest),
                "accepted zero digest: {:?}",
                String::from_utf8_lossy(&input)
            );
        }
    }

    #[test]
    fn validate_rejects_sparse_symbols_amid_skipped_bytes() {
        let mut input = vec![b'!'; HASH_SIZE_CROCKFORD];

        input[..8].copy_from_slice(b"0123ABCD");

        assert!(Hash::validate(input).is_err());
    }

    #[test]
    fn validate_is_idempotent() {
        let original = Hash::hash(b"idempotent").expect("hashing should succeed");
        let once = Hash::validate(original.to_crockford())
            .expect("validation of an uncorrupted hash should succeed");
        let twice = Hash::validate(once.to_crockford()).expect("revalidation should succeed");

        assert_eq!(once, twice);
    }

    #[test]
    fn validate_agrees_across_representations() {
        let original = Hash::hash(b"agreement").expect("hashing should succeed");

        let from_crockford = Hash::validate(original.to_crockford())
            .expect("validation of the Crockford representation should succeed");
        let from_base64 = Hash::validate(original.to_base64())
            .expect("validation of the base64 representation should succeed");
        let from_binary = Hash::validate(original.inner)
            .expect("validation of the binary representation should succeed");
        let from_compact = Hash::validate(original.compact())
            .expect("validation of the compact representation should succeed");

        assert_eq!(from_crockford, from_base64);
        assert_eq!(from_base64, from_binary);
        assert_eq!(from_binary, from_compact);
    }

    #[test]
    fn compact_length_falls_in_the_binary_range() {
        assert!((MIN_RECOVERABLE_BIN..=HASH_SIZE_BIN).contains(&HASH_SIZE_COMPACT));
    }
}
