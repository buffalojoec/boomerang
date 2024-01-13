use {
    crate::{
        dirs,
        program::{map_iteration, BoomerangProgramTestIteration},
        validator_options::IntoTestValidatorStartOptions,
    },
    libtest_mimic::Trial,
    solana_boomerang_client::BoomerangTestClientConfig,
    solana_boomerang_test_validator::{
        start_options::{AddressOrKeypair, BoomerangTestValidatorStartOptions},
        BoomerangTestValidator,
    },
    solana_sdk::pubkey::Pubkey,
    std::path::PathBuf,
};

fn get_program_so_path(program_name: &str) -> PathBuf {
    dirs::workspace_root()
        .unwrap()
        .join("target")
        .join("deploy")
        .join(format!("{}.so", program_name))
}

fn get_ledger_path() -> PathBuf {
    dirs::workspace_root().unwrap().join("test-ledger")
}

pub struct BoomerangIntegrationTest {
    iterations: Vec<BoomerangProgramTestIteration>,
    test_validator_start_options: Vec<BoomerangTestValidatorStartOptions>,
}
impl BoomerangIntegrationTest {
    pub fn new<P>(programs: &[(&str, &Pubkey)], tests: &[(BoomerangTestClientConfig, &[P])]) -> Self
    where
        P: Fn(BoomerangTestClientConfig, bool) -> Trial,
    {
        let mut iterations = vec![];

        let mut upgradeable_bpf_programs = vec![];

        for program in programs {
            let (program_file, integration_test_program_id) = program;

            // Store the test iteration
            iterations.push(map_iteration(
                program_file,
                tests,
                /* use_banks */ false,
            ));

            // Add the upgradeable program to the startup options
            upgradeable_bpf_programs.push(BoomerangTestValidatorStartOptions::UpgradeableProgram {
                address_or_keypair: AddressOrKeypair::Address(**integration_test_program_id),
                so_file_path: get_program_so_path(program_file),
                upgrade_authority: AddressOrKeypair::Address(**integration_test_program_id), // For now
            });
        }

        // TODO: Add other options here from test configurations
        let test_validator_start_options = upgradeable_bpf_programs;

        Self {
            iterations,
            test_validator_start_options,
        }
    }

    pub fn run(self) {
        let ledger_path = get_ledger_path();
        for iteration in self.iterations {
            println!(
                "Running integrations tests for {}",
                iteration.program_file()
            );

            for chunk in iteration.chunks() {
                // Start the test validator
                let test_validator = BoomerangTestValidator::new(
                    ledger_path.clone(),
                    "solana".to_string(),
                    "solana-test-validator".to_string(),
                    &[
                        &self.test_validator_start_options,
                        // Build out any additional startup options
                        &chunk.config().to_test_validator_start_options(),
                    ],
                );
                test_validator.solana_test_validator_teardown();
                test_validator.solana_test_validator_start();

                // Run the tests
                chunk.run();

                // Tear down the test validator
                test_validator.solana_test_validator_teardown();
            }
        }
    }
}
