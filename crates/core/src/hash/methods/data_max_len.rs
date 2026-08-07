use ps_pint16::PackedInt;

use crate::DIGEST_SIZE;

use super::super::Hash;

impl Hash {
    /// Returns the length of the hashed data.
    ///
    /// The length is stored as a [`PackedInt`], which rounds up, so this is an
    /// upper bound for data longer than [`PackedInt`] can represent exactly.
    #[must_use]
    pub const fn data_max_len(&self) -> PackedInt {
        PackedInt::from_16_bits(&[self.inner[DIGEST_SIZE], self.inner[DIGEST_SIZE + 1]])
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use crate::Hash;

    #[test]
    fn data_max_len_empty_data() {
        let h = Hash::hash(b"").expect("hashing should succeed");
        assert_eq!(h.data_max_len().to_usize(), 0);
    }

    #[test]
    fn data_max_len_small_data() {
        let data = b"small";
        let h = Hash::hash(data).expect("hashing should succeed");
        assert_eq!(h.data_max_len().to_usize(), data.len());
    }

    #[test]
    fn data_max_len_is_exact_at_boundaries() {
        for len in [0, 1, 127, 128, 255, 256, 511, 512, 65536, 0x2f000] {
            let data = vec![0u8; len];
            let h = Hash::hash(&data).expect("hashing should succeed");

            assert_eq!(h.data_max_len().to_usize(), len, "data_max_len({len})");
        }
    }

    #[test]
    fn data_max_len_is_exact_below_512() {
        for len in 0..512 {
            let data = vec![0u8; len];
            let h = Hash::hash(&data).expect("hashing should succeed");

            assert_eq!(h.data_max_len().to_usize(), len, "data_max_len({len})");
        }
    }

    #[test]
    fn data_max_len_large_data() {
        let data = vec![0u8; 1_000_000];
        let h = Hash::hash(&data).expect("hashing should succeed");
        assert!(h.data_max_len().to_usize() >= 1_000_000);
    }

    #[test]
    fn data_max_len_is_upper_bound() {
        for len in [100, 1000, 10000, 100_000] {
            let data = vec![0u8; len];
            let h = Hash::hash(&data).expect("hashing should succeed");
            assert!(h.data_max_len().to_usize() >= len);
        }
    }

    #[test]
    fn data_max_len_deterministic() {
        let data = b"deterministic";
        let h1 = Hash::hash(data).expect("hashing should succeed");
        let h2 = Hash::hash(data).expect("hashing should succeed");
        assert_eq!(h1.data_max_len(), h2.data_max_len());
    }

    #[test]
    fn data_max_len_survives_every_representation() {
        let data = b"preserved";
        let original = Hash::hash(data).expect("hashing should succeed");

        for encoded in [original.to_crockford(), original.to_base64()] {
            let validated =
                Hash::validate(encoded).expect("validation of an uncorrupted hash should succeed");

            assert_eq!(original.data_max_len(), validated.data_max_len());
        }
    }

    #[test]
    fn data_max_len_preserved_after_compact_recovery() {
        let data = b"compact recovery";
        let original = Hash::hash(data).expect("hashing should succeed");
        let mut vec = original.compact().to_vec();
        let recovered = Hash::validate_bin_vec(&mut vec)
            .expect("recovery from the compact form should succeed");
        assert_eq!(original.data_max_len(), recovered.data_max_len());
    }

    #[test]
    fn data_max_len_returns_packed_int() {
        let h = Hash::hash(b"packed int").expect("hashing should succeed");
        let len = h.data_max_len();
        assert_eq!(len.to_usize(), 10);
    }
}
