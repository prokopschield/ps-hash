use crate::{hash_inner, HashError};

use super::super::Hash;

impl Hash {
    #[allow(clippy::self_named_constructors)]
    pub fn hash(data: impl AsRef<[u8]>) -> Result<Self, HashError> {
        let inner = hash_inner(data.as_ref())?;
        Ok(Self { inner })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Hash, HASH_SIZE, HASH_SIZE_BIN};

    #[test]
    fn hash_empty_data() {
        let result = Hash::hash([]);
        assert!(result.is_ok());
    }

    #[test]
    fn hash_non_empty_data() {
        let result = Hash::hash(b"hello");
        assert!(result.is_ok());
    }

    #[test]
    fn hash_large_data() {
        let data = vec![0u8; 1_000_000];
        let result = Hash::hash(&data);
        assert!(result.is_ok());
    }

    #[test]
    fn hash_is_deterministic() {
        let data = b"deterministic";
        let h1 = Hash::hash(data).unwrap();
        let h2 = Hash::hash(data).unwrap();
        assert_eq!(h1, h2);
    }

    #[test]
    fn hash_different_data_produces_different_hashes() {
        let h1 = Hash::hash(b"data1").unwrap();
        let h2 = Hash::hash(b"data2").unwrap();
        assert_ne!(h1, h2);
    }

    #[test]
    fn hash_inner_has_correct_size() {
        let h = Hash::hash(b"test").unwrap();
        assert_eq!(h.inner.len(), HASH_SIZE_BIN);
    }

    #[test]
    fn hash_to_string_has_correct_size() {
        let h = Hash::hash(b"test").unwrap();
        assert_eq!(h.to_string().len(), HASH_SIZE);
    }

    #[test]
    fn hash_accepts_slice() {
        let data: &[u8] = b"slice";
        assert!(Hash::hash(data).is_ok());
    }

    #[test]
    fn hash_accepts_vec() {
        let data: Vec<u8> = vec![1, 2, 3];
        assert!(Hash::hash(&data).is_ok());
    }

    #[test]
    fn hash_accepts_array() {
        let data: [u8; 5] = [1, 2, 3, 4, 5];
        assert!(Hash::hash(data).is_ok());
    }

    #[test]
    fn hash_single_byte_difference() {
        let h1 = Hash::hash(&[0u8]).unwrap();
        let h2 = Hash::hash(&[1u8]).unwrap();
        assert_ne!(h1, h2);
    }

    #[test]
    fn hash_length_difference() {
        let h1 = Hash::hash(&[0u8]).unwrap();
        let h2 = Hash::hash(&[0u8, 0u8]).unwrap();
        assert_ne!(h1, h2);
    }
}
