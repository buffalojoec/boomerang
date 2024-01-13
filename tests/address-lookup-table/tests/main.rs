mod create_lookup_table;

use {
    create_lookup_table::TEST_RECENT_SLOT,
    libtest_mimic::Trial,
    solana_boomerang::client::{BoomerangClient, BoomerangTestClientConfig},
    solana_program::address_lookup_table,
    solana_sdk::{feature_set, pubkey::Pubkey},
    tokio,
};

const PROGRAM_IMPLEMENTATIONS: &[&str] = &[
    "solana_address_lookup_table_program",
    "solana_address_lookup_table_program",
    // More program implementations...
];

macro_rules! async_trial {
    ($test_func:path) => {{
        |program_file: String, program_id: Pubkey, use_banks: bool| {
            Trial::test(stringify!($test_func), move || {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(async {
                        let client = BoomerangClient::new(
                            &BoomerangTestClientConfig {
                                program_file,
                                program_id,
                                ..BoomerangTestClientConfig::default()
                            },
                            use_banks,
                        )
                        .await;
                        $test_func(client).await
                    });
                Ok(())
            })
        }
    }};
}

macro_rules! async_trial_with_advanced_slot_hashes {
    ($test_func:path) => {{
        |program_file: String, program_id: Pubkey, use_banks: bool| {
            Trial::test(stringify!($test_func), move || {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(async {
                        let client = BoomerangClient::new(
                            &BoomerangTestClientConfig {
                                advance_slot_hashes: vec![TEST_RECENT_SLOT],
                                program_file,
                                program_id,
                                ..BoomerangTestClientConfig::default()
                            },
                            use_banks,
                        )
                        .await;
                        $test_func(client).await
                    });
                Ok(())
            })
        }
    }};
}

macro_rules! async_trial_with_features_disabled {
    ($test_func:path) => {{
        |program_file: String, program_id: Pubkey, use_banks: bool| Trial::test(stringify!($test_func), move || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    let client = BoomerangClient::new(&BoomerangTestClientConfig {
                        features_disabled: vec![
                            feature_set::relax_authority_signer_check_for_lookup_table_creation::id(),
                        ],
                        program_file,
                        program_id,
                        ..BoomerangTestClientConfig::default()
                    }, use_banks).await;
                    $test_func(client).await
                });
            Ok(())
        })
    }};
}

macro_rules! async_trial_with_advanced_slot_hashes_and_features_disabled {
    ($test_func:path) => {{
        |program_file: String, program_id: Pubkey, use_banks: bool| Trial::test(stringify!($test_func), move || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    let client = BoomerangClient::new(&BoomerangTestClientConfig {
                        advance_slot_hashes: vec![TEST_RECENT_SLOT],
                        features_disabled: vec![
                            feature_set::relax_authority_signer_check_for_lookup_table_creation::id(),
                        ],
                        program_file,
                        program_id,
                        ..BoomerangTestClientConfig::default()
                    }, use_banks).await;
                    $test_func(client).await
                });
            Ok(())
        })
    }};
}

fn test_1(program_file: String, program_id: Pubkey, use_banks: bool) -> Trial {
    async_trial_with_advanced_slot_hashes!(create_lookup_table::test_create_lookup_table_idempotent)(
        program_file,
        program_id,
        use_banks,
    )
}

fn test_2(program_file: String, program_id: Pubkey, use_banks: bool) -> Trial {
    async_trial_with_advanced_slot_hashes_and_features_disabled!(
        create_lookup_table::test_create_lookup_table_not_idempotent
    )(program_file, program_id, use_banks)
}

fn test_3(program_file: String, program_id: Pubkey, use_banks: bool) -> Trial {
    async_trial_with_advanced_slot_hashes!(
        create_lookup_table::test_create_lookup_table_use_payer_as_authority
    )(program_file, program_id, use_banks)
}

fn test_4(program_file: String, program_id: Pubkey, use_banks: bool) -> Trial {
    async_trial_with_features_disabled!(
        create_lookup_table::test_create_lookup_table_missing_signer
    )(program_file, program_id, use_banks)
}

fn test_5(program_file: String, program_id: Pubkey, use_banks: bool) -> Trial {
    async_trial!(create_lookup_table::test_create_lookup_table_not_recent_slot)(
        program_file,
        program_id,
        use_banks,
    )
}

fn test_6(program_file: String, program_id: Pubkey, use_banks: bool) -> Trial {
    async_trial_with_advanced_slot_hashes!(
        create_lookup_table::test_create_lookup_table_pda_mismatch
    )(program_file, program_id, use_banks)
}

#[tokio::main]
async fn main() {
    let program_files = PROGRAM_IMPLEMENTATIONS;
    let program_id = address_lookup_table::program::id();

    let tests = vec![test_1, test_2, test_3, test_4, test_5, test_6];

    solana_boomerang::entrypoint(program_files, &program_id, &tests).await;
}
