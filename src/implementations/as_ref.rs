use crate::Hash;

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}
