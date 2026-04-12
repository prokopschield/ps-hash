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
mod tests {
    use crate::{Hash, HASH_SIZE};

    #[test]
    fn from_hash_to_string_correct_length() {
        let h = Hash::hash(b"test").unwrap();
        let s: String = h.into();
        assert_eq!(s.len(), HASH_SIZE);
    }

    #[test]
    fn from_hash_to_string_matches_to_string() {
        let h = Hash::hash(b"matches").unwrap();
        let s1: String = h.into();
        let s2 = h.to_string();
        assert_eq!(s1, s2);
    }

    #[test]
    fn from_hash_to_string_is_deterministic() {
        let h = Hash::hash(b"deterministic").unwrap();
        let s1: String = h.into();
        let s2: String = h.into();
        assert_eq!(s1, s2);
    }

    #[test]
    fn from_hash_ref_to_string_correct_length() {
        let h = Hash::hash(b"ref test").unwrap();
        let s: String = (&h).into();
        assert_eq!(s.len(), HASH_SIZE);
    }

    #[test]
    fn from_hash_ref_to_string_matches_to_string() {
        let h = Hash::hash(b"ref matches").unwrap();
        let s1: String = (&h).into();
        let s2 = h.to_string();
        assert_eq!(s1, s2);
    }

    #[test]
    fn from_hash_ref_to_string_is_deterministic() {
        let h = Hash::hash(b"ref deterministic").unwrap();
        let s1: String = (&h).into();
        let s2: String = (&h).into();
        assert_eq!(s1, s2);
    }

    #[test]
    fn from_hash_to_string_different_for_different_data() {
        let h1 = Hash::hash(b"data1").unwrap();
        let h2 = Hash::hash(b"data2").unwrap();
        let s1: String = h1.into();
        let s2: String = h2.into();
        assert_ne!(s1, s2);
    }

    #[test]
    fn from_hash_ref_preserves_original() {
        let h = Hash::hash(b"preserved").unwrap();
        let _s: String = (&h).into();
        assert_eq!(h.to_string().len(), HASH_SIZE);
    }

    #[test]
    fn from_hash_to_string_round_trips() {
        let original = Hash::hash(b"round trip").unwrap();
        let s: String = original.into();
        let recovered = Hash::validate(&s).unwrap();
        assert_eq!(original, recovered);
    }
}
