use std::path::Path;

mod setup;

use {
    crate::{
        dirs, program::BoomerangProgramTestIteration,
        validator_options::IntoTestValidatorStartOptions, BoomerangTests,
    },
    solana_boomerang_client::{BoomerangClient, BoomerangTestClient},
    solana_boomerang_test_validator::{commands::run_command, BoomerangTestValidator},
    solana_sdk::{pubkey::Pubkey, signer::Signer},
    std::str::FromStr,
};

const SLOTS_PER_EPOCH: u64 = 120;

pub struct BoomerangMigrationTest {
    migrations: Vec<(
        BoomerangProgramTestIteration,
        String, // Target program
    )>,
    solana_cli_alias: String,
    solana_test_validator_alias: String,
}

impl BoomerangMigrationTest {
    pub async fn new(migrations: &[(&str, &str, &str)], tests: BoomerangTests<'_>) -> Self {
        Self {
            migrations: migrations
                .iter()
                .map(|(program_file, program_id, target_program)| {
                    let iteration = BoomerangProgramTestIteration::new(
                        &(program_file, program_id),
                        tests,
                        /* use_banks */ false,
                    );
                    (iteration, target_program.to_string())
                })
                .collect(),
            solana_cli_alias: dirs::solana_cli_path_string(),
            solana_test_validator_alias: dirs::solana_test_validator_path_string(),
        }
    }

    async fn activate_feature_and_poll_for_activation(
        solana_cli_alias: &str,
        client: &BoomerangClient,
        feature_keypair_path: &Path,
    ) {
        run_command(&format!(
            "{} feature activate {} development",
            solana_cli_alias,
            feature_keypair_path.to_str().unwrap(),
        ));
        client.poll_for_next_epoch().await.unwrap();
        client.poll_slots(5).await.unwrap();
    }

    pub async fn run(self) {
        for migration in self.migrations {
            let (iteration, target_program) = migration;
            let (feature_keypair, feature_keypair_path) = setup::setup(&target_program);
            let feature_id = feature_keypair.pubkey().to_string();

            println!(
                "Running migration tests for {} replacing {}",
                iteration.program_file(),
                target_program,
            );

            for chunk in iteration.chunks() {
                let mut config = chunk.config().clone();
                config
                    .features_disabled
                    .push(Pubkey::from_str(&feature_id).unwrap());
                config.slots_per_epoch = SLOTS_PER_EPOCH;

                let test_validator = BoomerangTestValidator::new(
                    dirs::test_ledger_path(),
                    &self.solana_cli_alias,
                    &self.solana_test_validator_alias,
                    &[&config.to_test_validator_start_options()],
                );
                test_validator.solana_test_validator_teardown();
                test_validator.solana_test_validator_start();

                Self::activate_feature_and_poll_for_activation(
                    &self.solana_cli_alias,
                    &BoomerangClient::new(&config, /* use_banks */ false).await,
                    &feature_keypair_path,
                )
                .await;

                chunk.run();

                test_validator.solana_test_validator_teardown();
            }
        }
    }
}
