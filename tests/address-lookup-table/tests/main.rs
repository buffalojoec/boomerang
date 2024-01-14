#![cfg(feature = "test-sbf")]

mod create_lookup_table;

use solana_boomerang::boomerang;

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
#[boomerang::main(
    programs = [
        (
            "solana_address_lookup_table_program",
            "927eaPZzYLFfox14h7UyaZjGk6yL7RSWjtmFv8dhBUki"
        ),
        (
            "solana_address_lookup_table_program",
            "927eaPZzYLFfox14h7UyaZjGk6yL7RSWjtmFv8dhBUki"
        ),
    ],
    program_tests = [
        "solana_address_lookup_table_program",
        "solana_address_lookup_table_program",
    ],
    integration_tests = [
        "solana_address_lookup_table_program",
        "solana_address_lookup_table_program",
    ],
    migration_tests = [
        (
            "solana_address_lookup_table_program",
            "NativeProgram::AddressLookupTable"
        ),
        (
            "solana_address_lookup_table_program",
            "NativeProgram::AddressLookupTable"
        ),
    ],
)]
async fn main() {}
