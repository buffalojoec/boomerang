struct ParsedDeactivateFeaturesArg {
    _equals_sign: syn::Token![=],
    value: Vec<syn::Path>,
}
impl syn::parse::Parse for ParsedDeactivateFeaturesArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _equals_sign = input.parse::<syn::Token![=]>()?;
        let content;
        syn::bracketed!(content in input);
        let value = content
            .parse_terminated(syn::Path::parse, syn::Token![,])?
            .into_iter()
            .collect();

        Ok(Self {
            _equals_sign,
            value,
        })
    }
}

struct ParsedWarpSlotArg {
    _equals_sign: syn::Token![=],
    value: syn::LitInt,
}
impl syn::parse::Parse for ParsedWarpSlotArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _equals_sign = input.parse::<syn::Token![=]>()?;
        let value = input.parse::<syn::LitInt>()?;
        Ok(Self {
            _equals_sign,
            value,
        })
    }
}

enum ParsedTrialArg {
    DeactivateFeatures(ParsedDeactivateFeaturesArg),
    WarpSlot(ParsedWarpSlotArg),
}
impl syn::parse::Parse for ParsedTrialArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Ident) {
            let ident = input.parse::<syn::Ident>()?;
            match ident.to_string().as_str() {
                "deactivate_features" => {
                    let arg = ParsedDeactivateFeaturesArg::parse(input)?;
                    Ok(Self::DeactivateFeatures(arg))
                }
                "warp_slot" => {
                    let arg = ParsedWarpSlotArg::parse(input)?;
                    Ok(Self::WarpSlot(arg))
                }
                _ => Err(syn::Error::new(input.span(), "Unknown argument")),
            }
        } else {
            Err(syn::Error::new(input.span(), "Unknown argument"))
        }
    }
}

struct ParsedTrialArgs {
    args: Vec<ParsedTrialArg>,
}
impl syn::parse::Parse for ParsedTrialArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let args = input
            .parse_terminated(ParsedTrialArg::parse, syn::Token![,])?
            .into_iter()
            .collect();
        Ok(Self { args })
    }
}

pub fn parse_trial(
    input: syn::parse::ParseStream,
) -> syn::Result<crate::trial::SolanaBoomerangTrial> {
    use {quote::ToTokens, syn::parse::Parse};

    let input = ParsedTrialArgs::parse(input)?;

    let mut deactivate_features: Vec<String> = Vec::new();
    let mut warp_slot: u64 = 0;

    for arg in input.args {
        match arg {
            ParsedTrialArg::DeactivateFeatures(deactivate_features_arg) => {
                deactivate_features_arg.value.iter().for_each(|arg| {
                    deactivate_features.push(arg.to_token_stream().to_string());
                });
            }
            ParsedTrialArg::WarpSlot(warp_slot_arg) => {
                warp_slot = warp_slot_arg.value.base10_parse::<u64>().unwrap();
            }
        }
    }

    Ok(crate::trial::SolanaBoomerangTrial {
        deactivate_features,
        warp_slot,
    })
}

pub fn _is_boomerang_test_attr(attr: &syn::Attribute) -> bool {
    let path = &attr.path();
    let segments: Vec<&syn::PathSegment> = path.segments.iter().collect();
    if segments.len() != 2 {
        return false;
    }
    if &segments[0].ident == "boomerang" && &segments[1].ident == "test" {
        println!("Found boomerang::test attribute");
        return true;
    }
    println!("Found non-boomerang::test attribute");
    false
}
