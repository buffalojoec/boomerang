pub mod config_file;
pub mod start_options;

use {start_options::BoomerangTestValidatorStartOptions, std::process::Stdio};

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
    solana_cli_alias: String,
    solana_test_validator_alias: String,
    test_validator_start_options: String,
}
impl Default for BoomerangTestValidator {
    fn default() -> Self {
        Self {
            solana_cli_alias: "solana".to_string(),
            solana_test_validator_alias: "solana-test-validator".to_string(),
            test_validator_start_options: "".to_string(),
        }
    }
}
impl BoomerangTestValidator {
    pub fn new(
        solana_cli_alias: String,
        solana_test_validator_alias: String,
        test_validator_start_options: Vec<BoomerangTestValidatorStartOptions>,
    ) -> Self {
        Self {
            solana_cli_alias,
            solana_test_validator_alias,
            test_validator_start_options: BoomerangTestValidatorStartOptions::args_to_string(
                test_validator_start_options,
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
        let command = format!(
            "{} {}",
            self.solana_test_validator_alias, self.test_validator_start_options,
        );
        run_command_detached(&command)
    }
}
