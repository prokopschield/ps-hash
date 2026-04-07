use ps_base64::base64;

use crate::HASH_SIZE;

use super::super::Hash;

impl From<Hash> for [u8; HASH_SIZE] {
    fn from(hash: Hash) -> [u8; HASH_SIZE] {
        base64::sized_encode(&hash.inner)
    }
}
