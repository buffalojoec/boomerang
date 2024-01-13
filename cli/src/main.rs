use clap::Parser;

fn run_command(command: &str) {
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .status()
        .expect("failed to execute process");
    assert!(status.success());
}

#[derive(Parser)]
struct Cli {
    /// Run integrations tests on a program.
    #[clap(short, long, action)]
    integration: bool,
    /// Run migration tests between two program implementations.
    #[clap(short, long, action)]
    migration: bool,
    /// Run program tests on a program.
    #[clap(short, long, action)]
    program: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut boomerang_vars = vec![];
    if cli.program {
        boomerang_vars.push("PROGRAM=true");
    }
    if cli.integration {
        boomerang_vars.push("INTEGRATION=true");
    }
    if cli.migration {
        boomerang_vars.push("MIGRATION=true");
    }

    run_command(&format!("{} cargo test-sbf", boomerang_vars.join(" ")));

    Ok(())
}
