#![allow(clippy::missing_panics_doc)]
#![allow(clippy::unwrap_used)]

mod g250419;
mod h251027;

use ps_pint16::PackedInt;

use crate::{error::HashError, Hash};

#[test]
pub fn hash() -> Result<(), HashError> {
    let test_str = b"Hello, world!";
    let test_value = test_str.as_slice();
    let hash_value = super::hash(test_value)?.to_string();

    assert_eq!(
        "3Lqbann-vFOn43UpL64ukdU4TlKXU4nFFvUZCL1iFF4NAFBGLtfcLLDPwF92CquL",
        hash_value
    );

    assert_eq!(
        Hash::validate(hash_value).unwrap().length().to_usize(),
        test_value.len()
    );

    Ok(())
}

#[test]
pub fn hash_length() -> Result<(), HashError> {
    for input_length in 0..10000 {
        let input = b"F".repeat(input_length);
        let hash = super::hash(input.as_slice())?;
        let length = hash.length();

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
        let mut buffer = Vec::with_capacity(i);

        buffer.resize_with(i, || 42);

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
