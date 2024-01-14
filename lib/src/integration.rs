use {
    crate::{
        dirs,
        program::{BoomerangProgramTest, BoomerangProgramTestIteration},
        validator_options::IntoTestValidatorStartOptions,
        BoomerangTests,
    },
    solana_boomerang_test_validator::BoomerangTestValidator,
};

pub struct BoomerangIntegrationTest {
    iterations: Vec<BoomerangProgramTestIteration>,
    solana_cli_alias: String,
    solana_test_validator_alias: String,
}
impl Default for BoomerangIntegrationTest {
    fn default() -> Self {
        Self {
            iterations: Vec::new(),
            solana_cli_alias: "solana".to_string(),
            solana_test_validator_alias: "solana-test-validator".to_string(),
        }
    }
}

impl BoomerangIntegrationTest {
    pub fn new(programs: &[(&str, &str)], tests: BoomerangTests<'_>) -> Self {
        Self {
            iterations: BoomerangProgramTest::build_program_test_iterations(
                programs, tests, /* use_banks */ false,
            ),
            ..Self::default()
        }
    }

    pub fn run(self) {
        for iteration in self.iterations {
            println!(
                "Running integrations tests for {}",
                iteration.program_file()
            );

            for chunk in iteration.chunks() {
                let test_validator = BoomerangTestValidator::new(
                    dirs::test_ledger_path(),
                    &self.solana_cli_alias,
                    &self.solana_test_validator_alias,
                    &[&chunk.config().to_test_validator_start_options()],
                );
                test_validator.solana_test_validator_teardown();
                test_validator.solana_test_validator_start();

                chunk.run();

                test_validator.solana_test_validator_teardown();
            }
        }
    }
}
