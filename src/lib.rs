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
    dbg!(&input);
    let mut iter = input.into_iter();
    let token = iter.next().expect("Except token to charize!");

    debug_assert!(iter.next().is_none(), "Not support multiple characters");
    let ch = match token {
        TokenTree::Ident(ident) => {
            let ident = ident.to_string();
            let mut chars = ident.chars();
            let ch = chars.next().unwrap();

            debug_assert_eq!(chars.count(), 0, "Charize only support one character");
            ch
        }
        TokenTree::Punct(punct) => punct.as_char(),
        TokenTree::Literal(lit) => {
            let lit = lit.to_string();
            let mut chars = lit.chars();
            let ch = chars.next().unwrap();

            debug_assert_eq!(chars.count(), 0, "Charize only support one character");
            ch
        }
        _ => {
            panic!("Charize only support Ident or Punct")
        }
    };

    let mut token_stream = TokenStream::new();
    let tree = TokenTree::Literal(Literal::character(ch));

    token_stream.extend([tree]);
    token_stream
}
