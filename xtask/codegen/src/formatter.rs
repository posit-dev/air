use std::{
    collections::{BTreeMap, BTreeSet, HashSet, VecDeque},
    env,
    fs::{create_dir_all, read_dir, remove_file, File},
    io::Write,
    path::{Path, PathBuf},
};

use git2::{Repository, Status, StatusOptions};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use xtask::project_root;

use crate::ast::load_ast;
use crate::language_kind::{LanguageKind, ALL_LANGUAGE_KIND};

struct GitRepo {
    repo: Repository,
    allow_staged: bool,
    staged: HashSet<PathBuf>,
    dirty: HashSet<PathBuf>,
}

impl GitRepo {
    fn open() -> Self {
        let root = project_root();
        let repo = Repository::discover(&root).expect("failed to open git repo");

        let mut allow_staged = false;
        let mut allow_dirty = false;
        for arg in env::args() {
            match arg.as_str() {
                "--allow-staged" => {
                    allow_staged = true;
                }
                "--allow-dirty" => {
                    allow_dirty = true;
                }
                _ => {}
            }
        }

        let mut repo_opts = StatusOptions::new();
        repo_opts.include_ignored(false);

        let statuses = repo
            .statuses(Some(&mut repo_opts))
            .expect("failed to read repository status");

        let mut staged = HashSet::new();
        let mut dirty = HashSet::new();

        for status in statuses.iter() {
            if let Some(path) = status.path() {
                match status.status() {
                    Status::CURRENT => (),
                    Status::INDEX_NEW
                    | Status::INDEX_MODIFIED
                    | Status::INDEX_DELETED
                    | Status::INDEX_RENAMED
                    | Status::INDEX_TYPECHANGE => {
                        if !allow_staged {
                            staged.insert(root.join(path));
                        }
                    }
                    _ => {
                        if !allow_dirty {
                            dirty.insert(root.join(path));
                        }
                    }
                };
            }
        }

        drop(statuses);

        Self {
            repo,
            allow_staged,
            staged,
            dirty,
        }
    }

    fn check_path(&self, path: &Path) {
        if self.dirty.contains(path) {
            panic!("Codegen would overwrite '{}' but it has uncommitted changes. Commit the file to git, or pass --allow-dirty to the command to proceed anyway", path.display());
        }
        if self.staged.contains(path) {
            panic!("Codegen would overwrite '{}' but it has uncommitted changes. Commit the file to git, or pass --allow-staged to the command to proceed anyway", path.display());
        }
    }

    fn stage_paths(&self, paths: &[PathBuf]) {
        // Do not overwrite a version of the file
        // that's potentially already staged
        if self.allow_staged {
            return;
        }

        let root = project_root();
        self.repo
            .index()
            .expect("could not open index for git repository")
            .update_all(
                paths.iter().map(|path| {
                    path.strip_prefix(&root).unwrap_or_else(|err| {
                        panic!(
                            "path '{}' is not inside of project '{}': {}",
                            path.display(),
                            root.display(),
                            err,
                        )
                    })
                }),
                None,
            )
            .expect("failed to stage updated files");
    }
}

struct ModuleIndex {
    root: PathBuf,
    modules: BTreeMap<PathBuf, BTreeSet<String>>,
    unused_files: HashSet<PathBuf>,
}

impl ModuleIndex {
    fn new(root: PathBuf) -> Self {
        let mut unused_files = HashSet::new();
        let mut queue: VecDeque<_> = NodeDialect::all()
            .iter()
            .map(|dialect| root.join(dialect.as_str()))
            .collect();

        while let Some(dir) = queue.pop_front() {
            if !dir.exists() {
                continue;
            }

            let iter = read_dir(&dir)
                .unwrap_or_else(|err| panic!("failed to read '{}': {}", dir.display(), err));

            for entry in iter {
                let entry = entry.expect("failed to read DirEntry");

                let path = entry.path();
                let file_type = entry.file_type().unwrap_or_else(|err| {
                    panic!("failed to read file type of '{}': {}", path.display(), err)
                });

                if file_type.is_dir() {
                    queue.push_back(path);
                    continue;
                }

                unused_files.insert(path);
            }
        }

        Self {
            root,
            modules: BTreeMap::default(),
            unused_files,
        }
    }

