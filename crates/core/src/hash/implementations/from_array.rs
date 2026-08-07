use crate::{
    encoding::{base64, crockford},
    HASH_SIZE_BASE64, HASH_SIZE_CROCKFORD,
};

use super::super::Hash;

impl From<Hash> for [u8; HASH_SIZE_CROCKFORD] {
    fn from(hash: Hash) -> [u8; HASH_SIZE_CROCKFORD] {
        crockford::encode(&hash.inner)
    }
}

impl From<Hash> for [u8; HASH_SIZE_BASE64] {
    fn from(hash: Hash) -> [u8; HASH_SIZE_BASE64] {
        base64::encode(&hash.inner)
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use crate::{Hash, HASH_SIZE_BASE64, HASH_SIZE_CROCKFORD};

    #[test]
    fn both_array_sizes_are_reachable() {
        let h = Hash::hash(b"test").expect("hashing should succeed");

        let crockford: [u8; HASH_SIZE_CROCKFORD] = h.into();
        let base64: [u8; HASH_SIZE_BASE64] = h.into();

        assert_eq!(crockford.len(), 77);
        assert_eq!(base64.len(), 64);
    }

    #[test]
    fn arrays_match_their_string_representations() {
        let h = Hash::hash(b"matches").expect("hashing should succeed");

        let crockford: [u8; HASH_SIZE_CROCKFORD] = h.into();
        let base64: [u8; HASH_SIZE_BASE64] = h.into();

        assert_eq!(&crockford[..], h.to_crockford().as_bytes());
        assert_eq!(&base64[..], h.to_base64().as_bytes());
    }

    #[test]
    fn arrays_are_deterministic() {
        let h = Hash::hash(b"deterministic").expect("hashing should succeed");

        let first: [u8; HASH_SIZE_CROCKFORD] = h.into();
        let second: [u8; HASH_SIZE_CROCKFORD] = h.into();

        assert_eq!(first, second);
    }

    #[test]
    fn arrays_differ_for_different_data() {
        let h1 = Hash::hash(b"data1").expect("hashing should succeed");
        let h2 = Hash::hash(b"data2").expect("hashing should succeed");

        let arr1: [u8; HASH_SIZE_CROCKFORD] = h1.into();
        let arr2: [u8; HASH_SIZE_CROCKFORD] = h2.into();

        assert_ne!(arr1, arr2);
    }

    #[test]
    fn arrays_agree_for_the_same_data() {
        let h1 = Hash::hash(b"same").expect("hashing should succeed");
        let h2 = Hash::hash(b"same").expect("hashing should succeed");

        let arr1: [u8; HASH_SIZE_BASE64] = h1.into();
        let arr2: [u8; HASH_SIZE_BASE64] = h2.into();

        assert_eq!(arr1, arr2);
    }

    #[test]
    fn arrays_are_valid_utf8() {
        let h = Hash::hash(b"utf8").expect("hashing should succeed");

        let crockford: [u8; HASH_SIZE_CROCKFORD] = h.into();
        let base64: [u8; HASH_SIZE_BASE64] = h.into();

        assert!(std::str::from_utf8(&crockford).is_ok());
        assert!(std::str::from_utf8(&base64).is_ok());
    }

    #[test]
    fn both_arrays_round_trip_through_validate() {
        let original = Hash::hash(b"round trip").expect("hashing should succeed");

        let crockford: [u8; HASH_SIZE_CROCKFORD] = original.into();
        let base64: [u8; HASH_SIZE_BASE64] = original.into();

        assert_eq!(
            Hash::validate(crockford).expect("round trip through validate should succeed"),
            original
        );
        assert_eq!(
            Hash::validate(base64).expect("round trip through validate should succeed"),
            original
        );
    }
}
