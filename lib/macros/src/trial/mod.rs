mod parser;

pub struct SolanaBoomerangTrial {
    deactivate_features: Vec<String>,
    warp_slot: u64,
}

impl syn::parse::Parse for SolanaBoomerangTrial {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        parser::parse_trial(input)
    }
}

impl quote::ToTokens for SolanaBoomerangTrial {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend::<proc_macro2::TokenStream>(self.into());
    }
}

impl From<&SolanaBoomerangTrial> for proc_macro2::TokenStream {
    fn from(ast: &SolanaBoomerangTrial) -> Self {
        let deactivate_features = &ast.deactivate_features;
        let warp_slot = ast.warp_slot;

        println!("deactivate_features:  {:?}", deactivate_features);
        println!("warp_slot:            {}", warp_slot);
        println!("  ..BoomerangClientTestConfig::default()");

        // By default, we want to codegen the closure that creates the `Trial`.
        // However, when building the list of `SolanaBoomerangTrail`s, the
        // items sharing the same `warp_slot` and `deactivate_features` will
        // be merged into one test iteration.

        // Here the codegen is _just_ yielding the closure. Merging based on
        // configs comes later in the main `tests: [..]` list built by the
        // entrypoint.

        quote::quote! {}
    }
}
