use crate::{hash_inner, HashError};

use super::super::Hash;

impl Hash {
    #[allow(clippy::self_named_constructors)]
    pub fn hash(data: impl AsRef<[u8]>) -> Result<Self, HashError> {
        let inner = hash_inner(data.as_ref())?;
        Ok(Self { inner })
    }
}
