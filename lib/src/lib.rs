mod dirs;
pub mod integration;
pub mod program;

use {
    integration::BoomerangIntegrationTest, libtest_mimic::Trial, program::BoomerangProgramTest,
    solana_sdk::pubkey::Pubkey,
};
pub use {
    solana_boomerang_client as client,
    // solana_boomerang_macros as boomerang,
    solana_boomerang_test_validator as test_validator,
};

fn parse_env(variable: &str) -> bool {
    std::env::var(variable).unwrap_or_default() == "true"
}

pub async fn entrypoint<P>(
    program_files: &[&str],
    program_id: &Pubkey,
    integration_test_program_id: &Pubkey,
    tests: &[P],
) where
    P: Fn(String, Pubkey, bool) -> Trial,
{
    let integration = parse_env("INTEGRATION");
    let migration = parse_env("MIGRATION");
    let program = parse_env("PROGRAM");

    if !integration && !migration && !program {
        println!("No tests to run");
        return;
    }

    if program {
        // Run the program tests
        let program_test = BoomerangProgramTest::new_with_banks(program_files, program_id, tests);
        program_test.run();
    }

    if integration {
        // Run the integration tests
        let integration_test =
            BoomerangIntegrationTest::new(program_files, integration_test_program_id, tests);
        integration_test.run();
    }

    if migration {
        // Run the migration tests
    }
}
