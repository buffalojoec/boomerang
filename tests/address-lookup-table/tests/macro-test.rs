mod create_lookup_table;

use solana_boomerang::boomerang;

#[boomerang::test(
    features_disabled = [
        feature_set::relax_authority_signer_check_for_lookup_table_creation::id,
    ],
    warp_slot = 150,
)]
fn _this_is_a_test() {
    println!("This is a test");
}

#[boomerang::test]
fn _this_is_another_test() {
    println!("This is another test");
}

#[boomerang::main(
    programs = [
        (
            "solana_address_lookup_table_program",
            "927eaPZzYLFfox14h7UyaZjGk6yL7RSWjtmFv8dhBUki"
        ),
        (
            "solana_address_lookup_table_zig",
            "927eaPZzYLFfox14h7UyaZjGk6yL7RSWjtmFv8dhBUki"
        ),
    ],
    program_tests = [
        "solana_address_lookup_table_program",
        "solana_address_lookup_table_zig",
    ],
    integration_tests = [
        "solana_address_lookup_table_program",
        "solana_address_lookup_table_zig",
    ],
    migration_tests = [
        (
            "solana_address_lookup_table_program",
            "NativeProgram::AddressLookupTable"
        ),
        (
            "solana_address_lookup_table_zig",
            "NativeProgram::AddressLookupTable"
        ),
    ],
)]
fn main() {}
