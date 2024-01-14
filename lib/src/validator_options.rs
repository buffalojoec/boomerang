use {
    crate::dirs,
    solana_boomerang_client::BoomerangTestClientConfig,
    solana_boomerang_test_validator::start_options::{
        AddressOrKeypair, BoomerangTestValidatorStartOptions,
    },
};

pub trait IntoTestValidatorStartOptions {
    fn to_test_validator_start_options(&self) -> Vec<BoomerangTestValidatorStartOptions>;
}

impl IntoTestValidatorStartOptions for BoomerangTestClientConfig {
    fn to_test_validator_start_options(&self) -> Vec<BoomerangTestValidatorStartOptions> {
        let mut options = vec![];

        options.push(BoomerangTestValidatorStartOptions::UpgradeableProgram {
            address_or_keypair: AddressOrKeypair::Address(self.program_id.to_string()),
            so_file_path: dirs::program_so_path(&self.program_file),
            upgrade_authority: AddressOrKeypair::Address(self.program_id.to_string()),
        });

        self.features_disabled.iter().for_each(|feature| {
            options.push(BoomerangTestValidatorStartOptions::DeactivateFeature {
                feature_pubkey: feature.to_string(),
            });
        });

        if self.warp_slot > 0 {
            options.push(BoomerangTestValidatorStartOptions::WarpSlot {
                warp_slot: self.warp_slot,
            });
        }

        options
    }
}
