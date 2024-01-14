mod krate_parser;
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
    item_fn: &syn::ItemFn,
) -> syn::Result<Option<(trial::TrialConfig, trial::Trial)>> {
    for attr in &item_fn.attrs {
        if is_boomerang_test_attr(attr) {
            let trial_config = attr.parse_args::<trial::TrialConfig>()?;
            let trial = trial::Trial::from(item_fn);
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
    pub fn parse_iterations() -> syn::Result<Vec<Self>> {
        let all_item_fns = krate_parser::parse_all_item_fns()?;

        let all_trials = all_item_fns
            .iter()
            .map(try_parse_trial_with_config)
            .collect::<syn::Result<Vec<_>>>()?;

        let iterations = all_trials.into_iter().filter_map(|trial| trial).fold(
            Vec::<Iteration>::new(),
            |mut acc, (config, trial)| {
                if let Some(iteration) = acc.iter_mut().find(|i| (**i).config == config) {
                    iteration.trials.push(trial);
                } else {
                    acc.push(Self {
                        config,
                        trials: vec![trial],
                    });
                }
                acc
            },
        );

        Ok(iterations)
    }
}
