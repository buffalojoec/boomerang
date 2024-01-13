use {
    libtest_mimic::{Arguments, Trial},
    solana_boomerang_client::BoomerangTestClientConfig,
};

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

pub fn map_iteration<P>(
    program_file: &str,
    tests: &[(BoomerangTestClientConfig, &[P])],
    use_banks: bool,
) -> BoomerangProgramTestIteration
where
    P: Fn(BoomerangTestClientConfig, bool) -> Trial,
{
    let program_file = program_file.to_string();
    let chunks = tests
        .iter()
        .map(|(config, test_funcs)| {
            let trials = test_funcs
                .iter()
                .map(|test_func| test_func(config.clone(), use_banks))
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
    fn new<P>(
        program_files: &[&str],
        tests: &[(BoomerangTestClientConfig, &[P])],
        use_banks: bool,
    ) -> Self
    where
        P: Fn(BoomerangTestClientConfig, bool) -> Trial,
    {
        let iterations = program_files
            .iter()
            .map(|program_file| map_iteration(program_file, tests, use_banks))
            .collect();

        Self { iterations }
    }

    pub fn new_with_banks<P>(
        program_files: &[&str],
        tests: &[(BoomerangTestClientConfig, &[P])],
    ) -> Self
    where
        P: Fn(BoomerangTestClientConfig, bool) -> Trial,
    {
        Self::new(program_files, tests, /* use_banks */ true)
    }

    pub fn new_with_rpc<P>(
        program_files: &[&str],
        tests: &[(BoomerangTestClientConfig, &[P])],
    ) -> Self
    where
        P: Fn(BoomerangTestClientConfig, bool) -> Trial,
    {
        Self::new(program_files, tests, /* use_banks */ false)
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
