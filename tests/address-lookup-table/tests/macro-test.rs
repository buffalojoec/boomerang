use solana_boomerang::boomerang;

#[boomerang::test]
fn this_is_a_test() {
    println!("This is a test");
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
    program_tests = true,
    integration_tests = false,
    migration_tests = [
        (
            "solana_address_lookup_table_program",
            NativeProgram::AddressLookupTable
        ),
        (
            "solana_address_lookup_table_zig",
            NativeProgram::AddressLookupTable
        ),
    ],
)]
fn main() {}
