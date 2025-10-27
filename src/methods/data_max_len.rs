use ps_pint16::PackedInt;

use crate::{Hash, DIGEST_SIZE};

impl Hash {
    #[must_use]
    pub fn data_max_len(&self) -> PackedInt {
        PackedInt::from_16_bits(&[self.inner[DIGEST_SIZE], self.inner[DIGEST_SIZE + 1]])
    }
}
