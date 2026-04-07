use super::super::Hash;

impl From<Hash> for Vec<u8> {
    fn from(value: Hash) -> Self {
        value.to_string().into_bytes()
    }
}

impl From<&Hash> for Vec<u8> {
    fn from(value: &Hash) -> Self {
        value.to_string().into_bytes()
    }
}