    /// Add a new module to the index
    fn insert(&mut self, repo: &GitRepo, path: &Path) {
        self.unused_files.remove(path);

        // Walk up from the module file towards the root
        let mut parent = path.parent();
        let mut file_stem = path.file_stem();

        while let (Some(path), Some(stem)) = (parent, file_stem) {
            repo.check_path(&path.join("mod.rs"));

            // Insert each module into its parent
            let stem = stem.to_str().unwrap().to_owned();
            self.modules.entry(path.into()).or_default().insert(stem);

            parent = path.parent();
            file_stem = path.file_stem();

            // Stop at the root directory
            if parent == Some(&self.root) {
                break;
            }
        }
    }

    /// Create all the mod.rs files needed to import
    /// all the modules in the index up to the root
    fn print(mut self, stage: &mut Vec<PathBuf>) {
        for (path, imports) in self.modules {
            let mut content = String::new();

            let stem = path.file_stem().unwrap().to_str().unwrap();
            for import in imports {
                // Clippy complains about child modules having the same
                // names as their parent, eg. js/name/name.rs
                if import == stem {
                    content.push_str("#[allow(clippy::module_inception)]\n");
                }

                content.push_str("pub(crate) mod ");
                content.push_str(&import);
                content.push_str(";\n");
            }

            let content = xtask::reformat_with_command(content, "cargo codegen formatter").unwrap();

            let path = path.join("mod.rs");
            let mut file = File::create(&path).unwrap();
            file.write_all(content.as_bytes()).unwrap();
            drop(file);

            self.unused_files.remove(&path);
            stage.push(path);
        }

        for file in self.unused_files {
            remove_file(&file)
                .unwrap_or_else(|err| panic!("failed to delete '{}': {}", file.display(), err));
            stage.push(file);
        }
    }
}

enum NodeKind {
    Node,
    List { separated: bool },
    Bogus,
    Union { variants: Vec<String> },
}

pub fn generate_formatters() {
    let repo = GitRepo::open();

    for language in ALL_LANGUAGE_KIND {
        generate_formatter(&repo, language);
    }
}

