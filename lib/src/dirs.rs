use {
    cargo_metadata::MetadataCommand,
    solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer},
    std::path::PathBuf,
};

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

pub fn test_ledger_path() -> PathBuf {
    workspace_root().join("test-ledger")
}

pub fn _read_pubkey_from_keypair_path(
    path: &PathBuf,
) -> Result<Pubkey, Box<dyn std::error::Error>> {
    let file_contents = std::fs::read_to_string(path)?;
    let bytes: Vec<u8> = serde_json::from_str(&file_contents)?;
    let keypair = Keypair::from_bytes(&bytes)?;
    Ok(keypair.pubkey())
}

pub fn _write_keypair_to_path(
    keypair: &Keypair,
    path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = keypair.to_bytes().to_vec();
    let file_contents = serde_json::to_string(&bytes)?;
    std::fs::write(path, file_contents)?;
    Ok(())
}
