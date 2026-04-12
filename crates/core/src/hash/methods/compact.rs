use crate::HASH_SIZE_COMPACT;

use super::super::Hash;

impl Hash {
    #[inline]
    #[must_use]
    pub fn compact(&self) -> &[u8] {
        &self.inner[..HASH_SIZE_COMPACT]
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use crate::{Hash, HASH_SIZE_COMPACT};

    #[test]
    fn compact_returns_correct_size() {
        let h = Hash::hash(b"test").expect("hashing should succeed");
        assert_eq!(h.compact().len(), HASH_SIZE_COMPACT);
    }

    #[test]
    fn compact_is_prefix_of_inner() {
        let h = Hash::hash(b"prefix").expect("hashing should succeed");
        let compact = h.compact();
        assert_eq!(compact, &h.inner[..HASH_SIZE_COMPACT]);
    }

    #[test]
    fn compact_is_deterministic() {
        let h = Hash::hash(b"deterministic").expect("hashing should succeed");
        let c1 = h.compact();
        let c2 = h.compact();
        assert_eq!(c1, c2);
    }

    #[test]
    fn compact_different_for_different_hashes() {
        let h1 = Hash::hash(b"data1").expect("hashing should succeed");
        let h2 = Hash::hash(b"data2").expect("hashing should succeed");
        assert_ne!(h1.compact(), h2.compact());
    }

    #[test]
    fn compact_same_for_same_hashes() {
        let h1 = Hash::hash(b"same").expect("hashing should succeed");
        let h2 = Hash::hash(b"same").expect("hashing should succeed");
        assert_eq!(h1.compact(), h2.compact());
    }

    #[test]
    fn compact_contains_digest() {
        let h = Hash::hash(b"digest in compact").expect("hashing should succeed");
        let compact = h.compact();
        let digest = h.digest();
        assert!(compact.starts_with(digest));
    }

    #[test]
    fn compact_can_be_recovered() {
        let original = Hash::hash(b"recoverable").expect("hashing should succeed");
        let compact = original.compact().to_vec();
        let mut vec = compact;
        let recovered = Hash::validate_bin_vec(&mut vec)
            .expect("recovery from the compact form should succeed");
        assert_eq!(original, recovered);
    }

    #[test]
    fn compact_preserves_data_max_len() {
        let data = b"preserve length";
        let original = Hash::hash(data).expect("hashing should succeed");
        let mut vec = original.compact().to_vec();
        let recovered = Hash::validate_bin_vec(&mut vec)
            .expect("recovery from the compact form should succeed");
        assert_eq!(original.data_max_len(), recovered.data_max_len());
    }
}
