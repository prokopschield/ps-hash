use crate::Hash;

impl AsRef<str> for Hash {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
