use ps_util::subarray;

use crate::{Hash, PARITY_OFFSET, PARITY_SIZE};

impl Hash {
    #[must_use]
    pub fn parity(&self) -> &[u8; PARITY_SIZE] {
        subarray(&self.inner, PARITY_OFFSET)
    }
}
