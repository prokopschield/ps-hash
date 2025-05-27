use crate::Hash;

impl Hash {
    #[must_use]
    pub const fn as_str(&self) -> &str {
        unsafe {
            // safe because Hash is guaranteed to be valid ASCII
            std::str::from_utf8_unchecked(&self.inner)
        }
    }
}
