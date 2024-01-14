pub mod commands;
pub mod start_options;

use {
    commands::{run_command, run_command_detached, run_command_with_num_retries},
    start_options::BoomerangTestValidatorStartOptions,
    std::path::PathBuf,
};

pub struct BoomerangTestValidator {
    ledger_path: PathBuf,
    solana_cli_alias: String,
    solana_test_validator_alias: String,
    test_validator_start_options: String,
}
impl BoomerangTestValidator {
    pub fn new(
        ledger_path: PathBuf,
        solana_cli_alias: String,
        solana_test_validator_alias: String,
        start_options: &[&[BoomerangTestValidatorStartOptions]],
    ) -> Self {
        let mut test_validator_start_options = String::new();

        start_options.iter().for_each(|options| {
            test_validator_start_options.push_str(&format!(
                " {}",
                BoomerangTestValidatorStartOptions::args_to_string(options).as_str()
            ));
        });

        test_validator_start_options
            .push_str(format!(" --ledger {}", ledger_path.to_str().unwrap()).as_str());

        Self {
            ledger_path,
            solana_cli_alias,
            solana_test_validator_alias,
            test_validator_start_options,
        }
    }

    pub fn solana_feature_activate(&self, feature_keypair_path: &str) {
        println!("Activating feature: {}", feature_keypair_path);
        let command = format!(
            "{} feature activate {} development",
            self.solana_cli_alias, feature_keypair_path,
        );
        run_command(&command)
    }

    pub fn solana_test_validator_start(&self) {
        println!("Starting test validator");
        println!("Ledger path: {:?}", self.ledger_path);
        let command = format!(
            "{} {}",
            self.solana_test_validator_alias, self.test_validator_start_options,
        );
        run_command_detached(&command);
        std::thread::sleep(std::time::Duration::from_secs(5));
    }

    pub fn solana_test_validator_teardown(&self) {
        println!("Tearing down test validator");
        println!("Ledger path: {:?}", self.ledger_path);
        let command = format!("rm -rf {}", self.ledger_path.to_str().unwrap());
        run_command_with_num_retries(&command, 3);
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
