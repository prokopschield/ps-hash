use ps_util::subarray;

use crate::{Hash, DIGEST_SIZE};

impl Hash {
    #[must_use]
    pub fn digest(&self) -> &[u8; DIGEST_SIZE] {
        subarray(&self.inner, 0)
    }
}
