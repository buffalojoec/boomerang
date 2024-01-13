use {
    crate::{dirs, program::BoomerangProgramTest},
    libtest_mimic::Trial,
    solana_boomerang_test_validator::{
        start_options::{AddressOrKeypair, BoomerangTestValidatorStartOptions},
        BoomerangTestValidator,
    },
    solana_sdk::{pubkey::Pubkey, signature::Keypair},
    std::path::PathBuf,
};

fn create_local_temp_dir() {
    let local_temp_dir_path = get_local_temp_dir_path();
    std::fs::create_dir_all(local_temp_dir_path).unwrap();
}

fn get_local_temp_dir_path() -> String {
    dirs::workspace_root()
        .unwrap()
        .join("tmp")
        .to_str()
        .unwrap()
        .to_string()
}

fn get_program_so_path(program_name: &str) -> PathBuf {
    dirs::workspace_root()
        .unwrap()
        .join("target")
        .join("deploy")
        .join(format!("{}.so", program_name))
}

fn get_program_keypair_path(program_name: &str) -> PathBuf {
    dirs::workspace_root()
        .unwrap()
        .join("target")
        .join("deploy")
        .join(format!("{}-keypair.json", program_name))
}

fn get_upgrade_authority_keypair_path(program_name: &str) -> PathBuf {
    dirs::workspace_root()
        .unwrap()
        .join("target")
        .join("deploy")
        .join(format!("{}-upgrade-authority-keypair.json", program_name))
}

fn get_ledger_path() -> PathBuf {
    dirs::workspace_root().unwrap().join("test-ledger")
}

pub struct BoomerangIntegrationTest {
    harness: BoomerangProgramTest,
    test_validator_start_options: Vec<BoomerangTestValidatorStartOptions>,
}
impl BoomerangIntegrationTest {
    pub fn new<P>(program_files: &[&str], program_id: &Pubkey, tests: &[P]) -> Self
    where
        P: Fn(String, Pubkey, bool) -> Trial,
    {
        create_local_temp_dir();

        let upgradeable_bpf_programs = program_files
            .iter()
            .map(|program_file| {
                // Set up the local temp directory to store keypairs
                let program_keypair = Keypair::new();
                let upgrade_authority_keypair = Keypair::new();

                // Write the keypairs to the temporary directory
                let program_keypair_path = get_program_keypair_path(program_file);
                let upgrade_authority_keypair_path =
                    get_upgrade_authority_keypair_path(program_file);
                dirs::write_keypair_to_path(&program_keypair, &program_keypair_path).unwrap();
                dirs::write_keypair_to_path(
                    &upgrade_authority_keypair,
                    &upgrade_authority_keypair_path,
                )
                .unwrap();

                BoomerangTestValidatorStartOptions::UpgradeableProgram {
                    address_or_keypair: AddressOrKeypair::Keypair(program_keypair_path),
                    so_file_path: get_program_so_path(program_file),
                    upgrade_authority: AddressOrKeypair::Keypair(upgrade_authority_keypair_path),
                }
            })
            .collect::<Vec<_>>();

        // TODO: Add other options here from test configurations
        let test_validator_start_options = upgradeable_bpf_programs;

        let harness = BoomerangProgramTest::new_with_rpc(program_files, program_id, tests);

        Self {
            harness,
            test_validator_start_options,
        }
    }

    pub fn run(self) {
        let ledger_path = get_ledger_path();
        for iteration in self.harness.get_iterations() {
            println!("Running integrations tests for {}", iteration.program_file);

            // Start the test validator
            let test_validator = BoomerangTestValidator::new(
                ledger_path.clone(),
                "solana".to_string(),
                "solana-test-validator".to_string(),
                &self.test_validator_start_options,
            );
            test_validator.solana_test_validator_teardown();
            test_validator.solana_test_validator_start();

            // Run the tests
            iteration.run();

            // Tear down the test validator
            test_validator.solana_test_validator_teardown();
        }
    }
}
