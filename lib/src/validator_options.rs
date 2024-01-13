use {
    solana_boomerang_client::BoomerangTestClientConfig,
    solana_boomerang_test_validator::start_options::BoomerangTestValidatorStartOptions,
};

pub trait IntoTestValidatorStartOptions {
    fn to_test_validator_start_options(&self) -> Vec<BoomerangTestValidatorStartOptions>;
}

impl IntoTestValidatorStartOptions for BoomerangTestClientConfig {
    fn to_test_validator_start_options(&self) -> Vec<BoomerangTestValidatorStartOptions> {
        let mut options = vec![];

        self.features_disabled.iter().for_each(|feature| {
            options.push(BoomerangTestValidatorStartOptions::DeactivateFeature {
                feature_pubkey: *feature,
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
