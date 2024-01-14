mod parser;

pub struct Entrypoint {
    programs: Vec<(String, String)>,
    program_tests: Vec<String>,
    integration_tests: Vec<String>,
    migration_tests: Vec<(String, String)>,
}
impl Entrypoint {
    pub fn new(
        programs: Vec<(String, String)>,
        program_tests: Vec<String>,
        integration_tests: Vec<String>,
        migration_tests: Vec<(String, String)>,
    ) -> Self {
        Self {
            programs,
            program_tests,
            integration_tests,
            migration_tests,
        }
    }
}

impl syn::parse::Parse for Entrypoint {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        parser::parse_entrypoint(input)
    }
}

impl quote::ToTokens for Entrypoint {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend::<proc_macro2::TokenStream>(self.into());
    }
}

impl From<&Entrypoint> for proc_macro2::TokenStream {
    fn from(ast: &Entrypoint) -> Self {
        use quote::ToTokens;

        let programs = &ast.programs;
        let program_tests = &ast.program_tests;
        let integration_tests = &ast.integration_tests;
        let migration_tests = &ast.migration_tests;

        let test_iterations = crate::iteration::Iteration::parse_iterations().unwrap();

        let all_programs_tokens = programs
            .iter()
            .map(|(name, pubkey)| {
                quote::quote! {
                    (#name, #pubkey)
                }
            })
            .collect::<Vec<_>>();

        let all_program_tests_args_tokens = program_tests
            .iter()
            .map(|i| {
                quote::quote! {
                    #i
                }
            })
            .collect::<Vec<_>>();

        let all_integration_tests_args_tokens = integration_tests
            .iter()
            .map(|i| {
                quote::quote! {
                    #i
                }
            })
            .collect::<Vec<_>>();

        let all_migration_tests_args_tokens = migration_tests
            .iter()
            .map(|(name, file)| {
                quote::quote! {
                    (#name, #file)
                }
            })
            .collect::<Vec<_>>();

        let all_trials_tokens = test_iterations
            .iter()
            .flat_map(|i| i.trials().iter().map(|trial| trial.to_token_stream()))
            .collect::<Vec<_>>();

        let all_iterations_tokens = test_iterations
            .iter()
            .map(|i| i.to_token_stream())
            .collect::<Vec<_>>();

        quote::quote! {
            use solana_boomerang::tokio;

            #(# all_trials_tokens)*

            #[tokio::main]
            async fn main() {
                let programs = &[
                    #( #all_programs_tokens ),*
                ];

                let program_tests = &[
                    #( #all_program_tests_args_tokens ),*
                ];

                let integration_tests = &[
                    #( #all_integration_tests_args_tokens ),*
                ];

                let migration_tests = &[
                    #( #all_migration_tests_args_tokens ),*
                ];

                let tests: solana_boomerang::BoomerangTests = &[
                    #( #all_iterations_tokens ),*
                ];

                solana_boomerang::entrypoint(
                    programs,
                    program_tests,
                    integration_tests,
                    migration_tests,
                    tests,
                ).await;
            }
        }
    }
}
