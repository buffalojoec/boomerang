use {
    crate::BoomerangTests,
    libtest_mimic::{Arguments, Trial},
    solana_boomerang_client::BoomerangTestClientConfig,
    solana_sdk::pubkey::Pubkey,
    std::str::FromStr,
};

fn setup_config_for_test(
    config: &mut BoomerangTestClientConfig,
    program_file: &str,
    program_id: &Pubkey,
) {
    config.program_file = program_file.to_string();
    config.program_id = *program_id;
}

pub struct BoomerangProgramTestChunk {
    args: Arguments,
    config: BoomerangTestClientConfig,
    trials: Vec<Trial>,
}
impl BoomerangProgramTestChunk {
    pub fn config(&self) -> &BoomerangTestClientConfig {
        &self.config
    }

    pub fn run(self) {
        libtest_mimic::run(&self.args, self.trials).exit_if_failed();
    }
}

pub struct BoomerangProgramTestIteration {
    chunks: Vec<BoomerangProgramTestChunk>,
    program_file: String,
}
impl BoomerangProgramTestIteration {
    pub fn chunks(self) -> Vec<BoomerangProgramTestChunk> {
        self.chunks
    }

    pub fn program_file(&self) -> &str {
        &self.program_file
    }

    pub fn run(self) {
        for chunk in self.chunks {
            chunk.run();
        }
    }
}

pub fn map_iteration(
    program: &(&str, &str),
    tests: BoomerangTests<'_>,
    use_banks: bool,
) -> BoomerangProgramTestIteration {
    let (file, id) = program;
    let program_file = file.to_string();
    let program_id = Pubkey::from_str(*id).unwrap();
    let chunks = tests
        .iter()
        .map(|(config, test_funcs)| {
            let trials = test_funcs
                .iter()
                .map(|test_func| {
                    let mut config = config.clone();
                    setup_config_for_test(&mut config, &program_file, &program_id);
                    test_func(config, use_banks)
                })
                .collect();
            BoomerangProgramTestChunk {
                args: Arguments::from_args(),
                config: config.clone(),
                trials,
            }
        })
        .collect();
    BoomerangProgramTestIteration {
        chunks,
        program_file,
    }
}

pub struct BoomerangProgramTest {
    iterations: Vec<BoomerangProgramTestIteration>,
}
impl BoomerangProgramTest {
    fn new(programs: &[(&str, &str)], tests: BoomerangTests<'_>, use_banks: bool) -> Self {
        let iterations = programs
            .iter()
            .map(|program| map_iteration(program, tests, use_banks))
            .collect();

        Self { iterations }
    }

    pub fn new_with_banks(programs: &[(&str, &str)], tests: BoomerangTests<'_>) -> Self {
        Self::new(programs, tests, /* use_banks */ true)
    }

    pub fn new_with_rpc(programs: &[(&str, &str)], tests: BoomerangTests<'_>) -> Self {
        Self::new(programs, tests, /* use_banks */ false)
    }

    pub fn run(self) {
        for iteration in self.iterations {
            println!("Running program tests for {}", iteration.program_file);
            iteration.run();
        }
    }

    pub fn iterations(self) -> Vec<BoomerangProgramTestIteration> {
        self.iterations
    }
}
