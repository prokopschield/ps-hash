use crate::{Hash, DIGEST_SIZE};

impl Hash {
    #[must_use]
    pub fn digest(&self) -> &[u8] {
        &self.inner[..DIGEST_SIZE]
    }
}
