enum ParsedTrialConfigArg {
    DeactivateFeatures(Vec<crate::parser::ParsedPathItem>),
    WarpSlot(crate::parser::ParsedIntItem),
}
impl syn::parse::Parse for ParsedTrialConfigArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Ident) {
            let ident = input.parse::<syn::Ident>()?;
            match ident.to_string().as_str() {
                "features_disabled" => Ok(Self::DeactivateFeatures(
                    crate::parser::parse_bracketed_list_arg::<crate::parser::ParsedPathItem>(
                        input,
                    )?,
                )),
                "warp_slot" => Ok(Self::WarpSlot(crate::parser::parse_singleton_arg::<
                    crate::parser::ParsedIntItem,
                >(input)?)),
                _ => Err(syn::Error::new(input.span(), "Unknown argument")),
            }
        } else {
            Err(syn::Error::new(input.span(), "Unknown argument"))
        }
    }
}

pub fn parse_trial_config(
    input: syn::parse::ParseStream,
) -> syn::Result<crate::iteration::trial::TrialConfig> {
    let mut features_disabled: Vec<syn::Path> = Vec::new();
    let mut warp_slot: u64 = 0;

    let args = crate::parser::parse_list::<ParsedTrialConfigArg>(input)?;

    for arg in args {
        match arg {
            ParsedTrialConfigArg::DeactivateFeatures(features_disabled_arg) => {
                features_disabled_arg.iter().for_each(|arg| {
                    features_disabled.push(arg.value());
                });
            }
            ParsedTrialConfigArg::WarpSlot(warp_slot_arg) => {
                warp_slot = warp_slot_arg.value::<u64>();
            }
        }
    }

    Ok(crate::iteration::trial::TrialConfig {
        features_disabled,
        warp_slot,
    })
}
