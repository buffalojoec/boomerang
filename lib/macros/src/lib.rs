extern crate proc_macro;

mod entrypoint;
mod parser;

#[proc_macro_attribute]
pub fn main(
    attribute: proc_macro::TokenStream,
    _input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    use quote::ToTokens;
    syn::parse_macro_input!(attribute as entrypoint::SolanaBoomerangEntrypoint)
        .to_token_stream()
        .into()
}

#[proc_macro_attribute]
pub fn test(_: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // This macro itself is a no-op, but must be defined as a proc-macro
    // attribute to be used on a function as the `#[boomerang::test]`
    // attribute.
    //
    // The `#[boomerang::main]` macro will detect this attribute and parse the
    // test configurations from the provided arguments.
    input
}
