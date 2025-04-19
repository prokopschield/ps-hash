#![allow(clippy::missing_panics_doc)]
#![allow(clippy::unwrap_used)]

use ps_buffer::Buffer;
use ps_pint16::PackedInt;

use crate::error::HashError;

#[test]
pub fn hash() -> Result<(), HashError> {
    let test_str = b"Hello, world!";
    let test_value = test_str.as_slice();
    let hash_value = super::hash(test_value)?.to_string();

    assert_eq!(
        "3Lqbann~vFOn43UpL64ukdU4TlKXU4nFFvUZCL1iFF4NAFBGLtfcLLDPwF92CquL",
        hash_value
    );

    let parts = super::decode_parts(hash_value.as_bytes()).unwrap();

    assert_eq!(parts.2.to_usize(), test_value.len());

    Ok(())
}

#[test]
pub fn hash_length() -> Result<(), HashError> {
    for input_length in 0..10000 {
        let input = b"F".repeat(input_length);
        let hash = super::hash(input.as_slice())?;
        let (_, _, length) = super::decode_parts(hash.as_bytes()).unwrap();

        assert_eq!(
            PackedInt::from_usize(input_length),
            length,
            "{input_length}"
        );
    }

    Ok(())
}

#[test]
pub fn data_max_len() -> Result<(), HashError> {
    for i in 0..10000 {
        let buffer = Buffer::alloc(i)?;
        let hash = crate::hash(buffer)?;
        let length = crate::Hash::data_max_len(&hash).unwrap();

        assert_eq!(
            length,
            PackedInt::from_usize(i).to_usize(),
            "data_max_len({i}) test yielded {length}"
        );
    }

    Ok(())
}
