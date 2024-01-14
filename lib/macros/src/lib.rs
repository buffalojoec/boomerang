extern crate proc_macro;

mod entrypoint;
mod iteration;
mod krate_parser;
mod parser;

/// The `#[boomerang::main]` attribute defines the entrypoint of the program's
/// test suite.
/// * `programs` is a list of tuples of the form `(program_name, program_id)`
///   that declares the different program implementations that can be tested.
/// * `program_tests` is a list of program names that declares which programs
///   should be tested with a `BanksClient` program test.
/// * `integration_tests` is a list of program names that declares which
///   programs should be tested with an `RpcClient` integration test against a
///   local test validator.
/// * `migration_tests` is a list of tuples of the form `(source_program_name,
///   target)` that declares which native program the declared source program
///   should be migration tested against.
///
/// Example:
///
/// ```rust
/// #[boomerang::main(
///     programs = [
///         (
///             "solana_address_lookup_table_rust",
///             "927eaPZzYLFfox14h7UyaZjGk6yL7RSWjtmFv8dhBUki"
///         ),
///         (
///             "solana_address_lookup_table_zig",
///             "4ifTTRistQ33vBBPXEj4qFkNVgyMGPDUoV631PnU5Bcf"
///         ),
///     ],
///     program_tests = [
///         "solana_address_lookup_table_rust",
///         "solana_address_lookup_table_zig",
///     ],
///     integration_tests = [
///         "solana_address_lookup_table_rust",
///         "solana_address_lookup_table_zig",
///     ],
///     migration_tests = [
///         (
///             "solana_address_lookup_table_rust",
///             "NativeProgram::AddressLookupTable"
///         ),
///         (
///             "solana_address_lookup_table_zig",
///             "NativeProgram::AddressLookupTable"
///         ),
///     ],
/// )]
/// async fn main() {}
/// ```
#[proc_macro_attribute]
pub fn main(attr: proc_macro::TokenStream, _: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use quote::ToTokens;
    syn::parse_macro_input!(attr as entrypoint::Entrypoint)
        .to_token_stream()
        .into()
}

/// The `#[boomerang::test]` attribute defines a test case for the program.
/// The attribute accepts arguments for configuring the test case's startup
/// behavior. These startup configs are valid for both a `BanksClient` program
/// test and an `RpcClient` integration/migration test.
/// * `features_disabled` is a list of feature IDs from the Solana SDK's
///   `feature_set` to disable on startup. validator before running the test
///   case.
/// * `warp_slot` is the slot to warp the bank or test validator to before
///   running the test case.
///
/// Example:
///
/// ```rust
/// #[boomerang::test(
///     features_disabled = [
///         solana_sdk::feature_set::relax_authority_signer_check_for_lookup_table_creation::id,
///     ],
///     warp_slot = 123,
/// )]
/// pub async fn test_create_token(mut client: BoomerangClient) {
///     /* .. */
/// }
/// ```
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
