use super::super::Hash;

impl PartialEq for Hash {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl Eq for Hash {}
