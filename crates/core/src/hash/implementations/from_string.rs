use super::super::Hash;

impl From<Hash> for String {
    fn from(value: Hash) -> Self {
        value.to_string()
    }
}

impl From<&Hash> for String {
    fn from(value: &Hash) -> Self {
        value.to_string()
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use crate::{Hash, HASH_SIZE_CROCKFORD};

    #[test]
    fn from_hash_to_string_correct_length() {
        let h = Hash::hash(b"test").expect("hashing should succeed");
        let s: String = h.into();
        assert_eq!(s.len(), HASH_SIZE_CROCKFORD);
    }

    #[test]
    fn from_hash_to_string_matches_to_string() {
        let h = Hash::hash(b"matches").expect("hashing should succeed");
        let s1: String = h.into();
        let s2 = h.to_string();
        assert_eq!(s1, s2);
    }

    #[test]
    fn from_hash_to_string_is_deterministic() {
        let h = Hash::hash(b"deterministic").expect("hashing should succeed");
        let s1: String = h.into();
        let s2: String = h.into();
        assert_eq!(s1, s2);
    }

    #[test]
    fn from_hash_ref_to_string_correct_length() {
        let h = Hash::hash(b"ref test").expect("hashing should succeed");
        let s: String = (&h).into();
        assert_eq!(s.len(), HASH_SIZE_CROCKFORD);
    }

    #[test]
    fn from_hash_ref_to_string_matches_to_string() {
        let h = Hash::hash(b"ref matches").expect("hashing should succeed");
        let s1: String = (&h).into();
        let s2 = h.to_string();
        assert_eq!(s1, s2);
    }

    #[test]
    fn from_hash_ref_to_string_is_deterministic() {
        let h = Hash::hash(b"ref deterministic").expect("hashing should succeed");
        let s1: String = (&h).into();
        let s2: String = (&h).into();
        assert_eq!(s1, s2);
    }

    #[test]
    fn from_hash_to_string_different_for_different_data() {
        let h1 = Hash::hash(b"data1").expect("hashing should succeed");
        let h2 = Hash::hash(b"data2").expect("hashing should succeed");
        let s1: String = h1.into();
        let s2: String = h2.into();
        assert_ne!(s1, s2);
    }

    #[test]
    fn from_hash_ref_preserves_original() {
        let h = Hash::hash(b"preserved").expect("hashing should succeed");
        let _s: String = (&h).into();
        assert_eq!(h.to_string().len(), HASH_SIZE_CROCKFORD);
    }

    #[test]
    fn from_hash_to_string_round_trips() {
        let original = Hash::hash(b"round trip").expect("hashing should succeed");
        let s: String = original.into();
        let recovered = Hash::validate(&s).expect("round trip through validate should succeed");
        assert_eq!(original, recovered);
    }
}
