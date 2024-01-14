use {
    crate::dirs,
    solana_boomerang_test_validator::commands::{run_command, run_command_with_dir},
    solana_sdk::signature::Keypair,
    std::path::PathBuf,
};

const SOLANA_REPOSITORY: &str = "https://github.com/buffalojoe/solana.git";
const SOLANA_BRANCH: &str = "boomerang";

pub fn setup(_target_program: &str) -> (Keypair, PathBuf) {
    let solana_install_path = dirs::solana_install_path();

    // Clear any local changes
    run_command_with_dir("git reset --hard", &solana_install_path);

    // Fetch the latest changes
    if solana_install_path.exists() {
        run_command_with_dir(
            &format!("git checkout {}", SOLANA_BRANCH),
            &solana_install_path,
        );
        run_command_with_dir("git pull", &solana_install_path);
    } else {
        run_command(&format!(
            "git clone {} --branch {}",
            SOLANA_REPOSITORY, SOLANA_BRANCH,
        ));
    }

    // Generate a keypair for the feature ID
    let temp_dir = dirs::temporary_directory_path();
    dirs::create_directory(&temp_dir);

    let feature_keypair = Keypair::new();
    let feature_keypair_path = temp_dir.join("feature-keypair.json");

    dirs::write_keypair_to_path(&feature_keypair, &feature_keypair_path);

    // Add the feature ID to the bank as a native program migration
    // unimplemented!()

    // Build Solana
    run_command_with_dir("./cargo build", &solana_install_path);

    (feature_keypair, feature_keypair_path)
}
