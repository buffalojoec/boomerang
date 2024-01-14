mod parser;

pub struct Trial {
    function_full_path: syn::Ident,
    generated_trial_name: syn::Ident,
}

impl Trial {
    pub fn generated_trial_name(&self) -> &syn::Ident {
        &self.generated_trial_name
    }
}

impl From<&syn::ItemFn> for Trial {
    fn from(item_fn: &syn::ItemFn) -> Self {
        let function_full_path = item_fn.sig.ident.clone();
        let generated_trial_name = syn::Ident::new(
            &format!("boomerang_{}", function_full_path.to_string()),
            function_full_path.span(),
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

        let _ = quote::quote! {
            fn #generated_trial_name (
                config: solana_boomerang::client::BoomerangTestClientConfig,
                use_banks: bool,
            ) -> solana_boomerang::libtest_mimic::Trial {
                solana_boomerang::boomerang_trial!(
                    #function_full_path
                )(config, use_banks)
            }
        };
        quote::quote! {}
    }
}

pub struct TrialConfig {
    features_disabled: Vec<String>,
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

        let _ = quote::quote! {
            solana_boomerang::client::BoomerangTestClientConfig {
                features_disabled: vec![
                    #( #features_disabled ),*
                ],
                program_file: program_file.clone(),
                program_id,
                warp_slot: #warp_slot,
                ..solana_boomerang::client::BoomerangClientTestConfig::default()
            }
        };
        quote::quote! {}
    }
}
