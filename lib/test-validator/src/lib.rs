pub mod config_file;
pub mod start_options;

use {
    start_options::BoomerangTestValidatorStartOptions,
    std::{path::PathBuf, process::Stdio},
};

fn run_command(command: &str) {
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .status()
        .expect("failed to execute process");
    assert!(status.success());
}

fn run_command_detached(command: &str) {
    std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::null())
        .spawn()
        .expect("failed to execute process");
}

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
        start_options: &[BoomerangTestValidatorStartOptions],
    ) -> Self {
        Self {
            ledger_path,
            solana_cli_alias,
            solana_test_validator_alias,
            test_validator_start_options: BoomerangTestValidatorStartOptions::args_to_string(
                start_options,
            ),
        }
    }

    /// Activate a feature on the test validator
    pub fn solana_feature_activate(&self, feature_keypair_path: &str) {
        let command = format!(
            "{} feature activate {} development",
            self.solana_cli_alias, feature_keypair_path,
        );
        run_command(&command)
    }

    /// Start the test validator
    pub fn solana_test_validator_start(&self) {
        println!("Starting test validator");
        let command = format!(
            "{} {}",
            self.solana_test_validator_alias, self.test_validator_start_options,
        );
        run_command_detached(&command);
        std::thread::sleep(std::time::Duration::from_secs(5));
    }

    /// Tear down the test validator
    pub fn solana_test_validator_teardown(&self) {
        let command = format!("rm -rf {}", self.ledger_path.to_str().unwrap());
        run_command(&command)
    }
}
