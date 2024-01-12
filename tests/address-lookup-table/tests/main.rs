use {
    boomerang::client::{BoomerangClient, BoomerangTestClient, BoomerangTestClientConfig},
    libtest_mimic::Trial,
    solana_program::address_lookup_table,
    solana_sdk::feature_set,
    std::sync::Arc,
    tokio,
};

mod create_lookup_table;

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
                    let client = BoomerangClient::new(&config).await;
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
            let config = Arc::new(BoomerangTestClientConfig {
                features_disabled: vec![],
                program_file: program_file.to_string(),
                program_id: address_lookup_table::program::id(),
                ..BoomerangTestClientConfig::default()
            });

            let config_feature_disabled = Arc::new(BoomerangTestClientConfig {
                features_disabled: vec![
                    feature_set::relax_authority_signer_check_for_lookup_table_creation::id(),
                ],
                program_file: program_file.to_string(),
                program_id: address_lookup_table::program::id(),
                ..BoomerangTestClientConfig::default()
            });

            vec![
                async_trial!(
                    create_lookup_table::test_create_lookup_table_idempotent,
                    config
                ),
                async_trial!(
                    create_lookup_table::test_create_lookup_table_not_idempotent,
                    config_feature_disabled
                ),
                async_trial!(
                    create_lookup_table::test_create_lookup_table_use_payer_as_authority,
                    config
                ),
                async_trial!(
                    create_lookup_table::test_create_lookup_table_missing_signer,
                    config_feature_disabled
                ),
                async_trial!(
                    create_lookup_table::test_create_lookup_table_not_recent_slot,
                    config
                ),
                async_trial!(
                    create_lookup_table::test_create_lookup_table_pda_mismatch,
                    config
                ),
            ]
        })
        .collect()
}

#[tokio::main]
async fn main() {
    libtest_mimic::run(&libtest_mimic::Arguments::from_args(), tests()).exit();
}
