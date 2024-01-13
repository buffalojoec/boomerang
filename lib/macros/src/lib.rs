extern crate proc_macro;

mod entrypoint;
mod trial;

#[proc_macro_attribute]
pub fn main(attr: proc_macro::TokenStream, _: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use quote::ToTokens;
    syn::parse_macro_input!(attr as entrypoint::SolanaBoomerangEntrypoint)
        .to_token_stream()
        .into()
}

#[proc_macro_attribute]
pub fn test(attr: proc_macro::TokenStream, _: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // This macro itself is a no-op, but must be defined as a proc-macro
    // attribute to be used on a function as the `#[boomerang::test]`
    // attribute.
    //
    // The `#[boomerang::main]` macro will detect this attribute and parse the
    // test configurations from the provided arguments.
    // input

    // TODO: Take this out once the test suite parser is done
    use quote::ToTokens;
    syn::parse_macro_input!(attr as trial::SolanaBoomerangTrial)
        .to_token_stream()
        .into()
}
