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

enum ParsedTrialConfigArg {
    DeactivateFeatures(ParsedDeactivateFeaturesArg),
    WarpSlot(ParsedWarpSlotArg),
}
impl syn::parse::Parse for ParsedTrialConfigArg {
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

struct ParsedTrialConfigArgs {
    args: Vec<ParsedTrialConfigArg>,
}
impl syn::parse::Parse for ParsedTrialConfigArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let args = input
            .parse_terminated(ParsedTrialConfigArg::parse, syn::Token![,])?
            .into_iter()
            .collect();
        Ok(Self { args })
    }
}

pub fn parse_trial_config(
    input: syn::parse::ParseStream,
) -> syn::Result<crate::iteration::trial::TrialConfig> {
    use {quote::ToTokens, syn::parse::Parse};

    let input = ParsedTrialConfigArgs::parse(input)?;

    let mut deactivate_features: Vec<String> = Vec::new();
    let mut warp_slot: u64 = 0;

    for arg in input.args {
        match arg {
            ParsedTrialConfigArg::DeactivateFeatures(deactivate_features_arg) => {
                deactivate_features_arg.value.iter().for_each(|arg| {
                    deactivate_features.push(arg.to_token_stream().to_string());
                });
            }
            ParsedTrialConfigArg::WarpSlot(warp_slot_arg) => {
                warp_slot = warp_slot_arg.value.base10_parse::<u64>().unwrap();
            }
        }
    }

    Ok(crate::iteration::trial::TrialConfig {
        deactivate_features,
        warp_slot,
    })
}
