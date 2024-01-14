mod dirs;
pub mod integration;
pub mod migration;
pub mod program;
pub mod validator_options;

use {
    client::BoomerangTestClientConfig, integration::BoomerangIntegrationTest, libtest_mimic::Trial,
    program::BoomerangProgramTest,
};
pub use {
    libtest_mimic, solana_boomerang_client as client, solana_boomerang_macros as boomerang,
    solana_boomerang_test_validator as test_validator, tokio,
};

fn select_test_programs<'a>(
    programs: &'a [(&'a str, &'a str)],
    program_names: &'a [&'a str],
) -> Vec<(&'a str, &'a str)> {
    programs
        .iter()
        .filter(|(program_name, _)| program_names.contains(program_name))
        .map(|(program_name, program_id)| (*program_name, *program_id))
        .collect::<Vec<_>>()
}

#[macro_export]
macro_rules! boomerang_trial {
    ($test_func:path) => {{
        |config: solana_boomerang::client::BoomerangTestClientConfig, use_banks: bool| {
            solana_boomerang::libtest_mimic::Trial::test(stringify!($test_func), move || {
                solana_boomerang::tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(async {
                        let client =
                            solana_boomerang::client::BoomerangClient::new(&config, use_banks)
                                .await;
                        $test_func(client).await
                    });
                Ok(())
            })
        }
    }};
}

pub type BoomerangTestFn = fn(BoomerangTestClientConfig, bool) -> Trial;
pub type BoomerangTest<'a> = (BoomerangTestClientConfig, &'a [BoomerangTestFn]);
pub type BoomerangTests<'a> = &'a [BoomerangTest<'a>];

pub async fn entrypoint(
    programs: &[(&str, &str)],
    program_tests: &[&str],
    integration_tests: &[&str],
    migration_tests: &[(&str, &str)],
    tests: BoomerangTests<'_>,
) {
    if program_tests.is_empty() && integration_tests.is_empty() && migration_tests.is_empty() {
        println!("No tests to run");
        return;
    }

    if !program_tests.is_empty() {
        let programs = select_test_programs(programs, program_tests);
        let program_test = BoomerangProgramTest::new(&programs, tests);
        program_test.run();
    }

    if !integration_tests.is_empty() {
        let programs = select_test_programs(programs, integration_tests);
        let integration_test = BoomerangIntegrationTest::new(&programs, tests);
        integration_test.run();
    }

    if !migration_tests.is_empty() {
        // Run the migration tests
    }
}
