extern crate proc_macro;

mod expand;

use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Derive a `Valuable` implementation for a struct or enum.
///
/// # Examples
///
/// ```
/// use valuable::Valuable;
///
/// #[derive(Valuable)]
/// struct HelloWorld {
///     message: Message,
/// }
///
/// #[derive(Valuable)]
/// enum Message {
///     HelloWorld,
///     Custom(String),
/// }
/// ```
#[proc_macro_derive(Valuable, attributes(valuable))]
pub fn derive_valuable(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as syn::DeriveInput);
    expand::derive_valuable(&mut input).into()
}
