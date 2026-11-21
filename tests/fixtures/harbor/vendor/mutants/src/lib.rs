//! The attribute that tells mutation testing to leave a function alone.

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn skip(_arguments: TokenStream, item: TokenStream) -> TokenStream {
    item
}
