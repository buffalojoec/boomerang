struct ParsedStringItem(syn::LitStr);
impl syn::parse::Parse for ParsedStringItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let val: syn::LitStr = input.parse()?;
        Ok(Self(val))
    }
}
impl ParsedStringItem {
    fn parse_list(input: syn::parse::ParseStream) -> syn::Result<Vec<Self>> {
        use syn::parse::Parse;
        let content;
        syn::bracketed!(content in input);
        Ok(content
            .parse_terminated(Self::parse, syn::Token![,])?
            .into_iter()
            .collect::<Vec<_>>())
    }
}

struct ParsedStringListArg {
    _equals_sign: syn::Token![=],
    value: Vec<ParsedStringItem>,
}
impl syn::parse::Parse for ParsedStringListArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _equals_sign = input.parse::<syn::Token![=]>()?;
        let value = ParsedStringItem::parse_list(input)?;
        Ok(Self {
            _equals_sign,
            value,
        })
    }
}

struct ParsedStringTupleItem(syn::LitStr, syn::LitStr);
impl syn::parse::Parse for ParsedStringTupleItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        syn::parenthesized!(content in input);
        let val1: syn::LitStr = content.parse()?;
        content.parse::<syn::Token![,]>()?;
        let val2: syn::LitStr = content.parse()?;
        Ok(Self(val1, val2))
    }
}
impl ParsedStringTupleItem {
    fn parse_list(input: syn::parse::ParseStream) -> syn::Result<Vec<Self>> {
        use syn::parse::Parse;
        let content;
        syn::bracketed!(content in input);
        Ok(content
            .parse_terminated(Self::parse, syn::Token![,])?
            .into_iter()
            .collect::<Vec<_>>())
    }
}

struct ParsedStringTupleListArg {
    _equals_sign: syn::Token![=],
    value: Vec<ParsedStringTupleItem>,
}
impl syn::parse::Parse for ParsedStringTupleListArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _equals_sign = input.parse::<syn::Token![=]>()?;
        let value = ParsedStringTupleItem::parse_list(input)?;
        Ok(Self {
            _equals_sign,
            value,
        })
    }
}

enum ParsedEntrypointArg {
    Programs(ParsedStringTupleListArg),
    ProgramTests(ParsedStringListArg),
    IntegrationTests(ParsedStringListArg),
    MigrationTests(ParsedStringTupleListArg),
}
impl syn::parse::Parse for ParsedEntrypointArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Ident) {
            let ident = input.parse::<syn::Ident>()?;
            match ident.to_string().as_str() {
                "programs" => {
                    let arg = ParsedStringTupleListArg::parse(input)?;
                    Ok(Self::Programs(arg))
                }
                "program_tests" => {
                    let arg = ParsedStringListArg::parse(input)?;
                    Ok(Self::ProgramTests(arg))
                }
                "integration_tests" => {
                    let arg = ParsedStringListArg::parse(input)?;
                    Ok(Self::IntegrationTests(arg))
                }
                "migration_tests" => {
                    let arg = ParsedStringTupleListArg::parse(input)?;
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
) -> syn::Result<crate::entrypoint::Entrypoint> {
    use syn::parse::Parse;

    let input = ParsedEntrypointArgs::parse(input)?;

    let mut programs: Vec<(String, String)> = Vec::new();
    let mut program_tests: Vec<String> = Vec::new();
    let mut integration_tests: Vec<String> = Vec::new();
    let mut migration_tests: Vec<(String, String)> = Vec::new();

    for arg in input.args {
        match arg {
            ParsedEntrypointArg::Programs(programs_arg) => {
                programs_arg.value.iter().for_each(|arg| {
                    programs.push((arg.0.value(), arg.1.value()));
                });
            }
            ParsedEntrypointArg::ProgramTests(program_tests_arg) => {
                program_tests_arg.value.iter().for_each(|arg| {
                    program_tests.push(arg.0.value());
                });
            }
            ParsedEntrypointArg::IntegrationTests(integration_tests_arg) => {
                integration_tests_arg.value.iter().for_each(|arg| {
                    integration_tests.push(arg.0.value());
                });
            }
            ParsedEntrypointArg::MigrationTests(migration_tests_arg) => {
                migration_tests_arg.value.iter().for_each(|arg| {
                    migration_tests.push((arg.0.value(), arg.1.value()));
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
