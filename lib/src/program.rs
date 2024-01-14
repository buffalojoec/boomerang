use {
    crate::{BoomerangTest, BoomerangTests},
    libtest_mimic::{Arguments, Trial},
    solana_boomerang_client::BoomerangTestClientConfig,
    solana_sdk::pubkey::Pubkey,
    std::str::FromStr,
};

/// Overwrites the `program_file` and `program_id` fields of the given `config`
/// with the given values
fn setup_config_for_test(
    config: &mut BoomerangTestClientConfig,
    program_file: &str,
    program_id: &Pubkey,
) {
    config.program_file = program_file.to_string();
    config.program_id = *program_id;
}

/// A chunk of tests for a single program.
/// These chunks are provided to the `entrypoint` as trials that share a common
/// setup config.
pub struct BoomerangProgramTestChunk {
    args: Arguments,
    config: BoomerangTestClientConfig,
    trials: Vec<Trial>,
}
impl BoomerangProgramTestChunk {
    pub fn new(
        program_file: &str,
        program_id: &Pubkey,
        test_suite: &BoomerangTest<'_>,
        use_banks: bool,
    ) -> Self {
        let (test_config, test_funcs) = test_suite;

        let mut config = test_config.clone();
        setup_config_for_test(&mut config, program_file, program_id);

        let args = Arguments::default();
        let trials = test_funcs
            .iter()
            .map(|test_func| test_func(config.clone(), use_banks))
            .collect();

        Self {
            args,
            config,
            trials,
        }
    }

    pub fn config(&self) -> &BoomerangTestClientConfig {
        &self.config
    }

    /// Run the tests for a single chunk of program tests with shared setup
    /// configs.
    /// This particular function is used for integration and migration tests,
    /// since it allows each chunk to be run serially.
    pub fn run(self) {
        libtest_mimic::run(&self.args, self.trials).exit_if_failed();
    }
}

/// A program test iteration for a particular program.
/// These iterations comprise all test chunks for a given program.
pub struct BoomerangProgramTestIteration {
    chunks: Vec<BoomerangProgramTestChunk>,
    program_file: String,
}
impl BoomerangProgramTestIteration {
    pub fn new(program: &(&str, &str), tests: BoomerangTests<'_>, use_banks: bool) -> Self {
        let (file, id) = program;
        let program_file = file.to_string();
        let program_id = Pubkey::from_str(*id).unwrap();

        let chunks = tests
            .iter()
            .map(|test_suite| {
                BoomerangProgramTestChunk::new(&program_file, &program_id, test_suite, use_banks)
            })
            .collect();

        Self {
            chunks,
            program_file,
        }
    }

    pub fn chunks(self) -> Vec<BoomerangProgramTestChunk> {
        self.chunks
    }

    pub fn program_file(&self) -> &str {
        &self.program_file
    }

    /// Run the tests for the entire iteration in parallel.
    pub fn parallel_run(self) {
        let args = Arguments::default();
        let trials = self
            .chunks
            .into_iter()
            .map(|chunk| chunk.trials)
            .flatten()
            .collect();
        libtest_mimic::run(&args, trials).exit_if_failed();
    }
}

/// The Program Test runner.
/// Runs all program test iterations for a given set of programs.
pub struct BoomerangProgramTest {
    iterations: Vec<BoomerangProgramTestIteration>,
}
impl BoomerangProgramTest {
    pub fn build_program_test_iterations(
        programs: &[(&str, &str)],
        tests: BoomerangTests<'_>,
        use_banks: bool,
    ) -> Vec<BoomerangProgramTestIteration> {
        programs
            .iter()
            .map(|program| BoomerangProgramTestIteration::new(program, tests, use_banks))
            .collect()
    }

    pub fn new(programs: &[(&str, &str)], tests: BoomerangTests<'_>) -> Self {
        Self {
            iterations: Self::build_program_test_iterations(
                programs, tests, /* use_banks */ true,
            ),
        }
    }

    /// Since program tests are run with a `BankClient`, they can be run in
    /// parallel, so we don't need to run them one chunk at a time.
    /// Each trial's setup config will create a new unique `BankClient` for
    /// that particular trial, so we can run all trials in parallel.
    pub fn run(self) {
        for iteration in self.iterations {
            println!("Running program tests for {}", iteration.program_file);
            iteration.parallel_run();
        }
    }

    pub fn iterations(self) -> Vec<BoomerangProgramTestIteration> {
        self.iterations
    }
}
