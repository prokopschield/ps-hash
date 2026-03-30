use ps_base64::base64;
use ps_ecc::RSGenerateParityError;

use crate::{hash_inner, HASH_SIZE};

pub fn hash_encoded(data: &[u8]) -> Result<[u8; HASH_SIZE], RSGenerateParityError> {
    let inner = hash_inner(data)?;

    Ok(base64::sized_encode(&inner))
}

#[cfg(test)]
mod tests {
    use ps_base64::base64;

    use super::{hash_encoded, hash_inner, HASH_SIZE};

    #[test]
    fn hash_encoded_matches_known_vector() {
        let encoded = hash_encoded(b"Hello, world!").expect("hash_encoded should work");

        assert_eq!(
            String::from_utf8_lossy(&encoded),
            "3Lqbann-vFOn43UpL64ukdU4TlKXU4nFFvUZCL1iFF4NAFBGLtfcLLDPwF92CquL"
        );
    }

    #[test]
    fn hash_encoded_matches_encoded_inner() {
        let data = b"encode consistency";
        let encoded = hash_encoded(data).expect("hash_encoded should work");
        let inner = hash_inner(data).expect("hash_inner should work");
        let expected = base64::sized_encode(&inner);

        assert_eq!(encoded, expected);
        assert_eq!(encoded.len(), HASH_SIZE);
    }
}
