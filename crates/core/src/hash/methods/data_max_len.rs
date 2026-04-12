use ps_pint16::PackedInt;

use crate::DIGEST_SIZE;

use super::super::Hash;

impl Hash {
    #[must_use]
    pub fn data_max_len(&self) -> PackedInt {
        PackedInt::from_16_bits(&[self.inner[DIGEST_SIZE], self.inner[DIGEST_SIZE + 1]])
    }
}

#[cfg(test)]
mod tests {
    use crate::Hash;

    #[test]
    fn data_max_len_empty_data() {
        let h = Hash::hash(b"").unwrap();
        assert_eq!(h.data_max_len().to_usize(), 0);
    }

    #[test]
    fn data_max_len_small_data() {
        let data = b"small";
        let h = Hash::hash(data).unwrap();
        assert_eq!(h.data_max_len().to_usize(), data.len());
    }

    #[test]
    fn data_max_len_exact_255() {
        let data = vec![0u8; 255];
        let h = Hash::hash(&data).unwrap();
        assert_eq!(h.data_max_len().to_usize(), 255);
    }

    #[test]
    fn data_max_len_exact_256() {
        let data = vec![0u8; 256];
        let h = Hash::hash(&data).unwrap();
        assert_eq!(h.data_max_len().to_usize(), 256);
    }

    #[test]
    fn data_max_len_exact_65536() {
        let data = vec![0u8; 65536];
        let h = Hash::hash(&data).unwrap();
        assert_eq!(h.data_max_len().to_usize(), 65536);
    }

    #[test]
    fn data_max_len_large_data() {
        let data = vec![0u8; 1_000_000];
        let h = Hash::hash(&data).unwrap();
        assert!(h.data_max_len().to_usize() >= 1_000_000);
    }

    #[test]
    fn data_max_len_boundary_values() {
        for len in [0, 1, 127, 128, 255, 256, 65536, 0x2f000] {
            let data = vec![0u8; len];
            let h = Hash::hash(&data).unwrap();
            assert_eq!(h.data_max_len().to_usize(), len);
        }
    }

    #[test]
    fn data_max_len_is_upper_bound() {
        for len in [100, 1000, 10000, 100000] {
            let data = vec![0u8; len];
            let h = Hash::hash(&data).unwrap();
            assert!(h.data_max_len().to_usize() >= len);
        }
    }

    #[test]
    fn data_max_len_deterministic() {
        let data = b"deterministic";
        let h1 = Hash::hash(data).unwrap();
        let h2 = Hash::hash(data).unwrap();
        assert_eq!(h1.data_max_len(), h2.data_max_len());
    }

    #[test]
    fn data_max_len_preserved_after_validation() {
        let data = b"preserved";
        let original = Hash::hash(data).unwrap();
        let validated = Hash::validate(original.to_string()).unwrap();
        assert_eq!(original.data_max_len(), validated.data_max_len());
    }

    #[test]
    fn data_max_len_preserved_after_compact_recovery() {
        let data = b"compact recovery";
        let original = Hash::hash(data).unwrap();
        let mut vec = original.compact().to_vec();
        let recovered = Hash::validate_bin_vec(&mut vec).unwrap();
        assert_eq!(original.data_max_len(), recovered.data_max_len());
    }

    #[test]
    fn data_max_len_returns_packed_int() {
        let h = Hash::hash(b"packed int").unwrap();
        let len = h.data_max_len();
        assert_eq!(len.to_usize(), 10);
    }
}
