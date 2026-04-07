use super::super::Hash;

impl std::hash::Hash for Hash {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(&self.inner);
    }
}
