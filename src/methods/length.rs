use ps_pint16::PackedInt;

use crate::{Hash, DIGEST_SIZE};

impl Hash {
    #[must_use]
    pub fn length(&self) -> PackedInt {
        PackedInt::from_16_bits(&[self.inner[DIGEST_SIZE], self.inner[DIGEST_SIZE + 1]])
    }
}
