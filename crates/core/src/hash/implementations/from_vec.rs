use super::super::Hash;

impl From<Hash> for Vec<u8> {
    fn from(value: Hash) -> Self {
        value.to_string().into_bytes()
    }
}

impl From<&Hash> for Vec<u8> {
    fn from(value: &Hash) -> Self {
        value.to_string().into_bytes()
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use crate::{Hash, HASH_SIZE_CROCKFORD};

    #[test]
    fn from_hash_to_vec_correct_length() {
        let h = Hash::hash(b"test").expect("hashing should succeed");
        let v: Vec<u8> = h.into();
        assert_eq!(v.len(), HASH_SIZE_CROCKFORD);
    }

    #[test]
    fn from_hash_to_vec_matches_to_string() {
        let h = Hash::hash(b"matches").expect("hashing should succeed");
        let v: Vec<u8> = h.into();
        let s = h.to_string();
        assert_eq!(v, s.into_bytes());
    }

    #[test]
    fn from_hash_to_vec_is_deterministic() {
        let h = Hash::hash(b"deterministic").expect("hashing should succeed");
        let v1: Vec<u8> = h.into();
        let v2: Vec<u8> = h.into();
        assert_eq!(v1, v2);
    }

    #[test]
    fn from_hash_ref_to_vec_correct_length() {
        let h = Hash::hash(b"ref test").expect("hashing should succeed");
        let v: Vec<u8> = (&h).into();
        assert_eq!(v.len(), HASH_SIZE_CROCKFORD);
    }

    #[test]
    fn from_hash_ref_to_vec_matches_to_string() {
        let h = Hash::hash(b"ref matches").expect("hashing should succeed");
        let v: Vec<u8> = (&h).into();
        let s = h.to_string();
        assert_eq!(v, s.into_bytes());
    }

    #[test]
    fn from_hash_ref_to_vec_is_deterministic() {
        let h = Hash::hash(b"ref deterministic").expect("hashing should succeed");
        let v1: Vec<u8> = (&h).into();
        let v2: Vec<u8> = (&h).into();
        assert_eq!(v1, v2);
    }

    #[test]
    fn from_hash_to_vec_different_for_different_data() {
        let h1 = Hash::hash(b"data1").expect("hashing should succeed");
        let h2 = Hash::hash(b"data2").expect("hashing should succeed");
        let v1: Vec<u8> = h1.into();
        let v2: Vec<u8> = h2.into();
        assert_ne!(v1, v2);
    }

    #[test]
    fn from_hash_ref_preserves_original() {
        let h = Hash::hash(b"preserved").expect("hashing should succeed");
        let _v: Vec<u8> = (&h).into();
        assert_eq!(h.to_string().len(), HASH_SIZE_CROCKFORD);
    }

    #[test]
    fn from_hash_to_vec_is_valid_utf8() {
        let h = Hash::hash(b"utf8").expect("hashing should succeed");
        let v: Vec<u8> = h.into();
        assert!(std::str::from_utf8(&v).is_ok());
    }

    #[test]
    fn from_hash_to_vec_round_trips() {
        let original = Hash::hash(b"round trip").expect("hashing should succeed");
        let v: Vec<u8> = original.into();
        let recovered = Hash::validate(&v).expect("round trip through validate should succeed");
        assert_eq!(original, recovered);
    }
}
