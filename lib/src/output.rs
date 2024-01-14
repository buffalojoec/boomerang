use {
    std::io::Write,
    termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor},
};

fn boomerang_output(msg: &str, color: Color) -> Result<(), std::io::Error> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    println!();
    println!();
    stdout.set_color(ColorSpec::new().set_fg(Some(color)).set_bold(true))?;
    write!(&mut stdout, "    [boomerang]: ")?;
    stdout.reset()?;
    writeln!(&mut stdout, "{}", msg)?;
    println!();
    println!();
    Ok(())
}

fn boomerang(msg: &str, color: Color) {
    boomerang_output(msg, color).unwrap();
}

pub fn no_tests_to_run() {
    boomerang("No tests to run", Color::Yellow);
}

pub fn starting_program_tests(program: &str) {
    boomerang(
        &format!("Starting program tests for {}", program),
        Color::Cyan,
    );
}

pub fn starting_integration_tests(program: &str) {
    boomerang(
        &format!("Starting integration tests for {}", program),
        Color::Cyan,
    );
}

pub fn starting_migration_tests(source_program: &str, target: &str) {
    boomerang(
        &format!(
            "Starting migration tests for {} against {}",
            source_program, target
        ),
        Color::Cyan,
    );
}

pub fn chunk(i: usize, total: usize) {
    boomerang(&format!("Round {} of {}", i, total), Color::Cyan);
}
