use ps_base64::sized_decode;
use ps_ecc::ReedSolomon;

use crate::{Hash, HASH_SIZE, HASH_SIZE_BIN, MIN_RECOVERABLE, MIN_RECOVERABLE_BIN, PARITY_OFFSET};

impl Hash {
    pub fn validate(bytes: impl AsRef<[u8]>) -> Result<Self, crate::HashValidationError> {
        let bytes = bytes.as_ref();

        let mut hash = Self {
            inner: match bytes.len() {
                MIN_RECOVERABLE_BIN..=HASH_SIZE_BIN => {
                    // The constant 0xF4 is chosen arbitrarily.
                    // Using 0x00 produces Ok(AAA...AAA) for short inputs.
                    let mut inner = [0xF4; HASH_SIZE_BIN];

                    inner[..bytes.len()].copy_from_slice(bytes);

                    inner
                }

                // If the input is Base64, decode it before proceeding.
                MIN_RECOVERABLE..=HASH_SIZE => sized_decode::<HASH_SIZE_BIN>(bytes),

                // If the length matches neither format, it's not valid.
                len => Err(crate::HashValidationError::InvalidLength(len))?,
            },
        };

        let (data, parity) = hash.inner.split_at_mut(PARITY_OFFSET);

        ReedSolomon::correct_detached_in_place(parity, data)?;

        Ok(hash)
    }
}
