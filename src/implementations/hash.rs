use std::hash::Hash;

impl Hash for crate::Hash {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(&self.inner);
    }
}
