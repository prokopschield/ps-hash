mod implementations;
mod methods;

use ps_ecc::ReedSolomon;

use crate::{HashError, HASH_SIZE_BIN, PARITY};

pub const RS: ReedSolomon = match ReedSolomon::new(PARITY) {
    Ok(rs) => rs,
    Err(_) => panic!("Failed to construct Reed-Solomon codec."),
};

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Hash {
    pub(crate) inner: [u8; HASH_SIZE_BIN],
}

#[inline]
pub fn hash(data: impl AsRef<[u8]>) -> Result<Hash, HashError> {
    Hash::hash(data)
}

#[cfg(test)]
mod tests {
    use super::{hash, Hash, HASH_SIZE_BIN, PARITY, RS};

    #[test]
    fn rs_codec_is_valid() {
        assert_eq!(RS.parity(), PARITY);
    }

    #[test]
    fn rs_codec_parity_matches_constant() {
        assert_eq!(RS.parity(), 7);
    }

    #[test]
    fn hash_struct_has_correct_size() {
        assert_eq!(std::mem::size_of::<Hash>(), HASH_SIZE_BIN);
    }

    #[test]
    fn hash_struct_is_copy() {
        fn assert_copy<T: Copy>() {}
        assert_copy::<Hash>();
    }

    #[test]
    fn hash_struct_is_clone() {
        fn assert_clone<T: Clone>() {}
        assert_clone::<Hash>();
    }

    #[test]
    fn hash_function_delegates_to_method() {
        let data = b"test data";
        let via_fn = hash(data).unwrap();
        let via_method = Hash::hash(data).unwrap();
        assert_eq!(via_fn, via_method);
    }

    #[test]
    fn hash_function_returns_ok_for_empty() {
        assert!(hash(b"").is_ok());
    }

    #[test]
    fn hash_function_returns_ok_for_non_empty() {
        assert!(hash(b"non-empty").is_ok());
    }

    #[test]
    fn hash_inner_field_has_correct_size() {
        let h = hash(b"test").unwrap();
        assert_eq!(h.inner.len(), HASH_SIZE_BIN);
    }
}
