use proc_macro::{self, TokenStream};
use syn::parse_macro_input;

mod expand;

#[proc_macro_derive(TryFromRow)]
pub fn derive_try_from_row(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);

    expand::try_from_row(input)
        .unwrap_or_else(|error| error.to_compile_error())
        .into()
}
