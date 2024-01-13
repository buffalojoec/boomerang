pub use {
    solana_boomerang_client as client,
    // solana_boomerang_macros as boomerang,
    solana_boomerang_test_validator as test_validator,
};

// mod compatibility;
// mod integration;
// mod migration;
pub mod program;

// use compatibility::BoomerangCompatibilityTest;
// use integration::BoomerangIntegrationTest;
// use migration::BoomerangMigrationTest;
use program::BoomerangProgramTest;

fn parse_env(variable: &str) -> bool {
    std::env::var(variable).unwrap_or_default() == "true"
}

pub struct Boomerang {
    pub program_tests: Vec<BoomerangProgramTest>,
    // pub compatibility_tests: Vec<BoomerangCompatibilityTest>,
    // pub integration_tests: Vec<BoomerangIntegrationTest>,
    // pub migration_tests: Vec<BoomerangMigrationTest>,
}

pub async fn entrypoint(boomerang: Boomerang) {
    let args = libtest_mimic::Arguments::from_args();

    let integration = parse_env("INTEGRATION");
    let migration = parse_env("MIGRATION");
    let program = parse_env("PROGRAM");

    if !integration && !migration && !program {
        // TODO: Print message
        return;
    }

    if program {
        // Run the program tests
        for program_test in boomerang.program_tests {
            println!(
                "Running program tests for {}",
                program_test.program_implementation
            );
            libtest_mimic::run(&args, program_test.trials).exit_if_failed();
        }
    }

    if integration {
        // Run the integration tests
    }

    if migration {
        // Run the migration tests
    }
}
