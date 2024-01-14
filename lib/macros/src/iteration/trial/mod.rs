mod parser;

pub struct Trial {
    function_full_path: syn::Path,
    generated_trial_name: syn::Ident,
}

impl Trial {
    pub fn generated_trial_name(&self) -> &syn::Ident {
        &self.generated_trial_name
    }
}

impl From<&(&String, &syn::ItemFn)> for Trial {
    fn from(item_fn: &(&String, &syn::ItemFn)) -> Self {
        let function_full_path = syn::parse_str::<syn::Path>(&format!(
            "{}::{}",
            // This path starts with `::` to be used from the crate root.
            // However, since we're working with tests, the module is expected to be
            // declared in the test root, in our case `tests/main.rs` (as required by
            // Boomerang).
            item_fn.0.chars().skip(2).collect::<String>(),
            item_fn.1.sig.ident.to_string(),
        ))
        .unwrap();
        let generated_trial_name = syn::Ident::new(
            &format!("boomerang_{}", item_fn.1.sig.ident.to_string()),
            item_fn.1.sig.ident.span(),
        );
        Self {
            function_full_path,
            generated_trial_name,
        }
    }
}

impl quote::ToTokens for Trial {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend::<proc_macro2::TokenStream>(self.into());
    }
}

impl From<&Trial> for proc_macro2::TokenStream {
    fn from(ast: &Trial) -> Self {
        let function_full_path = &ast.function_full_path;
        let generated_trial_name = &ast.generated_trial_name;

        quote::quote! {
            fn #generated_trial_name (
                config: solana_boomerang::client::BoomerangTestClientConfig,
                use_banks: bool,
            ) -> solana_boomerang::libtest_mimic::Trial {
                solana_boomerang::boomerang_trial!(
                    #function_full_path
                )(config, use_banks)
            }
        }
    }
}

pub struct TrialConfig {
    features_disabled: Vec<syn::Path>,
    warp_slot: u64,
}

impl Default for TrialConfig {
    fn default() -> Self {
        Self {
            features_disabled: Vec::new(),
            warp_slot: 0,
        }
    }
}

impl PartialEq for TrialConfig {
    fn eq(&self, other: &Self) -> bool {
        self.features_disabled == other.features_disabled && self.warp_slot == other.warp_slot
    }
}

impl syn::parse::Parse for TrialConfig {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        parser::parse_trial_config(input)
    }
}

impl quote::ToTokens for TrialConfig {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend::<proc_macro2::TokenStream>(self.into());
    }
}

impl From<&TrialConfig> for proc_macro2::TokenStream {
    fn from(ast: &TrialConfig) -> Self {
        let features_disabled = &ast.features_disabled;
        let warp_slot = ast.warp_slot;

        quote::quote! {
            solana_boomerang::client::BoomerangTestClientConfig {
                features_disabled: vec![
                    #( #features_disabled() ),*
                ],
                warp_slot: #warp_slot,
                ..solana_boomerang::client::BoomerangTestClientConfig::default()
            }
        }
    }
}
