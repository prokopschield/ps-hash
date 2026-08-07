//! Known-answer vectors that freeze the on-the-wire format.
//!
//! Every value below was produced by this crate and must never change without
//! a deliberate version bump: the hashes are content addresses, so a silent
//! drift in the digest, the length field, the parity bytes, or either alphabet
//! would orphan every stored object.

use crate::{encoding, hash_inner, Hash};

/// `(input, Crockford Base32, base64url)`.
const VECTORS: &[(&str, &str, &str)] = &[
    (
        "",
        "9JHRVYVD0PYV4ENVQ4HAZCVGDPY6AS1DS5D83YV81W5D372DH8VG0048NADNPPKJ2R39SGKDSTQA8",
        "TKON-20FvbI6u7kir7NwbbxlZC3JWoH7aA8K0ZxNijcAAIiqm1tachYGnMJtzq6k",
    ),
    (
        "a",
        "VQGTXKRKE8CSJT4ZAK1E1R1JXYTACPCK81463B5CJBXEJ06FD7J0206MZC69C2Z7XCTXP0G66DFWR",
        "3eGuzxNyGZlon1TC4OAy77SmWZNASGGsrJL66QDPaeQBANT7DJYL5-s12wIGM1_M",
    ),
    (
        "abc",
        "VS7TA4XQ8YFDKFQQFENQN55FJVWCCEF5SDQA37T94NHGT9XXH0M060309HK7GMG5567NJZW1JH1ER",
        "3k-lE7dHntm-93urepSvlvjGOeXLbqGfSSVjDSe9iCgDAGBMZnhSBSmPWX-BlELs",
    ),
    (
        "hello world",
        "DR2ACNHM8WS81BN3VV75ZBEY447Q55JTW061KYKN487CERFNAF6GP05EEHVKGSCBPZ7GG3V4Z77HR",
        "bgSmVjRHMoCuo97OX63eIQ9yllrgDBn6dSIOx2H1U80LAK50dzhli7fPCA9k-c8c",
    ),
    (
        "The quick brown fox jumps over the lazy dog",
        "Z2YYZARXF964TYK18FR4MM899RM3WETPAARJNAEWPPBTZJBNH3C2P06KYEKE7T3JRZ79BWZEFPQ42",
        "-L3vqx16TE16YUPwSlEJTig-O1ZSsSqp3LWXr8l1iNgrANPzpuPocsfOlfPufa5B",
    ),
    (
        "Hello, world!",
        "VJX9PTKSZTY579Z3EMMJZBHEJ7AKGKJJJX9RKH8PYMCGHFB22HF0T02G8RQDFQ1CP37W0QVP1ANRP",
        "3Lqbann-vFOn43UpL64ukdU4TlKXU4nFFvUZCL1iFF4NAFBGLtfcLLDPwF92CquL",
    ),
];

/// A thousand `0x5A` bytes, exercising a length well beyond the short
/// [`VECTORS`] inputs.
const LONG_INPUT_VECTOR: (&str, &str) = (
    "ZVJFND3135B2FZDKXCYZYPNW30XQ6ZPYTC6SE5795D68Z0K1675Z80MBTE463XCXST5YDXME9NV70",
    "_uT6tGEZVif9s-s9_1q8GDtzft7TDZcU6StMj4JhMcv0AovTiGH1nc6L5vaOTXZw",
);

#[test]
fn crockford_vectors_are_stable() {
    for (input, crockford, _) in VECTORS {
        let hash = Hash::hash(input).expect("hashing should succeed");

        assert_eq!(hash.to_crockford(), *crockford, "Crockford for {input:?}");
    }
}

#[test]
fn base64_vectors_are_stable() {
    for (input, _, base64) in VECTORS {
        let hash = Hash::hash(input).expect("hashing should succeed");

        assert_eq!(hash.to_base64(), *base64, "base64url for {input:?}");
    }
}

#[test]
fn long_input_vector_is_stable() {
    let hash = Hash::hash(vec![0x5A; 1000]).expect("hashing should succeed");

    assert_eq!(hash.to_crockford(), LONG_INPUT_VECTOR.0);
    assert_eq!(hash.to_base64(), LONG_INPUT_VECTOR.1);
}

#[test]
fn vectors_decode_back_to_the_same_hash() {
    for (input, crockford, base64) in VECTORS {
        let expected = Hash::hash(input).expect("hashing should succeed");

        assert_eq!(
            Hash::validate(crockford).expect("validation of a stable vector should succeed"),
            expected
        );
        assert_eq!(
            Hash::validate(base64).expect("validation of a stable vector should succeed"),
            expected
        );
    }
}

#[test]
fn both_representations_encode_the_same_bytes() {
    for (input, crockford, base64) in VECTORS {
        let inner = hash_inner(input.as_bytes()).expect("hashing should succeed");

        assert_eq!(encoding::crockford::decode(crockford.as_bytes()), inner);
        assert_eq!(encoding::base64::decode(base64.as_bytes()), inner);
    }
}

#[test]
fn vectors_have_the_documented_lengths() {
    for (input, crockford, base64) in VECTORS {
        assert_eq!(crockford.len(), crate::HASH_SIZE_CROCKFORD, "{input:?}");
        assert_eq!(base64.len(), crate::HASH_SIZE_BASE64, "{input:?}");
    }
}
