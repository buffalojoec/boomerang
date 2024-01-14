mod parser;

pub struct Entrypoint {
    programs: Vec<(String, String)>,
    program_tests: bool,
    integration_tests: bool,
    migration_tests: Vec<(String, String)>,
}
impl Entrypoint {
    pub fn new(
        programs: Vec<(String, String)>,
        program_tests: bool,
        integration_tests: bool,
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
        let programs = &ast.programs;
        let program_tests = ast.program_tests;
        let integration_tests = ast.integration_tests;
        let migration_tests = &ast.migration_tests;

        let _test_iterations = crate::iteration::Iteration::parse_iterations().unwrap();

        println!("programs:             {:?}", programs);
        println!("program_tests:        {}", program_tests);
        println!("integration_tests:    {}", integration_tests);
        println!("migration_tests:      {:?}", migration_tests);

        quote::quote! {}
    }
}
