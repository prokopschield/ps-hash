mod implementations;
mod methods;

use ps_ecc::ReedSolomon;

use crate::{HashError, HASH_SIZE_BIN, PARITY};

pub const RS: ReedSolomon = match ReedSolomon::new(PARITY) {
    Ok(rs) => rs,
    Err(_) => panic!("Failed to construct Reed-Solomon codec."),
};

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Hash {
    pub(crate) inner: [u8; HASH_SIZE_BIN],
}

#[inline]
pub fn hash(data: impl AsRef<[u8]>) -> Result<Hash, HashError> {
    Hash::hash(data)
}
