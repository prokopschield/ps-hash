use ps_pint16::PackedInt;

use crate::{inner_from_parts, HashError, DIGEST_SIZE};

use super::super::Hash;

impl Hash {
    /// Reconstructs a hash from its stored parts: the digest and the packed
    /// data length.
    ///
    /// The parity block is a deterministic function of the parts, so storage
    /// that holds many hashes may keep only the digest and length field and
    /// rebuild the full hash on demand.
    ///
    /// # Errors
    ///
    /// - [`HashError::ZeroDigest`] if `digest` is all zeros. No hashed input
    ///   produces the zero digest, and [`Hash::validate`] rejects it, so a
    ///   hash built from it could never be validated.
    /// - [`HashError::RSGenerateParityError`] if parity generation fails.
    pub fn from_parts(digest: &[u8; DIGEST_SIZE], data_len: PackedInt) -> Result<Self, HashError> {
        if digest == &[0; DIGEST_SIZE] {
            return Err(HashError::ZeroDigest);
        }

        let inner = inner_from_parts(digest, data_len)?;

        Ok(Self { inner })
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use ps_pint16::PackedInt;

    use crate::{Hash, HashError, DIGEST_SIZE, PARITY_OFFSET};

    #[test]
    fn from_parts_reconstructs_a_hashed_value() {
        let original = Hash::hash(b"reconstruct me").expect("hashing should succeed");
        let rebuilt = Hash::from_parts(original.digest(), original.data_max_len())
            .expect("reconstruction from parts should succeed");

        assert_eq!(original, rebuilt);
        assert_eq!(original.inner, rebuilt.inner);
    }

    #[test]
    fn from_parts_regenerates_the_parity_block() {
        let original = Hash::hash(b"parity regeneration").expect("hashing should succeed");
        let rebuilt = Hash::from_parts(original.digest(), original.data_max_len())
            .expect("reconstruction from parts should succeed");

        assert_eq!(original.parity(), rebuilt.parity());
    }

    #[test]
    fn from_parts_round_trips_through_stored_bytes() {
        let original = Hash::hash(b"stored parts").expect("hashing should succeed");

        let stored = &original.inner[..PARITY_OFFSET];

        let digest = stored[..32]
            .try_into()
            .expect("digest slice should have the correct length");
        let data_len = PackedInt::from_16_bits(&[stored[32], stored[33]]);
        let rebuilt =
            Hash::from_parts(&digest, data_len).expect("reconstruction from parts should succeed");

        assert_eq!(original, rebuilt);
    }

    #[test]
    fn from_parts_differs_for_different_lengths() {
        let original = Hash::hash(b"length matters").expect("hashing should succeed");

        let same = Hash::from_parts(original.digest(), original.data_max_len())
            .expect("reconstruction from parts should succeed");
        let different = Hash::from_parts(original.digest(), PackedInt::from_usize(12345))
            .expect("reconstruction from parts should succeed");

        assert_eq!(original, same);
        assert_ne!(original.inner, different.inner);
    }

    #[test]
    fn from_parts_rejects_the_zero_digest() {
        assert_eq!(
            Hash::from_parts(&[0; DIGEST_SIZE], PackedInt::from_usize(0)),
            Err(HashError::ZeroDigest)
        );
    }

    #[test]
    fn from_parts_validates_cleanly() {
        let original = Hash::hash(b"valid parts").expect("hashing should succeed");
        let rebuilt = Hash::from_parts(original.digest(), original.data_max_len())
            .expect("reconstruction from parts should succeed");

        assert_eq!(
            Hash::validate(rebuilt.inner).expect("validation of a rebuilt hash should succeed"),
            original
        );
    }
}