fn generate_formatter(repo: &GitRepo, language_kind: LanguageKind) {
    let ast = load_ast(language_kind);

    // Store references to all the files created by the codegen
    // script to build the module import files
    let formatter_crate_path = project_root()
        .join("crates")
        .join(language_kind.formatter_crate_name());

    if !formatter_crate_path.exists() {
        return;
    }

    let mut modules = ModuleIndex::new(formatter_crate_path.join("src"));
    let mut format_impls =
        BoilerplateImpls::new(formatter_crate_path.join("src/generated.rs"), language_kind);

    // Build an unified iterator over all the AstNode types
    let names = ast
        .nodes
        .into_iter()
        .map(|node| (NodeKind::Node, node.name))
        .chain(ast.lists.into_iter().map(|(name, node)| {
            (
                NodeKind::List {
                    separated: node.separator.is_some(),
                },
                name,
            )
        }))
        .chain(ast.bogus.into_iter().map(|name| (NodeKind::Bogus, name)))
        .chain(ast.unions.into_iter().map(|node| {
            (
                NodeKind::Union {
                    variants: node.variants,
                },
                node.name,
            )
        }));

    let mut stage = Vec::new();

    // Create a default implementation for these nodes only if
    // the file doesn't already exist
    for (kind, name) in names {
        let module = name_to_module(&kind, &name, language_kind);
        let path = module.as_path();
        modules.insert(repo, &path);

        let node_id = Ident::new(&name, Span::call_site());
        let _node_fields_id = Ident::new(&format!("{name}Fields"), Span::call_site());
        let format_id = Ident::new(&format!("Format{name}"), Span::call_site());

        let qualified_format_id = {
            let dialect = Ident::new(module.dialect.as_str(), Span::call_site());
            let concept = Ident::new(module.concept.as_str(), Span::call_site());
            let module = Ident::new(&module.name, Span::call_site());
            quote! { crate::#dialect::#concept::#module::#format_id }
        };

        format_impls.push(&kind, &node_id, &qualified_format_id);

        // Union nodes except for AnyFunction and AnyClass have a generated
        // implementation, the codegen will always overwrite any existing file
        let allow_overwrite = matches!(kind, NodeKind::Union { .. });

        if !allow_overwrite && path.exists() {
            continue;
        }

        let dir = path.parent().unwrap();
        create_dir_all(dir).unwrap();

        repo.check_path(&path);

        let syntax_crate_ident = language_kind.syntax_crate_ident();
        let formatter_ident = language_kind.formatter_ident();
        let formatter_context_ident = language_kind.format_context_ident();

        // Generate a default implementation of Format/FormatNode using format_list on
        // non-separated lists, format on the wrapped node for unions and
        // format_verbatim for all the other nodes
        let tokens = match kind {
            NodeKind::List { separated: false } => quote! {
                use crate::prelude::*;
                use #syntax_crate_ident::#node_id;

                #[derive(Debug, Clone, Default)]
                pub(crate) struct #format_id;

                impl FormatRule<#node_id> for #format_id {
                    type Context = #formatter_context_ident;

                    fn fmt(&self, node: &#node_id, f: &mut #formatter_ident) -> FormatResult<()> {
                        f.join().entries(node.iter().formatted()).finish()
                    }
                }
            },
            NodeKind::List { .. } => quote! {
                use crate::prelude::*;
                use #syntax_crate_ident::#node_id;

                #[derive(Debug, Clone, Default)]
                pub(crate) struct #format_id;

                impl FormatRule<#node_id> for #format_id {
                    type Context = #formatter_context_ident;

                    fn fmt(&self, node: &#node_id, f: &mut #formatter_ident) -> FormatResult<()> {
                        format_verbatim_node(node.syntax()).fmt(f)
                    }
                }
            },
            NodeKind::Node => {
                quote! {
                    use crate::prelude::*;

                    use biome_rowan::AstNode;
                    use #syntax_crate_ident::#node_id;

                    #[derive(Debug, Clone, Default)]
                    pub(crate) struct #format_id;

                    impl FormatNodeRule<#node_id> for #format_id {
                        fn fmt_fields(&self, node: &#node_id, f: &mut #formatter_ident) -> FormatResult<()> {
                            format_verbatim_node(node.syntax()).fmt(f)
                        }
                    }
                }
            }
            NodeKind::Bogus => {
                quote! {
                    use crate::FormatBogusNodeRule;
                    use #syntax_crate_ident::#node_id;

                    #[derive(Debug, Clone, Default)]
                    pub(crate) struct #format_id;

                    impl FormatBogusNodeRule<#node_id> for #format_id {
                    }
                }
            }
            NodeKind::Union { variants } => {
                // For each variant of the union call to_format_element on the wrapped node
                let match_arms: Vec<_> = variants
                    .into_iter()
                    .map(|variant| {
                        let variant = Ident::new(&variant, Span::call_site());
                        quote! { #node_id::#variant(node) => node.format().fmt(f), }
                    })
                    .collect();

                quote! {
                    use crate::prelude::*;
                    use #syntax_crate_ident::#node_id;

                    #[derive(Debug, Clone, Default)]
                    pub(crate) struct #format_id;

                    impl FormatRule<#node_id> for #format_id {
                        type Context = #formatter_context_ident;

                        fn fmt(&self, node: &#node_id, f: &mut #formatter_ident) -> FormatResult<()> {
                            match node {
                                #( #match_arms )*
                            }
                        }
                    }
                }
            }
        };

        let tokens = if allow_overwrite {
            xtask::reformat_with_command(tokens, "cargo codegen formatter").unwrap()
        } else {
            xtask::reformat_without_preamble(tokens).unwrap()
        };

        let mut file = File::create(&path).unwrap();
        file.write_all(tokens.as_bytes()).unwrap();
        drop(file);

        stage.push(path);
    }

    modules.print(&mut stage);
    format_impls.print(&mut stage);

    repo.stage_paths(&stage);
}

