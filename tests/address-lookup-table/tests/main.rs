#![cfg(feature = "test-sbf")]

mod create_lookup_table;

use solana_boomerang::boomerang;

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
