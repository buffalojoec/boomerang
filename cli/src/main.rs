use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// Run compatibility tests between multiple program implementations.
    #[clap(short, long, action)]
    compatibility: bool,
    /// Run migration tests between two program implementations.
    #[clap(short, long, action)]
    migration: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Run tests on the program itself
    println!("Running basic tests...");
    println!("cargo test-sbf");

    if cli.compatibility {
        // Run tests on the other implementations
        println!("Running compatibility tests...");
        println!("COMPATIBILITY_TEST=1 cargo test-sbf");
    }
    if cli.migration {
        // Run tests on the target program
        // Execute the migration
        // Run tests on the target program again
        println!("Running migration tests...");
        println!("MIGRATION_TEST=1 cargo test-sbf");
    }

    Ok(())
}
