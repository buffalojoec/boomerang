use {
    crate::{
        dirs,
        program::{BoomerangProgramTest, BoomerangProgramTestIteration},
        validator_options::IntoTestValidatorStartOptions,
        BoomerangTests,
    },
    solana_boomerang_test_validator::BoomerangTestValidator,
};

const SOLANA_CLI_ALIAS: &str = "solana";
const SOLANA_TEST_VALIDATOR_ALIAS: &str = "solana-test-validator";

pub struct BoomerangIntegrationTest {
    iterations: Vec<BoomerangProgramTestIteration>,
}
impl BoomerangIntegrationTest {
    pub fn new(programs: &[(&str, &str)], tests: BoomerangTests<'_>) -> Self {
        Self {
            iterations: BoomerangProgramTest::build_program_test_iterations(
                programs, tests, /* use_banks */ false,
            ),
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
                    SOLANA_CLI_ALIAS.to_string(),
                    SOLANA_TEST_VALIDATOR_ALIAS.to_string(),
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
