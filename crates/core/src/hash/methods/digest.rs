use ps_util::subarray;

use crate::DIGEST_SIZE;

use super::super::Hash;

impl Hash {
    #[must_use]
    pub fn digest(&self) -> &[u8; DIGEST_SIZE] {
        subarray(&self.inner, 0)
    }
}
