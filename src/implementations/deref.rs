use std::ops::Deref;

use crate::{Hash, HASH_SIZE_BIN};

impl Deref for Hash {
    type Target = [u8; HASH_SIZE_BIN];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
