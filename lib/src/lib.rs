mod dirs;
pub mod integration;
pub mod program;
mod validator_options;

use {
    client::BoomerangTestClientConfig, integration::BoomerangIntegrationTest, libtest_mimic::Trial,
    program::BoomerangProgramTest, solana_sdk::pubkey::Pubkey,
};
pub use {
    libtest_mimic,
    solana_boomerang_client as client,
    // solana_boomerang_macros as boomerang,
    solana_boomerang_test_validator as test_validator,
    tokio,
};

fn parse_env(variable: &str) -> bool {
    std::env::var(variable).unwrap_or_default() == "true"
}

#[macro_export]
macro_rules! boomerang_trial {
    ($test_func:path) => {{
        |config: BoomerangTestClientConfig, use_banks: bool| {
            Trial::test(stringify!($test_func), move || {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(async {
                        let client = BoomerangClient::new(&config, use_banks).await;
                        $test_func(client).await
                    });
                Ok(())
            })
        }
    }};
}

type BoomerangTestFn = fn(BoomerangTestClientConfig, bool) -> Trial;
pub type BoomerangTests<'a> = &'a [(BoomerangTestClientConfig, &'a [BoomerangTestFn])];

pub async fn entrypoint(programs: &[(&str, &Pubkey)], tests: BoomerangTests<'_>) {
    let integration = parse_env("INTEGRATION");
    let migration = parse_env("MIGRATION");
    let program = parse_env("PROGRAM");

    if !integration && !migration && !program {
        println!("No tests to run");
        return;
    }

    if program {
        // Run the program tests
        let program_files = programs
            .iter()
            .map(|(program_file, _)| *program_file)
            .collect::<Vec<_>>();
        let program_test = BoomerangProgramTest::new_with_banks(&program_files, tests);
        program_test.run();
    }

    if integration {
        // Run the integration tests
        let integration_test = BoomerangIntegrationTest::new(programs, tests);
        integration_test.run();
    }

    if migration {
        // Run the migration tests
    }
}
