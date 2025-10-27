use crate::{Hash, PARITY_OFFSET};

impl Hash {
    #[must_use]
    pub fn parity(&self) -> &[u8] {
        &self.inner[PARITY_OFFSET..]
    }
}
