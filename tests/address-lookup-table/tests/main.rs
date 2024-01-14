#![cfg(feature = "test-sbf")]

mod create_lookup_table;

// ======================= MACRO GENERATED ======================= //

use solana_boomerang::tokio;

fn boomerang_test_create_lookup_table_idempotent(
    config: solana_boomerang::client::BoomerangTestClientConfig,
    use_banks: bool,
) -> solana_boomerang::libtest_mimic::Trial {
    solana_boomerang::boomerang_trial!(create_lookup_table::test_create_lookup_table_idempotent)(
        config, use_banks,
    )
}

fn boomerang_test_create_lookup_table_not_idempotent(
    config: solana_boomerang::client::BoomerangTestClientConfig,
    use_banks: bool,
) -> solana_boomerang::libtest_mimic::Trial {
    solana_boomerang::boomerang_trial!(create_lookup_table::test_create_lookup_table_not_idempotent)(
        config, use_banks,
    )
}

fn boomerang_test_create_lookup_table_use_payer_as_authority(
    config: solana_boomerang::client::BoomerangTestClientConfig,
    use_banks: bool,
) -> solana_boomerang::libtest_mimic::Trial {
    solana_boomerang::boomerang_trial!(
        create_lookup_table::test_create_lookup_table_use_payer_as_authority
    )(config, use_banks)
}

fn boomerang_test_create_lookup_table_missing_signer(
    config: solana_boomerang::client::BoomerangTestClientConfig,
    use_banks: bool,
) -> solana_boomerang::libtest_mimic::Trial {
    solana_boomerang::boomerang_trial!(create_lookup_table::test_create_lookup_table_missing_signer)(
        config, use_banks,
    )
}

fn boomerang_test_create_lookup_table_not_recent_slot(
    config: solana_boomerang::client::BoomerangTestClientConfig,
    use_banks: bool,
) -> solana_boomerang::libtest_mimic::Trial {
    solana_boomerang::boomerang_trial!(
        create_lookup_table::test_create_lookup_table_not_recent_slot
    )(config, use_banks)
}

fn boomerang_test_create_lookup_table_pda_mismatch(
    config: solana_boomerang::client::BoomerangTestClientConfig,
    use_banks: bool,
) -> solana_boomerang::libtest_mimic::Trial {
    solana_boomerang::boomerang_trial!(create_lookup_table::test_create_lookup_table_pda_mismatch)(
        config, use_banks,
    )
}

#[tokio::main]
async fn main() {
    let programs = &[
        (
            "solana_address_lookup_table_program",
            "927eaPZzYLFfox14h7UyaZjGk6yL7RSWjtmFv8dhBUki",
        ),
        (
            "solana_address_lookup_table_program",
            "927eaPZzYLFfox14h7UyaZjGk6yL7RSWjtmFv8dhBUki",
        ),
        // More program implementations...
    ];

    let program_tests = &[
        "solana_address_lookup_table_program",
        "solana_address_lookup_table_program",
    ];

    let integration_tests = &[
        "solana_address_lookup_table_program",
        "solana_address_lookup_table_program",
    ];

    let migration_tests = &[
        (
            "solana_address_lookup_table_program",
            "NativeProgram::AddressLookupTable",
        ),
        (
            "solana_address_lookup_table_program",
            "NativeProgram::AddressLookupTable",
        ),
    ];

    let tests: solana_boomerang::BoomerangTests = &[
        (
            solana_boomerang::client::BoomerangTestClientConfig {
                warp_slot: 123,
                ..solana_boomerang::client::BoomerangTestClientConfig::default()
            },
            &[
                boomerang_test_create_lookup_table_idempotent,
                boomerang_test_create_lookup_table_use_payer_as_authority,
                boomerang_test_create_lookup_table_pda_mismatch,
            ]
        ),
        (
            solana_boomerang::client::BoomerangTestClientConfig {
                features_disabled: vec![
                    solana_sdk::feature_set::relax_authority_signer_check_for_lookup_table_creation::id(),
                ],
                warp_slot: 123,
                ..solana_boomerang::client::BoomerangTestClientConfig::default()
            },
            &[
                boomerang_test_create_lookup_table_not_idempotent,
            ]
        ),
        (
            solana_boomerang::client::BoomerangTestClientConfig {
                features_disabled: vec![
                    solana_sdk::feature_set::relax_authority_signer_check_for_lookup_table_creation::id(),
                ],
                ..solana_boomerang::client::BoomerangTestClientConfig::default()
            },
            &[
                boomerang_test_create_lookup_table_missing_signer,
            ]
        ),
        (
            solana_boomerang::client::BoomerangTestClientConfig {
                ..solana_boomerang::client::BoomerangTestClientConfig::default()
            },
            &[
                boomerang_test_create_lookup_table_not_recent_slot
            ]
        ),
    ];

    solana_boomerang::entrypoint(
        programs,
        program_tests,
        integration_tests,
        migration_tests,
        tests,
    )
    .await;
}
