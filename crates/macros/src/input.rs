use syn::{Expr, ExprArray, ExprLit, Lit, LitByte, LitInt};

pub fn parse_bytes(expr: &Expr) -> syn::Result<Vec<u8>> {
    match expr {
        Expr::Group(group) => parse_bytes(&group.expr),

        Expr::Paren(paren) => parse_bytes(&paren.expr),

        Expr::Lit(ExprLit {
            lit: Lit::Str(string),
            ..
        }) => {
            reject_suffix(string, string.suffix(), "string literal")?;

            Ok(string.value().into_bytes())
        }

        Expr::Lit(ExprLit {
            lit: Lit::ByteStr(byte_string),
            ..
        }) => {
            reject_suffix(byte_string, byte_string.suffix(), "byte string literal")?;

            Ok(byte_string.value())
        }

        Expr::Array(array) => parse_byte_array(array),

        Expr::Repeat(repeat) => {
            let byte = parse_byte_element(&repeat.expr)?;
            let length = parse_repeat_length(&repeat.len)?;

            if length > isize::MAX as usize {
                return Err(syn::Error::new_spanned(
                    &repeat.len,
                    "repeat length exceeds the maximum allocatable size",
                ));
            }

            Ok(vec![byte; length])
        }

        _ => Err(syn::Error::new_spanned(
            expr,
            "hash! expects a string literal, byte string literal, byte array literal, or byte repeat expression",
        )),
    }
}

fn parse_byte_array(array: &ExprArray) -> syn::Result<Vec<u8>> {
    let mut output = Vec::with_capacity(array.elems.len());

    for element in &array.elems {
        output.push(parse_byte_element(element)?);
    }

    Ok(output)
}

fn parse_byte_element(element: &Expr) -> syn::Result<u8> {
    match element {
        Expr::Group(group) => parse_byte_element(&group.expr),

        Expr::Paren(paren) => parse_byte_element(&paren.expr),

        Expr::Lit(ExprLit {
            lit: Lit::Byte(byte),
            ..
        }) => parse_byte(byte),

        Expr::Lit(ExprLit {
            lit: Lit::Int(integer),
            ..
        }) => parse_u8(integer),

        _ => Err(syn::Error::new_spanned(
            element,
            "byte arrays must contain only u8 literal elements",
        )),
    }
}

fn parse_repeat_length(expr: &Expr) -> syn::Result<usize> {
    match expr {
        Expr::Group(group) => parse_repeat_length(&group.expr),

        Expr::Paren(paren) => parse_repeat_length(&paren.expr),

        Expr::Lit(ExprLit {
            lit: Lit::Int(integer),
            ..
        }) => {
            let suffix = integer.suffix();

            if !suffix.is_empty() && suffix != "usize" {
                return Err(syn::Error::new_spanned(
                    integer,
                    format!("expected a usize literal, found suffix `{suffix}`"),
                ));
            }

            integer.base10_parse::<usize>()
        }

        _ => Err(syn::Error::new_spanned(
            expr,
            "repeat length must be an integer literal",
        )),
    }
}

fn parse_byte(byte: &LitByte) -> syn::Result<u8> {
    reject_suffix(byte, byte.suffix(), "byte literal")?;

    Ok(byte.value())
}

fn reject_suffix<T: quote::ToTokens>(literal: &T, suffix: &str, kind: &str) -> syn::Result<()> {
    if suffix.is_empty() {
        return Ok(());
    }

    Err(syn::Error::new_spanned(
        literal,
        format!("unexpected suffix `{suffix}` on {kind}"),
    ))
}

