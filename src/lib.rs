//! # charize
//!
//! Provide a simple proc macro `charize` convert token to char.
//!
//! ## Usage
//!
//! ```rust
//! # pub fn main() {
//! assert_eq!('o', charize::charize!(o));
//! assert_eq!('1', charize::charize!(1));
//! assert_eq!('我', charize::charize!(我));
//! assert_eq!('P', charize::charize!(P));
//! assert_eq!('@', charize::charize!(@));
//! assert_eq!('&', charize::charize!(&));
//! # }
//! ```
//!
//! ## LICENSE
//!
//! MPL-2.0
//!
use proc_macro::Literal;
use proc_macro::TokenStream;
use proc_macro::TokenTree;

#[proc_macro]
pub fn charize(input: TokenStream) -> TokenStream {
    let mut ch = None;
    let mut iter = input.into_iter();

    while ch.is_none() {
        if let Some(token) = iter.next() {
            debug_assert!(
                iter.next().is_none(),
                "Charize not support multiple characters"
            );
            match &token {
                TokenTree::Ident(ident) => {
                    let ident = ident.to_string();
                    let mut chars = ident.chars();

                    ch = chars.next();
                    debug_assert_eq!(chars.count(), 0, "Charize only support one character");
                }
                TokenTree::Punct(punct) => {
                    ch = Some(punct.as_char());
                }
                TokenTree::Literal(lit) => {
                    let lit = lit.to_string();
                    let mut chars = lit.chars();

                    ch = chars.next();
                    debug_assert_eq!(chars.count(), 0, "Charize only support one character");
                }
                TokenTree::Group(group) => {
                    iter = group.stream().into_iter();
                }
            }
        }
    }

    let mut token = TokenStream::new();
    let value = ch.expect("Charize expect token to charize!");
    let tree = TokenTree::Literal(Literal::character(value));

    token.extend([tree]);
    token
}
