use ps_base64::base64;

use crate::HASH_SIZE;

use super::super::Hash;

impl From<Hash> for [u8; HASH_SIZE] {
    fn from(hash: Hash) -> [u8; HASH_SIZE] {
        base64::sized_encode(&hash.inner)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Hash, HASH_SIZE};

    #[test]
    fn from_hash_to_array_correct_size() {
        let h = Hash::hash(b"test").unwrap();
        let arr: [u8; HASH_SIZE] = h.into();
        assert_eq!(arr.len(), HASH_SIZE);
    }

    #[test]
    fn from_hash_to_array_is_64_bytes() {
        let h = Hash::hash(b"64 bytes").unwrap();
        let arr: [u8; HASH_SIZE] = h.into();
        assert_eq!(arr.len(), 64);
    }

    #[test]
    fn from_hash_to_array_matches_to_string() {
        let h = Hash::hash(b"matches").unwrap();
        let arr: [u8; HASH_SIZE] = h.into();
        let s = h.to_string();
        assert_eq!(&arr[..], s.as_bytes());
    }

    #[test]
    fn from_hash_to_array_is_deterministic() {
        let h = Hash::hash(b"deterministic").unwrap();
        let arr1: [u8; HASH_SIZE] = h.into();
        let arr2: [u8; HASH_SIZE] = h.into();
        assert_eq!(arr1, arr2);
    }

    #[test]
    fn from_hash_to_array_different_for_different_data() {
        let h1 = Hash::hash(b"data1").unwrap();
        let h2 = Hash::hash(b"data2").unwrap();
        let arr1: [u8; HASH_SIZE] = h1.into();
        let arr2: [u8; HASH_SIZE] = h2.into();
        assert_ne!(arr1, arr2);
    }

    #[test]
    fn from_hash_to_array_same_for_same_data() {
        let h1 = Hash::hash(b"same").unwrap();
        let h2 = Hash::hash(b"same").unwrap();
        let arr1: [u8; HASH_SIZE] = h1.into();
        let arr2: [u8; HASH_SIZE] = h2.into();
        assert_eq!(arr1, arr2);
    }

    #[test]
    fn from_hash_to_array_is_valid_utf8() {
        let h = Hash::hash(b"utf8").unwrap();
        let arr: [u8; HASH_SIZE] = h.into();
        assert!(std::str::from_utf8(&arr).is_ok());
    }

    #[test]
    fn from_hash_to_array_round_trips() {
        let original = Hash::hash(b"round trip").unwrap();
        let arr: [u8; HASH_SIZE] = original.into();
        let recovered = Hash::validate(&arr).unwrap();
        assert_eq!(original, recovered);
    }
}
