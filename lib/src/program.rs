use {
    libtest_mimic::{Arguments, Trial},
    solana_boomerang_client::BoomerangTestClientConfig,
};

pub struct BoomerangProgramTestIteration {
    pub args: Arguments,
    pub program_file: String,
    pub trials: Vec<Trial>,
}
impl BoomerangProgramTestIteration {
    pub fn run(self) {
        std::thread::sleep(std::time::Duration::from_secs(2));
        libtest_mimic::run(&self.args, self.trials).exit_if_failed();
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
    let trials = tests
        .iter()
        .map(|(config, test_funcs)| {
            test_funcs
                .iter()
                .map(|test_func| test_func(config.clone(), use_banks))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();
    BoomerangProgramTestIteration {
        args: Arguments::from_args(),
        program_file,
        trials,
    }
}

pub fn build_iterations<P>(
    program_files: &[&str],
    tests: &[(BoomerangTestClientConfig, &[P])],
    use_banks: bool,
) -> Vec<BoomerangProgramTestIteration>
where
    P: Fn(BoomerangTestClientConfig, bool) -> Trial,
{
    program_files
        .iter()
        .map(|program_file| map_iteration(program_file, tests, use_banks))
        .collect()
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
        let iterations = build_iterations(program_files, tests, use_banks);

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
