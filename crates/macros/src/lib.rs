mod expand;
mod input;

use proc_macro::TokenStream;
use syn::parse_macro_input;

fn expand_hash_expr(expr: &syn::Expr) -> proc_macro2::TokenStream {
    match expand::expand_hash(expr) {
        Ok(tokens) => tokens,
        Err(error) => error.to_compile_error(),
    }
}

/// Hashes literal input at compile time and expands to the canonical
/// Crockford Base32 form as a `&'static str`.
///
/// Accepts a string literal, byte string literal, byte array literal, or
/// byte repeat expression. The expansion equals the runtime
/// `ps_hash::hash(input)?.to_string()` for the same input. Any other input
/// produces a compile error.
///
/// # Examples
///
/// ```
/// const HASH: &str = ps_hash_macros::hash!("hello");
///
/// assert_eq!(HASH.len(), 77);
/// assert_eq!(HASH, ps_hash_macros::hash!(b"hello"));
/// assert_eq!(HASH, ps_hash_macros::hash!([b'h', b'e', b'l', b'l', b'o']));
/// ```
#[proc_macro]
pub fn hash(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as syn::Expr);
    expand_hash_expr(&expr).into()
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use syn::parse_quote;

    use super::expand_hash_expr;

    #[test]
    fn dispatcher_expands_hash_macro_input() {
        let output = expand_hash_expr(&parse_quote!("dispatcher"));
        let literal: syn::LitStr = syn::parse2(output).expect("output should be a string literal");

        let expected =
            ps_hash_core::hash_encoded(b"dispatcher").expect("hash_encoded should succeed");
        let expected = String::from_utf8_lossy(&expected).into_owned();

        assert_eq!(literal.value(), expected);
    }
}
