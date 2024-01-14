mod trial;

fn is_boomerang_test_attr(attr: &syn::Attribute) -> bool {
    let path = &attr.path();
    let segments: Vec<&syn::PathSegment> = path.segments.iter().collect();
    if segments.len() != 2 {
        return false;
    }
    &segments[0].ident == "boomerang" && &segments[1].ident == "test"
}

fn try_parse_trial_with_config(
    path_and_fn: (&String, &syn::ItemFn),
) -> syn::Result<Option<(trial::TrialConfig, trial::Trial)>> {
    let (_, item_fn) = path_and_fn;
    for attr in &item_fn.attrs {
        if is_boomerang_test_attr(attr) {
            // If the `#[boomerang::test]` attribute is present without arguments, then
            // `parse_args` will return `Ok(None)`, so we return the default config
            let trial_config = attr.parse_args::<trial::TrialConfig>().unwrap_or_default();
            let trial = trial::Trial::from(&path_and_fn);
            return Ok(Some((trial_config, trial)));
        }
    }
    Ok(None)
}

pub struct Iteration {
    config: trial::TrialConfig,
    trials: Vec<trial::Trial>,
}

impl Iteration {
    pub fn trials(&self) -> &Vec<trial::Trial> {
        &self.trials
    }

    pub fn parse_iterations() -> syn::Result<Vec<Self>> {
        Ok(crate::krate_parser::get_parsed_crate_context()
            .functions()
            .try_fold(Vec::<Iteration>::new(), |mut acc, func| {
                try_parse_trial_with_config(func).map(|trial| {
                    if let Some((config, trial)) = trial {
                        // Combine any trials with matching configs into the same iteration
                        if let Some(iteration) = acc.iter_mut().find(|i| i.config == config) {
                            iteration.trials.push(trial);
                        } else {
                            acc.push(Self {
                                config,
                                trials: vec![trial],
                            });
                        }
                    }
                    acc
                })
            })?)
    }
}

impl quote::ToTokens for Iteration {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend::<proc_macro2::TokenStream>(self.into());
    }
}

impl From<&Iteration> for proc_macro2::TokenStream {
    fn from(ast: &Iteration) -> Self {
        use quote::ToTokens;

        let config = &ast.config;
        let trials = &ast.trials;

        let config_tokens = config.to_token_stream();

        let all_generated_trial_names = trials
            .iter()
            .map(|trial| trial.generated_trial_name())
            .collect::<Vec<_>>();

        quote::quote! {
            (
                #config_tokens,
                &[
                    #( #all_generated_trial_names ),*
                ]
            )
        }
    }
}
