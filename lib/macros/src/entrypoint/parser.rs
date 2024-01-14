enum ParsedEntrypointArg {
    Programs(Vec<crate::parser::ParsedStringTupleItem>),
    ProgramTests(Vec<crate::parser::ParsedStringItem>),
    IntegrationTests(Vec<crate::parser::ParsedStringItem>),
    MigrationTests(Vec<crate::parser::ParsedStringTupleItem>),
}
impl syn::parse::Parse for ParsedEntrypointArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use crate::parser::{parse_bracketed_list_arg, ParsedStringItem, ParsedStringTupleItem};

        if input.peek(syn::Ident) {
            let ident = input.parse::<syn::Ident>()?;
            match ident.to_string().as_str() {
                "programs" => Ok(Self::Programs(parse_bracketed_list_arg::<
                    ParsedStringTupleItem,
                >(input)?)),
                "program_tests" => Ok(Self::ProgramTests(parse_bracketed_list_arg::<
                    ParsedStringItem,
                >(input)?)),
                "integration_tests" => Ok(Self::IntegrationTests(parse_bracketed_list_arg::<
                    ParsedStringItem,
                >(input)?)),
                "migration_tests" => Ok(Self::MigrationTests(parse_bracketed_list_arg::<
                    ParsedStringTupleItem,
                >(input)?)),
                _ => Err(syn::Error::new(input.span(), "Unknown argument")),
            }
        } else {
            Err(syn::Error::new(input.span(), "Unknown argument"))
        }
    }
}

pub fn parse_entrypoint(
    input: syn::parse::ParseStream,
) -> syn::Result<crate::entrypoint::Entrypoint> {
    let mut programs: Vec<(String, String)> = Vec::new();
    let mut program_tests: Vec<String> = Vec::new();
    let mut integration_tests: Vec<String> = Vec::new();
    let mut migration_tests: Vec<(String, String)> = Vec::new();

    let args = crate::parser::parse_list::<ParsedEntrypointArg>(input)?;

    for arg in args {
        match arg {
            ParsedEntrypointArg::Programs(programs_arg) => {
                programs_arg.iter().for_each(|arg| {
                    programs.push(arg.value());
                });
            }
            ParsedEntrypointArg::ProgramTests(program_tests_arg) => {
                program_tests_arg.iter().for_each(|arg| {
                    program_tests.push(arg.value());
                });
            }
            ParsedEntrypointArg::IntegrationTests(integration_tests_arg) => {
                integration_tests_arg.iter().for_each(|arg| {
                    integration_tests.push(arg.value());
                });
            }
            ParsedEntrypointArg::MigrationTests(migration_tests_arg) => {
                migration_tests_arg.iter().for_each(|arg| {
                    migration_tests.push(arg.value());
                });
            }
        }
    }

    Ok(crate::entrypoint::Entrypoint::new(
        programs,
        program_tests,
        integration_tests,
        migration_tests,
    ))
}
