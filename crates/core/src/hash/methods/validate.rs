use ps_base64::sized_decode;
use ps_ecc::ReedSolomon;

use crate::{
    HashValidationError, HASH_SIZE, HASH_SIZE_BIN, MIN_RECOVERABLE, MIN_RECOVERABLE_BIN,
    PARITY_OFFSET,
};

use super::super::Hash;

impl Hash {
    pub fn validate(bytes: impl AsRef<[u8]>) -> Result<Self, HashValidationError> {
        let bytes = bytes.as_ref();

        let mut hash = Self {
            inner: match bytes.len() {
                MIN_RECOVERABLE_BIN..=HASH_SIZE_BIN => {
                    let mut inner = [0xF4; HASH_SIZE_BIN];
                    inner[..bytes.len()].copy_from_slice(bytes);
                    inner
                }
                MIN_RECOVERABLE..=HASH_SIZE => sized_decode::<HASH_SIZE_BIN>(bytes),
                len => Err(HashValidationError::InvalidLength(len))?,
            },
        };

        let (data, parity) = hash.inner.split_at_mut(PARITY_OFFSET);
        ReedSolomon::correct_detached_in_place(parity, data)?;

        Ok(hash)
    }
}
