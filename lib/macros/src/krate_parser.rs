//! Taken from `https://github.com/metaplex-foundation/shank`.
//!
//! This module takes code from `shank-macro-impl` to parse the crate's source
//! code.
//!
//! Because `shank-macro-impl` is using an older version of `syn`, we need to
//! use the same version of `syn` to parse the crate's source code.
//!
//! If `shank-macro-impl` is updated to use a newer version of `syn`, then we
//! can update this module to use `shank-macro-impl` directly.
//!
//! As mentioned in the documentation in `shank-macro-impl`, this module is
//! adapted from:
//! https://github.com/project-serum/anchor/blob/d8d720067dd6e2a3bec50207b84008276c914732/lang/syn/src/parser/context.rs

#[derive(Copy, Clone)]
pub struct ModuleContext<'krate> {
    pub detail: &'krate ParsedModule,
}

#[derive(Debug)]
pub struct ParsedModule {
    pub name: String,
    pub file: std::path::PathBuf,
    pub path: String,
    pub items: Vec<syn::Item>,
}

impl ParsedModule {
    fn parse_recursive(
        root: &std::path::Path,
    ) -> Result<std::collections::BTreeMap<String, ParsedModule>, anyhow::Error> {
        let root_content = std::fs::read_to_string(root)?;
        Self::parse_content_recursive(root, root_content)
    }

    fn parse_content_recursive(
        root: &std::path::Path,
        root_content: String,
    ) -> Result<std::collections::BTreeMap<String, ParsedModule>, anyhow::Error> {
        let mut modules = std::collections::BTreeMap::new();

        let root_file = syn::parse_file(&root_content)?;
        let root_mod = Self::new(
            String::new(),
            root.to_owned(),
            "crate".to_owned(),
            root_file.items,
        );

        struct UnparsedModule {
            file: std::path::PathBuf,
            path: String,
            name: String,
            item: syn::ItemMod,
        }

        let mut unparsed = root_mod
            .submodules()
            .map(|item| UnparsedModule {
                file: root_mod.file.clone(),
                path: root_mod.path.clone(),
                name: item.ident.to_string(),
                item: item.clone(),
            })
            .collect::<Vec<_>>();

        while let Some(to_parse) = unparsed.pop() {
            let path = format!("{}::{}", to_parse.path, to_parse.name);
            let module = Self::from_item_mod(&to_parse.file, &path, to_parse.item)?;

            unparsed.extend(module.submodules().map(|item| UnparsedModule {
                item: item.clone(),
                file: module.file.clone(),
                path: module.path.clone(),
                name: item.ident.to_string(),
            }));
            modules.insert(path, module);
        }

        modules.insert(root_mod.name.clone(), root_mod);

        Ok(modules)
    }

    fn from_item_mod(
        parent_file: &std::path::Path,
        parent_path: &str,
        item: syn::ItemMod,
    ) -> syn::Result<Self> {
        Ok(match item.content {
            Some((_, items)) => {
                // The module content is within the parent file being parsed
                Self::new(
                    parent_path.to_owned(),
                    parent_file.to_owned(),
                    item.ident.to_string(),
                    items,
                )
            }
            None => {
                // The module is referencing some other file, so we need to load that
                // to parse the items it has.
                let parent_dir = parent_file.parent().unwrap();
                let parent_filename = parent_file.file_stem().unwrap().to_str().unwrap();
                let parent_mod_dir = parent_dir.join(parent_filename);

                let possible_file_paths = vec![
                    parent_dir.join(format!("{}.rs", item.ident)),
                    parent_dir.join(format!("{}/mod.rs", item.ident)),
                    parent_mod_dir.join(format!("{}.rs", item.ident)),
                    parent_mod_dir.join(format!("{}/mod.rs", item.ident)),
                ];

                let mod_file_path = possible_file_paths
                    .into_iter()
                    .find(|p| p.exists())
                    .ok_or_else(|| syn::Error::new_spanned(&item, "could not find file"))?;
                let mod_file_content = std::fs::read_to_string(&mod_file_path)
                    .map_err(|_| syn::Error::new_spanned(&item, "could not read file"))?;
                let mod_file = syn::parse_file(&mod_file_content)?;

                Self::new(
                    parent_path.to_owned(),
                    mod_file_path,
                    item.ident.to_string(),
                    mod_file.items,
                )
            }
        })
    }

    fn new(path: String, file: std::path::PathBuf, name: String, items: Vec<syn::Item>) -> Self {
        Self {
            name,
            file,
            path,
            items,
        }
    }

    fn submodules(&self) -> impl Iterator<Item = &syn::ItemMod> {
        self.items.iter().filter_map(|i| match i {
            syn::Item::Mod(item) => Some(item),
            _ => None,
        })
    }

    fn functions(&self) -> impl Iterator<Item = &syn::ItemFn> {
        self.items.iter().filter_map(|i| match i {
            syn::Item::Fn(item) => Some(item),
            _ => None,
        })
    }
}

#[derive(Debug)]
pub struct CrateContext {
    modules: std::collections::BTreeMap<String, ParsedModule>,
}

impl CrateContext {
    pub fn functions(&self) -> impl Iterator<Item = (&String, &syn::ItemFn)> {
        self.modules
            .iter()
            .flat_map(|(path, module)| module.functions().map(move |f| (path, f)))
    }

    fn parse(root: &std::path::Path) -> Result<Self, anyhow::Error> {
        Ok(CrateContext {
            modules: ParsedModule::parse_recursive(root)?,
        })
    }
}

pub fn get_parsed_crate_context() -> CrateContext {
    // TODO: This is obviously not dynamic yet.
    // We need to figure out how to get the crate root path dynamically.
    //
    // One option is to use `proc_macro_span` to get the `SourceFile` for a
    // call site.
    // However, this is an unstable feature and requires the nightly compiler.
    //
    // Another option is to potentially use `cargo_metadata`, however it does
    // not appear to pander specifically to navigating test folders.
    //
    // Ideally, since this execution path is happening from the
    // `#[boomerang::main]` macro, and there's only one such macro invocation
    // in any given test suite, we need to devise a way to detect the file path
    // of the `#[boomerang::main]` macro invocation.
    let root = std::env::current_dir()
        .unwrap()
        .join("tests")
        .join("address-lookup-table")
        .join("tests")
        .join("main.rs");
    CrateContext::parse(&root).expect(
        "Failed to detect `tests/main.rs`. \
            Make sure your `#[boomerang::main]` function is in `tests/main.rs`",
    )
}
