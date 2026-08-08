use quote::quote;
use syn::Expr;

use crate::input::parse_bytes;

pub fn expand_hash(expr: &Expr) -> syn::Result<proc_macro2::TokenStream> {
    let bytes = parse_bytes(expr)?;

    let hash = ps_hash_core::hash_encoded(&bytes)
        .map_err(|error| syn::Error::new_spanned(expr, error.to_string()))?;

    let hash_string = std::str::from_utf8(&hash)
        .map_err(|error| syn::Error::new_spanned(expr, error.to_string()))?;

    Ok(quote!(#hash_string))
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use syn::parse_quote;

    use super::expand_hash;

    #[test]
    fn expands_to_expected_literal() {
        let expr = parse_quote!("compile-time hash");
        let tokens = expand_hash(&expr).expect("expansion should succeed");
        let literal: syn::LitStr = syn::parse2(tokens).expect("output should be a string literal");

        let expected =
            ps_hash_core::hash_encoded(b"compile-time hash").expect("hash_encoded should succeed");
        let expected = String::from_utf8_lossy(&expected).into_owned();

        assert_eq!(literal.value(), expected);
    }

    #[test]
    fn reports_error_for_unsupported_input() {
        let expr = parse_quote!(non_literal);
        assert!(expand_hash(&expr).is_err());
    }
}
