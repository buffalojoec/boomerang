//! The idea here is that this entire file should be replaced by a minimal
//! procedural macro attribute configuration.
//!
//! Perhaps you include the modules above this comment, and then you have a
//! macro attribute that looks something like this:
//!
//! #[boomerang::main(
//!     programs = [
//!         (
//!             "solana_address_lookup_table_program",
//!             "927eaPZzYLFfox14h7UyaZjGk6yL7RSWjtmFv8dhBUki"
//!         ),
//!         (
//!             "solana_address_lookup_table_zig",
//!             "927eaPZzYLFfox14h7UyaZjGk6yL7RSWjtmFv8dhBUki"
//!         ),
//!     ],
//!     program_tests = true,
//!     integration_tests = true,
//!     migration_tests = [
//!         (
//!             "solana_address_lookup_table_program",
//!             NativeProgram::AddressLookupTable
//!         ),
//!         (
//!             "solana_address_lookup_table_zig",
//!             NativeProgram::AddressLookupTable
//!         ),
//!     ]
//! )]
//! async fn main() {}
//!
//! Additionally, for the tests themselves, you could have a macro attribute
//! that looks something like this:
//!
//! #[boomerang::test]
//! #[boomerang_test_config(
//!     deactivate_features = [
//!         feature_set::relax_authority_signer_check_for_lookup_table_creation::id(),
//!     ],
//!     warp_slot = 150,
//! )]
//! async fn test_1(mut client: BoomerangClient) {
//!     /* .. */
//! }

mod create_lookup_table;

use {
    create_lookup_table::TEST_RECENT_SLOT,
    solana_boomerang::{
        boomerang_trial,
        client::{BoomerangClient, BoomerangTestClientConfig},
        libtest_mimic::Trial,
        tokio, BoomerangTests,
    },
    solana_program::address_lookup_table,
    solana_sdk::{feature_set, pubkey::Pubkey},
    std::str::FromStr,
};

fn test_1(config: BoomerangTestClientConfig, use_banks: bool) -> Trial {
    boomerang_trial!(create_lookup_table::test_create_lookup_table_idempotent)(config, use_banks)
}

fn test_2(config: BoomerangTestClientConfig, use_banks: bool) -> Trial {
    boomerang_trial!(create_lookup_table::test_create_lookup_table_not_idempotent)(
        config, use_banks,
    )
}

fn test_3(config: BoomerangTestClientConfig, use_banks: bool) -> Trial {
    boomerang_trial!(create_lookup_table::test_create_lookup_table_use_payer_as_authority)(
        config, use_banks,
    )
}

fn test_4(config: BoomerangTestClientConfig, use_banks: bool) -> Trial {
    boomerang_trial!(create_lookup_table::test_create_lookup_table_missing_signer)(
        config, use_banks,
    )
}

fn test_5(config: BoomerangTestClientConfig, use_banks: bool) -> Trial {
    boomerang_trial!(create_lookup_table::test_create_lookup_table_not_recent_slot)(
        config, use_banks,
    )
}

fn test_6(config: BoomerangTestClientConfig, use_banks: bool) -> Trial {
    boomerang_trial!(create_lookup_table::test_create_lookup_table_pda_mismatch)(config, use_banks)
}

#[tokio::main]
async fn main() {
    let integration_test_program_id =
        Pubkey::from_str("927eaPZzYLFfox14h7UyaZjGk6yL7RSWjtmFv8dhBUki").unwrap();

    let programs = &[
        (
            "solana_address_lookup_table_program",
            &integration_test_program_id,
        ),
        (
            "solana_address_lookup_table_program",
            &integration_test_program_id,
        ),
        // More program implementations...
    ];

    let program_file = "solana_address_lookup_table_program.so".to_string();
    let program_id = address_lookup_table::program::id();

    let config_default = BoomerangTestClientConfig {
        program_file: program_file.clone(),
        program_id,
        ..BoomerangTestClientConfig::default()
    };
    let config_advance_slot_hashes = BoomerangTestClientConfig {
        program_file: program_file.clone(),
        program_id,
        warp_slot: TEST_RECENT_SLOT,
        ..BoomerangTestClientConfig::default()
    };
    let config_disable_feature = BoomerangTestClientConfig {
        features_disabled: vec![
            feature_set::relax_authority_signer_check_for_lookup_table_creation::id(),
        ],
        program_file: program_file.clone(),
        program_id,
        ..BoomerangTestClientConfig::default()
    };
    let config_advance_slot_hashes_and_disable_feature = BoomerangTestClientConfig {
        features_disabled: vec![
            feature_set::relax_authority_signer_check_for_lookup_table_creation::id(),
        ],
        program_file,
        program_id,
        warp_slot: TEST_RECENT_SLOT,
        ..BoomerangTestClientConfig::default()
    };

    let tests: BoomerangTests = &[
        (config_advance_slot_hashes, &[test_1, test_3, test_6]),
        (config_advance_slot_hashes_and_disable_feature, &[test_2]),
        (config_disable_feature, &[test_4]),
        (config_default, &[test_5]),
    ];

    solana_boomerang::entrypoint(programs, tests).await;
}
