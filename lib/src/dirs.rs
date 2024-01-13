use {
    cargo_metadata::MetadataCommand,
    solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer},
    std::path::PathBuf,
};

pub fn workspace_root() -> Result<PathBuf, Box<dyn std::error::Error>> {
    MetadataCommand::new()
        .exec()
        .map(|metadata| metadata.workspace_root.into_std_path_buf())
        .map_err(|err| err.into())
}

pub fn _read_pubkey_from_keypair_path(path: &str) -> Result<Pubkey, Box<dyn std::error::Error>> {
    let file_contents = std::fs::read_to_string(path)?;
    let bytes: Vec<u8> = serde_json::from_str(&file_contents)?;
    let keypair = Keypair::from_bytes(&bytes)?;
    Ok(keypair.pubkey())
}

pub fn write_keypair_to_path(
    keypair: &Keypair,
    path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = keypair.to_bytes().to_vec();
    let file_contents = serde_json::to_string(&bytes)?;
    std::fs::write(path, file_contents)?;
    Ok(())
}