struct BoilerplateImpls {
    language: LanguageKind,
    path: PathBuf,
    impls: Vec<TokenStream>,
}

impl BoilerplateImpls {
    fn new(file_name: PathBuf, language: LanguageKind) -> Self {
        Self {
            path: file_name,
            impls: vec![],
            language,
        }
    }

    fn push(&mut self, kind: &NodeKind, node_id: &Ident, format_id: &TokenStream) {
        let syntax_crate_ident = self.language.syntax_crate_ident();
        let formatter_ident = self.language.formatter_ident();
        let formatter_context_ident = self.language.format_context_ident();

        let format_rule_impl = match kind {
            NodeKind::List { .. } | NodeKind::Union { .. } => quote!(),
            kind => {
                let rule = if matches!(kind, NodeKind::Bogus) {
                    Ident::new("FormatBogusNodeRule", Span::call_site())
                } else {
                    Ident::new("FormatNodeRule", Span::call_site())
                };

                quote! {
                    impl FormatRule<#syntax_crate_ident::#node_id> for #format_id {
                       type Context = #formatter_context_ident;
                        #[inline(always)]
                        fn fmt(&self, node: &#syntax_crate_ident::#node_id, f: &mut #formatter_ident) -> FormatResult<()> {
                            #rule::<#syntax_crate_ident::#node_id>::fmt(self, node, f)
                        }
                    }
                }
            }
        };

        self.impls.push(quote! {
            #format_rule_impl

            impl AsFormat<#formatter_context_ident> for #syntax_crate_ident::#node_id {
                type Format<'a> = FormatRefWithRule<'a, #syntax_crate_ident::#node_id, #format_id>;

                fn format(&self) -> Self::Format<'_> {
                    #![allow(clippy::default_constructed_unit_structs)]
                    FormatRefWithRule::new(self, #format_id::default())
                }
            }

            impl IntoFormat<#formatter_context_ident> for #syntax_crate_ident::#node_id {
                type Format = FormatOwnedWithRule<#syntax_crate_ident::#node_id, #format_id>;

                fn into_format(self) -> Self::Format {
                    #![allow(clippy::default_constructed_unit_structs)]
                    FormatOwnedWithRule::new(self, #format_id::default())
                }
            }
        });
    }

    fn print(self, stage: &mut Vec<PathBuf>) {
        let impls = self.impls;

        let formatter_ident = self.language.formatter_ident();
        let formatter_context_ident = self.language.format_context_ident();

        let tokens = quote! {
            use crate::{AsFormat, IntoFormat, FormatNodeRule, FormatBogusNodeRule, #formatter_ident, #formatter_context_ident};
            use biome_formatter::{FormatRefWithRule, FormatOwnedWithRule, FormatRule, FormatResult};

            #( #impls )*
        };

        let content = xtask::reformat_with_command(tokens, "cargo codegen formatter").unwrap();
        let mut file = File::create(&self.path).unwrap();
        file.write_all(content.as_bytes()).unwrap();

        stage.push(self.path);
    }
}

enum NodeDialect {
    R,
}

impl NodeDialect {
    fn all() -> &'static [NodeDialect] {
        &[NodeDialect::R]
    }

    fn as_str(&self) -> &'static str {
        match self {
            NodeDialect::R => "r",
        }
    }

    fn from_str(name: &str) -> NodeDialect {
        match name {
            "R" => NodeDialect::R,
            _ => {
                panic!("missing prefix {name}");
            }
        }
    }
}

