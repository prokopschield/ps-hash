use ps_ecc::ReedSolomon;

use crate::{HashValidationError, HASH_SIZE_BIN, PARITY_OFFSET};

use super::super::Hash;

impl Hash {
    pub fn validate_bin_vec(hash: &mut Vec<u8>) -> Result<Self, HashValidationError> {
        hash.resize(HASH_SIZE_BIN, 0xF4);

        let (data, parity) = hash.split_at_mut(PARITY_OFFSET);
        ReedSolomon::correct_detached_in_place(parity, data)?;

        let mut inner = [0u8; HASH_SIZE_BIN];
        inner.copy_from_slice(hash);

        Ok(Self { inner })
    }
}
