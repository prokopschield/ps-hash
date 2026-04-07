use ps_util::subarray;

use crate::{PARITY_OFFSET, PARITY_SIZE};

use super::super::Hash;

impl Hash {
    #[must_use]
    pub fn parity(&self) -> &[u8; PARITY_SIZE] {
        subarray(&self.inner, PARITY_OFFSET)
    }
}
