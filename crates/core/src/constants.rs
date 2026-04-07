pub const DIGEST_SIZE: usize = 32;
pub const HASH_SIZE_BIN: usize = 48;
pub const HASH_SIZE_COMPACT: usize = 42;
pub const HASH_SIZE: usize = 64;
pub const PARITY: u8 = 7;
pub const PARITY_OFFSET: usize = 34;
pub const PARITY_SIZE: usize = 14;
pub const SIZE_SIZE: usize = std::mem::size_of::<u16>();
pub const MIN_RECOVERABLE: usize = HASH_SIZE - (PARITY as usize * 8 / 6);
pub const MIN_RECOVERABLE_BIN: usize = HASH_SIZE_BIN - (PARITY as usize);

#[cfg(test)]
mod tests {
    use super::{
        DIGEST_SIZE, HASH_SIZE, HASH_SIZE_BIN, HASH_SIZE_COMPACT, MIN_RECOVERABLE,
        MIN_RECOVERABLE_BIN, PARITY, PARITY_OFFSET, PARITY_SIZE, SIZE_SIZE,
    };

    #[test]
    fn constants_are_consistent() {
        assert_eq!(DIGEST_SIZE, 32);
        assert_eq!(PARITY_OFFSET, DIGEST_SIZE + std::mem::size_of::<u16>());
        assert_eq!(HASH_SIZE_BIN, 48);
        assert_eq!(HASH_SIZE_COMPACT, 42);
        assert_eq!(HASH_SIZE, 64);
        assert_eq!(PARITY, 7);
        assert_eq!(PARITY_SIZE, 14);
        assert_eq!(SIZE_SIZE, std::mem::size_of::<u16>());
        assert_eq!(MIN_RECOVERABLE, 55);
        assert_eq!(MIN_RECOVERABLE_BIN, 41);
    }
}
