use {cargo_metadata::MetadataCommand, solana_sdk::signature::Keypair, std::path::PathBuf};

pub fn workspace_root() -> PathBuf {
    MetadataCommand::new()
        .exec()
        .map(|metadata| metadata.workspace_root.into_std_path_buf())
        .expect("Failed to get workspace root")
}

pub fn program_so_path(program_name: &str) -> PathBuf {
    workspace_root()
        .join("target")
        .join("deploy")
        .join(format!("{}.so", program_name))
}

pub fn solana_install_path() -> PathBuf {
    workspace_root().join(".solana")
}

pub fn solana_cli_path() -> PathBuf {
    solana_install_path()
        .join("target")
        .join("debug")
        .join("solana")
}
pub fn solana_cli_path_string() -> String {
    solana_cli_path()
        .to_str()
        .expect("Failed to convert Solana CLI path to string")
        .to_string()
}

pub fn solana_test_validator_path() -> PathBuf {
    solana_install_path()
        .join("target")
        .join("debug")
        .join("solana-test-validator")
}
pub fn solana_test_validator_path_string() -> String {
    solana_test_validator_path()
        .to_str()
        .expect("Failed to convert Solana test validator path to string")
        .to_string()
}

pub fn temporary_directory_path() -> PathBuf {
    workspace_root().join("tmp")
}

pub fn test_ledger_path() -> PathBuf {
    workspace_root().join("test-ledger")
}

pub fn create_directory(path: &PathBuf) {
    if !path.exists() {
        std::fs::create_dir_all(path).expect("Failed to create directory");
    }
}

pub fn write_keypair_to_path(keypair: &Keypair, path: &PathBuf) {
    let bytes = keypair.to_bytes().to_vec();
    let file_contents = serde_json::to_string(&bytes).expect("Failed to serialize keypair to JSON");
    std::fs::write(path, file_contents).expect("Failed to write keypair to file");
}
