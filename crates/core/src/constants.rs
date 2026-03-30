pub const DIGEST_SIZE: usize = 32;
pub const HASH_SIZE_BIN: usize = 48;
pub const HASH_SIZE: usize = 64;
pub const PARITY: u8 = 7;
pub const PARITY_OFFSET: usize = 34;

#[cfg(test)]
mod tests {
    use super::{DIGEST_SIZE, HASH_SIZE, HASH_SIZE_BIN, PARITY, PARITY_OFFSET};

    #[test]
    fn constants_are_consistent() {
        assert_eq!(DIGEST_SIZE, 32);
        assert_eq!(PARITY_OFFSET, DIGEST_SIZE + std::mem::size_of::<u16>());
        assert_eq!(HASH_SIZE_BIN, 48);
        assert_eq!(HASH_SIZE, 64);
        assert_eq!(PARITY, 7);
    }
}
