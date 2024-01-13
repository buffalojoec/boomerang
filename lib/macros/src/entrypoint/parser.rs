struct ParsedProgramItem(syn::LitStr, syn::LitStr);
impl syn::parse::Parse for ParsedProgramItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        syn::parenthesized!(content in input);
        let name: syn::LitStr = content.parse()?;
        content.parse::<syn::Token![,]>()?;
        let id: syn::LitStr = content.parse()?;
        Ok(Self(name, id))
    }
}

struct ParsedProgramsArg {
    _equals_sign: syn::Token![=],
    value: Vec<ParsedProgramItem>,
}
impl syn::parse::Parse for ParsedProgramsArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _equals_sign = input.parse::<syn::Token![=]>()?;
        let content;
        syn::bracketed!(content in input);
        let value = content
            .parse_terminated(ParsedProgramItem::parse, syn::Token![,])?
            .into_iter()
            .collect();

        Ok(Self {
            _equals_sign,
            value,
        })
    }
}

struct ParsedTestArg {
    _equals_sign: syn::Token![=],
    value: syn::LitBool,
}
impl syn::parse::Parse for ParsedTestArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _equals_sign = input.parse::<syn::Token![=]>()?;
        let value = input.parse::<syn::LitBool>()?;
        Ok(Self {
            _equals_sign,
            value,
        })
    }
}

struct ParsedMigrationItem(syn::LitStr, syn::Path);
impl syn::parse::Parse for ParsedMigrationItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        syn::parenthesized!(content in input);
        let name: syn::LitStr = content.parse()?;
        content.parse::<syn::Token![,]>()?;
        let id: syn::Path = content.parse()?;
        Ok(Self(name, id))
    }
}

struct ParsedMigrationTestArg {
    _equals_sign: syn::Token![=],
    value: Vec<ParsedMigrationItem>,
}
impl syn::parse::Parse for ParsedMigrationTestArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _equals_sign = input.parse::<syn::Token![=]>()?;
        let content;
        syn::bracketed!(content in input);
        let value = content
            .parse_terminated(ParsedMigrationItem::parse, syn::Token![,])?
            .into_iter()
            .collect();

        Ok(Self {
            _equals_sign,
            value,
        })
    }
}

enum ParsedEntrypointArg {
    Programs(ParsedProgramsArg),
    ProgramTests(ParsedTestArg),
    IntegrationTests(ParsedTestArg),
    MigrationTests(ParsedMigrationTestArg),
}
impl syn::parse::Parse for ParsedEntrypointArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Ident) {
            let ident = input.parse::<syn::Ident>()?;
            match ident.to_string().as_str() {
                "programs" => {
                    let arg = ParsedProgramsArg::parse(input)?;
                    Ok(Self::Programs(arg))
                }
                "program_tests" => {
                    let arg = ParsedTestArg::parse(input)?;
                    Ok(Self::ProgramTests(arg))
                }
                "integration_tests" => {
                    let arg = ParsedTestArg::parse(input)?;
                    Ok(Self::IntegrationTests(arg))
                }
                "migration_tests" => {
                    let arg = ParsedMigrationTestArg::parse(input)?;
                    Ok(Self::MigrationTests(arg))
                }
                _ => Err(syn::Error::new(input.span(), "Unknown argument")),
            }
        } else {
            Err(syn::Error::new(input.span(), "Unknown argument"))
        }
    }
}

struct ParsedEntrypointArgs {
    args: Vec<ParsedEntrypointArg>,
}
impl syn::parse::Parse for ParsedEntrypointArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let args = input
            .parse_terminated(ParsedEntrypointArg::parse, syn::Token![,])?
            .into_iter()
            .collect();
        Ok(Self { args })
    }
}

pub fn parse_entrypoint(
    input: syn::parse::ParseStream,
) -> syn::Result<crate::entrypoint::SolanaBoomerangEntrypoint> {
    use {quote::ToTokens, syn::parse::Parse};

    let input = ParsedEntrypointArgs::parse(input)?;

    let mut programs: Vec<(String, String)> = Vec::new();
    let mut program_tests = false;
    let mut integration_tests = false;
    let mut migration_tests: Vec<(String, String)> = Vec::new();

    for arg in input.args {
        match arg {
            ParsedEntrypointArg::Programs(programs_arg) => {
                programs_arg.value.iter().for_each(|arg| {
                    programs.push((arg.0.value(), arg.1.value()));
                });
            }
            ParsedEntrypointArg::ProgramTests(arg) => {
                program_tests = arg.value.value;
            }
            ParsedEntrypointArg::IntegrationTests(arg) => {
                integration_tests = arg.value.value;
            }
            ParsedEntrypointArg::MigrationTests(migration_tests_arg) => {
                migration_tests_arg.value.iter().for_each(|arg| {
                    migration_tests.push((arg.0.value(), arg.1.to_token_stream().to_string()));
                });
            }
        }
    }

    Ok(crate::entrypoint::SolanaBoomerangEntrypoint::new(
        programs,
        program_tests,
        integration_tests,
        migration_tests,
    ))
}
