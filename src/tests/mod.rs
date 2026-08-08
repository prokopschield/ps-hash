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
    let hash_value = super::hash(test_value)?.to_base64();

    assert_eq!(
        "3Lqbann-vFOn43UpL64ukdU4TlKXU4nFFvUZCL1iFF4NAFBGLtfcLLDPwF92CquL",
        hash_value
    );

    assert_eq!(
        Hash::validate(hash_value)
            .unwrap()
            .data_max_len()
            .to_usize(),
        test_value.len()
    );

    Ok(())
}

#[test]
pub fn hash_data_max_len() -> Result<(), HashError> {
    for input_length in 0..10000 {
        let input = b"F".repeat(input_length);
        let hash = super::hash(input.as_slice())?;
        let length = hash.data_max_len();

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
        let length = crate::Hash::data_max_len(&hash).to_usize();

        assert_eq!(
            length,
            PackedInt::from_usize(i).to_usize(),
            "data_max_len({i}) test yielded {length}"
        );
    }

    Ok(())
}

#[test]
pub fn hash_macro_string_literal() -> Result<(), HashError> {
    const MACRO_HASH: &str = crate::hash!("compile-time hash");
    let runtime_hash = crate::hash("compile-time hash")?.to_string();

    assert_eq!(MACRO_HASH, runtime_hash);

    Ok(())
}

#[test]
pub fn hash_macro_byte_string_literal() -> Result<(), HashError> {
    const MACRO_HASH: &str = crate::hash!(b"compile-time hash bytes");
    let runtime_hash = crate::hash(b"compile-time hash bytes")?.to_string();

    assert_eq!(MACRO_HASH, runtime_hash);

    Ok(())
}

#[test]
pub fn hash_macro_through_macro_rules() -> Result<(), HashError> {
    macro_rules! wrap {
        ($input:expr) => {
            crate::hash!($input)
        };
    }

    const MACRO_HASH: &str = wrap!("wrapped literal");
    let runtime_hash = crate::hash("wrapped literal")?.to_string();

    assert_eq!(MACRO_HASH, runtime_hash);

    Ok(())
}

#[test]
pub fn hash_macro_array_elements_through_macro_rules() -> Result<(), HashError> {
    macro_rules! build_array {
        ($($byte:expr),*) => {
            crate::hash!([$($byte),*])
        };
    }

    const MACRO_HASH: &str = build_array!(1, 2, 255, b'a');
    let runtime_hash = crate::hash([1_u8, 2, 255, b'a'])?.to_string();

    assert_eq!(MACRO_HASH, runtime_hash);

    Ok(())
}

#[test]
pub fn hash_macro_byte_array_literal() -> Result<(), HashError> {
    const MACRO_HASH: &str = crate::hash!([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let runtime_hash = crate::hash([0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9])?.to_string();

    assert_eq!(MACRO_HASH, runtime_hash);

    Ok(())
}

#[test]
pub fn hash_macro_repeat_expression() -> Result<(), HashError> {
    const MACRO_HASH: &str = crate::hash!([42u8; 16]);
    let runtime_hash = crate::hash([42_u8; 16])?.to_string();

    assert_eq!(MACRO_HASH, runtime_hash);

    Ok(())
}

#[test]
pub fn hash_macro_repeat_through_macro_rules() -> Result<(), HashError> {
    macro_rules! build_repeat {
        ($byte:expr, $len:expr) => {
            crate::hash!([$byte; $len])
        };
    }

    const MACRO_HASH: &str = build_repeat!(9, 5);
    let runtime_hash = crate::hash([9_u8; 5])?.to_string();

    assert_eq!(MACRO_HASH, runtime_hash);

    Ok(())
}

#[test]
pub fn hash_macro_empty_string() -> Result<(), HashError> {
    const MACRO_HASH: &str = crate::hash!("");
    let runtime_hash = crate::hash("")?.to_string();

    assert_eq!(MACRO_HASH, runtime_hash);

    Ok(())
}

#[test]
pub fn hash_macro_unicode_string() -> Result<(), HashError> {
    const MACRO_HASH: &str = crate::hash!("héllo, 🌍");
    let runtime_hash = crate::hash("héllo, 🌍")?.to_string();

    assert_eq!(MACRO_HASH, runtime_hash);

    Ok(())
}

#[test]
pub fn hash_macro_escape_sequences() -> Result<(), HashError> {
    const MACRO_HASH: &str = crate::hash!("line1\nline2\0\u{263A}");
    let runtime_hash = crate::hash("line1\nline2\0\u{263A}")?.to_string();

    assert_eq!(MACRO_HASH, runtime_hash);

    Ok(())
}
