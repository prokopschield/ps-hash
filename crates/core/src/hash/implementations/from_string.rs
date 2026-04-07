use super::super::Hash;

impl From<Hash> for String {
    fn from(value: Hash) -> Self {
        value.to_string()
    }
}

impl From<&Hash> for String {
    fn from(value: &Hash) -> Self {
        value.to_string()
    }
}