fn parse_u8(integer: &LitInt) -> syn::Result<u8> {
    let suffix = integer.suffix();

    if !suffix.is_empty() && suffix != "u8" {
        return Err(syn::Error::new_spanned(
            integer,
            format!("expected a u8 literal, found suffix `{suffix}`"),
        ));
    }

    integer.base10_parse::<u8>()
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
    use syn::parse_quote;

    use super::parse_bytes;

    #[test]
    fn parses_string_literal() {
        let expr = parse_quote!("hello");
        let bytes = parse_bytes(&expr).expect("string literal should parse");
        assert_eq!(bytes, b"hello");
    }

    #[test]
    fn parses_byte_string_literal() {
        let expr = parse_quote!(b"hello");
        let bytes = parse_bytes(&expr).expect("byte string literal should parse");
        assert_eq!(bytes, b"hello");
    }

    #[test]
    fn parses_byte_array_literal() {
        let expr = parse_quote!([0, 1, 2, b'a']);
        let bytes = parse_bytes(&expr).expect("byte array literal should parse");
        assert_eq!(bytes, vec![0, 1, 2, b'a']);
    }

    #[test]
    fn parses_string_escape_sequences() {
        let expr = parse_quote!("a\n\t\\\"\u{263A}");
        let bytes = parse_bytes(&expr).expect("escaped string literal should parse");

        assert_eq!(bytes, "a\n\t\\\"\u{263A}".as_bytes());
    }

    #[test]
    fn parses_raw_string_literal() {
        let expr = parse_quote!(r"a\nb");
        let bytes = parse_bytes(&expr).expect("raw string literal should parse");

        assert_eq!(bytes, br"a\nb");
    }

    #[test]
    fn parses_byte_string_escape_sequences() {
        let expr = parse_quote!(b"\x00\xff\n");
        let bytes = parse_bytes(&expr).expect("escaped byte string literal should parse");

        assert_eq!(bytes, vec![0x00, 0xff, b'\n']);
    }

    #[test]
    fn parses_empty_string_literal() {
        let expr = parse_quote!("");
        let bytes = parse_bytes(&expr).expect("empty string literal should parse");

        assert!(bytes.is_empty());
    }

    #[test]
    fn parses_empty_byte_array() {
        let expr = parse_quote!([]);
        let bytes = parse_bytes(&expr).expect("empty byte array should parse");

        assert!(bytes.is_empty());
    }

    #[test]
    fn parses_non_base10_integer_elements() {
        let expr = parse_quote!([0x41, 0o17, 0b1010]);
        let bytes = parse_bytes(&expr).expect("non-base-10 elements should parse");

        assert_eq!(bytes, vec![0x41, 0o17, 0b1010]);
    }

    #[test]
    fn rejects_non_literal_expression() {
        let expr = parse_quote!(some_value);
        let error = parse_bytes(&expr).expect_err("identifier should be rejected");
        assert!(error.to_string().contains("hash! expects"));
    }

    #[test]
    fn rejects_non_u8_array_element() {
        let expr = parse_quote!([0, 256]);
        assert!(parse_bytes(&expr).is_err());
    }

    #[test]
    fn accepts_u8_suffix() {
        let expr = parse_quote!([65u8]);
        let bytes = parse_bytes(&expr).expect("u8-suffixed literal should parse");
        assert_eq!(bytes, vec![65]);
    }

    #[test]
    fn rejects_non_u8_suffix() {
        let expr = parse_quote!([65u16]);
        let error = parse_bytes(&expr).expect_err("non-u8 suffix should be rejected");
        assert!(error.to_string().contains("suffix"));
    }

    #[test]
    fn parses_group_wrapped_literal() {
        let tokens = TokenStream::from(TokenTree::Group(Group::new(
            Delimiter::None,
            quote::quote!("hello"),
        )));

        let expr: syn::Expr = syn::parse2(tokens).expect("group should parse as an expression");
        let bytes = parse_bytes(&expr).expect("group-wrapped literal should parse");

        assert_eq!(bytes, b"hello");
    }

    #[test]
    fn parses_group_wrapped_array_elements() {
        let one = TokenTree::Group(Group::new(Delimiter::None, quote::quote!(1)));
        let two = TokenTree::Group(Group::new(Delimiter::None, quote::quote!(2u8)));
        let letter = TokenTree::Group(Group::new(Delimiter::None, quote::quote!(b'a')));

        let expr: syn::Expr = syn::parse2(quote::quote!([#one, #two, #letter]))
            .expect("array of groups should parse as an expression");
        let bytes = parse_bytes(&expr).expect("group-wrapped elements should parse");

        assert_eq!(bytes, vec![1, 2, b'a']);
    }

    #[test]
    fn rejects_suffixed_string_literal() {
        let expr: syn::Expr =
            syn::parse_str(r#""x"suffix"#).expect("suffixed string literal should lex");
        let error = parse_bytes(&expr).expect_err("suffixed string literal should be rejected");

        assert!(error.to_string().contains("suffix"));
    }

    #[test]
    fn rejects_suffixed_byte_string_literal() {
        let expr: syn::Expr =
            syn::parse_str(r#"b"x"foo"#).expect("suffixed byte string literal should lex");
        let error =
            parse_bytes(&expr).expect_err("suffixed byte string literal should be rejected");

        assert!(error.to_string().contains("suffix"));
    }

    #[test]
    fn rejects_suffixed_byte_literal_element() {
        let expr: syn::Expr =
            syn::parse_str("[b'a'foo]").expect("suffixed byte literal should lex");
        let error = parse_bytes(&expr).expect_err("suffixed byte literal should be rejected");

        assert!(error.to_string().contains("suffix"));
    }

    #[test]
    fn parses_repeat_expression() {
        let expr = parse_quote!([0u8; 4]);
        let bytes = parse_bytes(&expr).expect("repeat expression should parse");

        assert_eq!(bytes, vec![0, 0, 0, 0]);
    }

    #[test]
    fn parses_repeat_expression_with_byte_literal() {
        let expr = parse_quote!([b'a'; 3]);
        let bytes = parse_bytes(&expr).expect("byte literal repeat expression should parse");

        assert_eq!(bytes, vec![b'a', b'a', b'a']);
    }

    #[test]
    fn parses_repeat_expression_with_usize_length() {
        let expr = parse_quote!([7; 2usize]);
        let bytes = parse_bytes(&expr).expect("usize-suffixed length should parse");

        assert_eq!(bytes, vec![7, 7]);
    }

    #[test]
    fn parses_zero_length_repeat_expression() {
        let expr = parse_quote!([0u8; 0]);
        let bytes = parse_bytes(&expr).expect("zero-length repeat expression should parse");

        assert!(bytes.is_empty());
    }

    #[test]
    fn parses_group_wrapped_repeat_parts() {
        let byte = TokenTree::Group(Group::new(Delimiter::None, quote::quote!(b'z')));
        let length = TokenTree::Group(Group::new(Delimiter::None, quote::quote!(3)));

        let expr: syn::Expr = syn::parse2(quote::quote!([#byte; #length]))
            .expect("repeat expression of groups should parse as an expression");
        let bytes = parse_bytes(&expr).expect("group-wrapped repeat parts should parse");

        assert_eq!(bytes, vec![b'z', b'z', b'z']);
    }

    #[test]
    fn parses_parenthesized_input() {
        let expr = parse_quote!(("hello"));
        let bytes = parse_bytes(&expr).expect("parenthesized literal should parse");

        assert_eq!(bytes, b"hello");
    }

    #[test]
    fn parses_parenthesized_repeat_parts() {
        let expr = parse_quote!([(b'z'); (3)]);
        let bytes = parse_bytes(&expr).expect("parenthesized repeat parts should parse");

        assert_eq!(bytes, vec![b'z', b'z', b'z']);
    }

    #[test]
    fn rejects_unallocatable_repeat_length() {
        let expr = parse_quote!([0u8; 18446744073709551615]);
        let error = parse_bytes(&expr).expect_err("unallocatable length should be rejected");

        assert!(error
            .to_string()
            .contains("exceeds the maximum allocatable size"));
    }

    #[test]
    fn rejects_overflowing_repeat_length() {
        let expr = parse_quote!([0u8; 340282366920938463463374607431768211456]);
        let error = parse_bytes(&expr).expect_err("overflowing length should be rejected");

        assert!(error.to_string().contains("number too large"));
    }

    #[test]
    fn rejects_negative_array_element() {
        let expr = parse_quote!([-1]);
        let error = parse_bytes(&expr).expect_err("negative element should be rejected");

        assert!(error.to_string().contains("u8 literal"));
    }

    #[test]
    fn rejects_repeat_expression_with_non_literal_length() {
        let expr = parse_quote!([0u8; LEN]);
        let error = parse_bytes(&expr).expect_err("non-literal length should be rejected");

        assert!(error
            .to_string()
            .contains("repeat length must be an integer literal"));
    }

    #[test]
    fn rejects_repeat_expression_with_wrongly_suffixed_length() {
        let expr = parse_quote!([0u8; 4u32]);
        let error = parse_bytes(&expr).expect_err("non-usize length suffix should be rejected");

        assert!(error.to_string().contains("suffix"));
    }

    #[test]
    fn rejects_repeat_expression_with_non_u8_element() {
        let expr = parse_quote!([some_value; 4]);
        let error = parse_bytes(&expr).expect_err("non-literal element should be rejected");

        assert!(error.to_string().contains("u8 literal"));
    }
}
