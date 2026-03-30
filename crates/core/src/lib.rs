mod constants;
mod digest;
mod encode;
mod inner;

pub use constants::{DIGEST_SIZE, HASH_SIZE, HASH_SIZE_BIN, PARITY, PARITY_OFFSET};
pub use digest::{blake3, sha256};
pub use encode::hash_encoded;
pub use inner::hash_inner;

#[cfg(test)]
mod tests {
    use super::{blake3, hash_encoded, hash_inner, sha256, DIGEST_SIZE, HASH_SIZE, HASH_SIZE_BIN};

    #[test]
    fn public_api_exports_work() {
        let data = b"core api";

        assert_eq!(sha256(data).len(), DIGEST_SIZE);
        assert_eq!(blake3(data).as_bytes().len(), DIGEST_SIZE);
        assert_eq!(
            hash_inner(data).expect("hash_inner should work").len(),
            HASH_SIZE_BIN
        );
        assert_eq!(
            hash_encoded(data).expect("hash_encoded should work").len(),
            HASH_SIZE
        );
    }
}
