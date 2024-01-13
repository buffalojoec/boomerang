use {solana_sdk::pubkey::Pubkey, std::path::PathBuf};

trait ToStringArg {
    fn to_string_arg(&self) -> String;
}

impl ToStringArg for PathBuf {
    fn to_string_arg(&self) -> String {
        self.to_str()
            .expect("Failed to convert path to string")
            .to_string()
    }
}

pub enum AddressOrKeypair {
    Address(Pubkey),
    Keypair(PathBuf),
}
impl ToStringArg for AddressOrKeypair {
    fn to_string_arg(&self) -> String {
        match self {
            AddressOrKeypair::Address(address) => address.to_string(),
            AddressOrKeypair::Keypair(keypair_path) => keypair_path.to_string_arg(),
        }
    }
}

pub enum UrlOrMoniker {
    Url(String),
    Localnet,
    Devnet,
    Testnet,
    MainnetBeta,
}

pub enum BoomerangTestValidatorStartOptions {
    /// Load an account from the provided JSON file
    Account { address: Pubkey, dump_path: PathBuf },
    /// Load all the accounts from the JSON files found in the specified
    /// DIRECTORY
    AccountDir { directory: PathBuf },
    /// Add a SBF program to the genesis configuration with upgrades disabled
    BpfProgram {
        address_or_keypair: AddressOrKeypair,
        so_file_path: PathBuf,
    },
    /// Copy an account from the cluster referenced by the --url argument the
    /// genesis configuration
    Clone { address: Pubkey },
    /// Copy an upgradeable program and its executable data from the cluster
    /// referenced by the --url argument the genesis configuration
    CloneUpgradeableProgram { address: Pubkey },
    /// Configuration file to use
    Config { path: PathBuf },
    /// Deactivate this feature in genesis
    DeactivateFeature { feature_pubkey: Pubkey },
    /// Use DIR as ledger location
    Ledger { dir: PathBuf },
    /// Copy an account from the cluster referenced by the --url argument,
    /// skipping it if it doesn't exist
    MaybeClone { address: Pubkey },
    /// Address of the mint account that will receive tokens created at genesis
    Mint { address: Pubkey },
    /// Override the number of slots in an epoch
    SlotsPerEpoch { slots: u64 },
    /// Add an upgradeable SBF program to the genesis configuration
    UpgradeableProgram {
        address_or_keypair: AddressOrKeypair,
        so_file_path: PathBuf,
        upgrade_authority: AddressOrKeypair,
    },
    /// URL for Solana's JSON RPC or moniker
    Url { url_or_moniker: UrlOrMoniker },
    /// Warp the ledger to WARP_SLOT after starting the validator.
    /// If no slot is provided then the current slot of the cluster referenced
    /// by the --url argument will be used.
    WarpSlot { warp_slot: u64 },
}
impl ToStringArg for BoomerangTestValidatorStartOptions {
    fn to_string_arg(&self) -> String {
        match self {
            Self::Account { address, dump_path } => format!(
                "--account {} {}",
                address.to_string(),
                dump_path.to_string_arg()
            ),
            Self::AccountDir { directory } => {
                format!("--account-dir {}", directory.to_string_arg())
            }
            Self::BpfProgram {
                address_or_keypair,
                so_file_path,
            } => format!(
                "--bpf-program {} {}",
                address_or_keypair.to_string_arg(),
                so_file_path.to_string_arg()
            ),
            Self::Clone { address } => {
                format!("--clone {}", address.to_string())
            }
            Self::CloneUpgradeableProgram { address } => {
                format!("--clone-upgradeable-program {}", address.to_string())
            }
            Self::Config { path } => {
                format!("--config {}", path.to_string_arg())
            }
            Self::DeactivateFeature { feature_pubkey } => {
                format!("--deactivate-feature {}", feature_pubkey.to_string())
            }
            Self::Ledger { dir } => {
                format!("--ledger {}", dir.to_string_arg())
            }
            Self::MaybeClone { address } => {
                format!("--maybe-clone {}", address.to_string())
            }
            Self::Mint { address } => {
                format!("--mint {}", address.to_string())
            }
            Self::SlotsPerEpoch { slots } => {
                format!("--slots-per-epoch {}", slots.to_string())
            }
            Self::UpgradeableProgram {
                address_or_keypair,
                so_file_path,
                upgrade_authority,
            } => format!(
                "--upgradeable-program {} {} {}",
                address_or_keypair.to_string_arg(),
                so_file_path.to_string_arg(),
                upgrade_authority.to_string_arg()
            ),
            Self::Url { url_or_moniker } => match url_or_moniker {
                UrlOrMoniker::Url(url) => format!("--url {}", url),
                UrlOrMoniker::Localnet => "--ul".to_string(),
                UrlOrMoniker::Devnet => "-ud".to_string(),
                UrlOrMoniker::Testnet => "-ut".to_string(),
                UrlOrMoniker::MainnetBeta => "-um".to_string(),
            },
            Self::WarpSlot { warp_slot } => {
                format!("--warp-slot {}", warp_slot.to_string())
            }
        }
    }
}
impl BoomerangTestValidatorStartOptions {
    pub fn args_to_string(args: Vec<Self>) -> String {
        args.into_iter()
            .map(|arg| arg.to_string_arg())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
