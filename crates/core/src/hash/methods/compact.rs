use crate::HASH_SIZE_COMPACT;

use super::super::Hash;

impl Hash {
    #[inline]
    #[must_use]
    pub fn compact(&self) -> &[u8] {
        &self.inner[..HASH_SIZE_COMPACT]
    }
}
