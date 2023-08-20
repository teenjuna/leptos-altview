mod body;
mod node;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::body::Body;

#[proc_macro]
pub fn view(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as Body);
    dbg!(input);
    quote!(()).into()
}
