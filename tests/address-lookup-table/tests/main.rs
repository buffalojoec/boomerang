mod create_lookup_table;

use {
    create_lookup_table::TEST_RECENT_SLOT,
    libtest_mimic::Trial,
    solana_boomerang::client::{BoomerangClient, BoomerangTestClientConfig},
    solana_program::address_lookup_table,
    solana_sdk::feature_set,
    std::sync::Arc,
    tokio,
};

const PROGRAM_IMPLEMENTATIONS: &[&str] = &[
    "solana_address_lookup_table_program",
    // More program implementations...
];

macro_rules! async_trial {
    ($test_func:path, $client_config:expr) => {{
        let config = $client_config.clone();
        Trial::test(stringify!($test_func), move || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    let client = BoomerangClient::new(&config, /* use_banks */ true).await;
                    $test_func(client).await
                });
            Ok(())
        })
    }};
}

fn tests() -> Vec<Trial> {
    PROGRAM_IMPLEMENTATIONS
        .iter()
        .flat_map(|program_file| {
            let config1 = Arc::new(BoomerangTestClientConfig {
                advance_slot_hashes: vec![TEST_RECENT_SLOT],
                features_disabled: vec![],
                program_file: program_file.to_string(),
                program_id: address_lookup_table::program::id(),
                ..BoomerangTestClientConfig::default()
            });
            let config2 = Arc::new(BoomerangTestClientConfig {
                advance_slot_hashes: vec![TEST_RECENT_SLOT],
                features_disabled: vec![
                    feature_set::relax_authority_signer_check_for_lookup_table_creation::id(),
                ],
                program_file: program_file.to_string(),
                program_id: address_lookup_table::program::id(),
                ..BoomerangTestClientConfig::default()
            });
            let config3 = Arc::new(BoomerangTestClientConfig {
                advance_slot_hashes: vec![TEST_RECENT_SLOT],
                features_disabled: vec![],
                program_file: program_file.to_string(),
                program_id: address_lookup_table::program::id(),
                ..BoomerangTestClientConfig::default()
            });
            let config4 = Arc::new(BoomerangTestClientConfig {
                features_disabled: vec![
                    feature_set::relax_authority_signer_check_for_lookup_table_creation::id(),
                ],
                program_file: program_file.to_string(),
                program_id: address_lookup_table::program::id(),
                ..BoomerangTestClientConfig::default()
            });
            let config5 = Arc::new(BoomerangTestClientConfig {
                features_disabled: vec![],
                program_file: program_file.to_string(),
                program_id: address_lookup_table::program::id(),
                ..BoomerangTestClientConfig::default()
            });
            let config6 = Arc::new(BoomerangTestClientConfig {
                advance_slot_hashes: vec![TEST_RECENT_SLOT],
                features_disabled: vec![],
                program_file: program_file.to_string(),
                program_id: address_lookup_table::program::id(),
                ..BoomerangTestClientConfig::default()
            });

            vec![
                async_trial!(
                    create_lookup_table::test_create_lookup_table_idempotent,
                    config1
                ),
                async_trial!(
                    create_lookup_table::test_create_lookup_table_not_idempotent,
                    config2
                ),
                async_trial!(
                    create_lookup_table::test_create_lookup_table_use_payer_as_authority,
                    config3
                ),
                async_trial!(
                    create_lookup_table::test_create_lookup_table_missing_signer,
                    config4
                ),
                async_trial!(
                    create_lookup_table::test_create_lookup_table_not_recent_slot,
                    config5
                ),
                async_trial!(
                    create_lookup_table::test_create_lookup_table_pda_mismatch,
                    config6
                ),
            ]
        })
        .collect()
}

#[tokio::main]
async fn main() {
    libtest_mimic::run(&libtest_mimic::Arguments::from_args(), tests()).exit();
}