enum NodeConcept {
    Bogus,
    List,
    Union,
    /// Auxiliary (everything else)
    Auxiliary,
    // TODO(air): Add concepts that make sense for us, which
    // map to folder names that we want in the formatter
    // Expression,
    // Statement,
    // Assignment,
    // Value,
}

impl NodeConcept {
    fn as_str(&self) -> &'static str {
        match self {
            // NodeConcept::Expression => "expressions",
            // NodeConcept::Statement => "statements",
            // NodeConcept::Assignment => "assignments",
            NodeConcept::Bogus => "bogus",
            NodeConcept::List => "lists",
            NodeConcept::Union => "any",
            NodeConcept::Auxiliary => "auxiliary",
            // NodeConcept::Value => "value",
        }
    }
}

struct NodeModuleInformation {
    language: LanguageKind,
    dialect: NodeDialect,
    concept: NodeConcept,
    name: String,
}

impl NodeModuleInformation {
    fn as_path(&self) -> PathBuf {
        project_root()
            .join("crates")
            .join(self.language.formatter_crate_name())
            .join("src")
            .join(self.dialect.as_str())
            .join(self.concept.as_str())
            .join(format!("{}.rs", self.name))
    }
}

fn get_node_concept(
    kind: &NodeKind,
    _dialect: &NodeDialect,
    language: &LanguageKind,
    _name: &str,
) -> NodeConcept {
    if matches!(kind, NodeKind::Bogus) {
        NodeConcept::Bogus
    } else if matches!(kind, NodeKind::List { .. }) {
        NodeConcept::List
    } else if matches!(kind, NodeKind::Union { .. }) {
        NodeConcept::Union
    } else {
        match language {
            // TODO(air): implement additional concepts to clean up folder names
            LanguageKind::R => NodeConcept::Auxiliary,
        }
    }
}

/// Convert an AstNode name to a path / Rust module name
fn name_to_module(kind: &NodeKind, in_name: &str, language: LanguageKind) -> NodeModuleInformation {
    let mut upper_case_indices = in_name.match_indices(|c: char| c.is_uppercase());

    assert!(matches!(upper_case_indices.next(), Some((0, _))));

    let (second_upper_start, _) = upper_case_indices.next().expect("Node name malformed");
    let (mut dialect_prefix, mut name) = in_name.split_at(second_upper_start);

    // AnyJsX
    if dialect_prefix == "Any" {
        let (third_upper_start, _) = upper_case_indices.next().expect("Node name malformed");
        (dialect_prefix, name) = name.split_at(third_upper_start - dialect_prefix.len());
    }

    let dialect = NodeDialect::from_str(dialect_prefix);

    // Classify nodes by concept
    let concept = get_node_concept(kind, &dialect, &language, name);

    // Convert the names from CamelCase to snake_case
    let mut stem = String::new();
    for (index, char) in name.chars().enumerate() {
        if char.is_lowercase() {
            stem.push(char);
        } else {
            if index > 0 {
                stem.push('_');
            }
            for char in char.to_lowercase() {
                stem.push(char);
            }
        }
    }

    // "type" and "enum" are Rust keywords, add the "ts_"
    // prefix to these modules to avoid parsing errors
    let stem = match stem.as_str() {
        "type" => String::from("ts_type"),
        "enum" => String::from("ts_enum"),
        _ => stem,
    };

    NodeModuleInformation {
        name: stem,
        language,
        dialect,
        concept,
    }
}

impl LanguageKind {
    fn formatter_ident(&self) -> Ident {
        let name = match self {
            LanguageKind::R => "RFormatter",
        };

        Ident::new(name, Span::call_site())
    }

    fn format_context_ident(&self) -> Ident {
        let name = match self {
            LanguageKind::R => "RFormatContext",
        };

        Ident::new(name, Span::call_site())
    }
}
