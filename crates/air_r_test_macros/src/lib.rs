use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::TokenTree;
use quote::quote;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

/// Expands to one `#[test] fn` per `*.md` under `directory`
///
/// ```ignore
/// gen_corpus_tests!("tests/corpus", crate::run_file);
/// ```
#[proc_macro]
pub fn gen_corpus_tests(input: TokenStream) -> TokenStream {
    let (directory, runner) = parse_arguments(input.into());

    // Cargo sets this to the manifest directory of the crate being compiled,
    // which is the caller rather than this crate
    let manifest = std::env::var("CARGO_MANIFEST_DIR").expect("cargo sets CARGO_MANIFEST_DIR");
    let root = PathBuf::from(manifest).join(directory);

    let mut module = Module::default();
    collect(&root, &mut module);
    module.generate(&runner).into()
}

fn parse_arguments(input: TokenStream2) -> (String, TokenStream2) {
    let mut tokens = input.into_iter();

    let directory = match tokens.next() {
        Some(TokenTree::Literal(literal)) => literal.to_string().trim_matches('"').to_string(),
        _ => panic!("expected a string literal naming the corpus directory"),
    };

    match tokens.next() {
        Some(TokenTree::Punct(punct)) if punct.as_char() == ',' => {}
        _ => panic!("expected `,` after the corpus directory"),
    }

    (directory, tokens.collect())
}

#[derive(Default)]
struct Module {
    children: Vec<(String, Module)>,
    tests: Vec<(String, String)>,
}

impl Module {
    fn generate(self, runner: &TokenStream2) -> TokenStream2 {
        let modules = self.children.into_iter().map(|(name, child)| {
            let name = Ident::new(&name, Span::call_site());
            let body = child.generate(runner);
            quote! { mod #name { #body } }
        });

        let tests = self.tests.into_iter().map(|(name, path)| {
            let name = Ident::new(&name, Span::call_site());
            quote! {
                #[test]
                fn #name() {
                    #runner(std::path::Path::new(#path));
                }
            }
        });

        quote! { #(#modules)* #(#tests)* }
    }
}

fn collect(directory: &Path, module: &mut Module) {
    let mut entries: Vec<PathBuf> = fs::read_dir(directory)
        .unwrap_or_else(|err| panic!("cannot read {}: {err}", directory.display()))
        .map(|entry| entry.expect("cannot read directory entry").path())
        .collect();

    // read_dir() order is not stable across filesystems
    entries.sort();

    for path in entries {
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .expect("corpus paths are UTF-8");

        if path.is_dir() {
            let mut child = Module::default();
            let name = identifier(name);
            collect(&path, &mut child);
            module.children.push((name, child));
        } else if name.ends_with(".md") && !name.ends_with(".snap.md") {
            let test = identifier(name.trim_end_matches(".md"));
            module.tests.push((test, path.display().to_string()));
        }
    }
}

/// A file or directory name as a Rust identifier
fn identifier(name: &str) -> String {
    let mut identifier: String = name
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect();

    if identifier.starts_with(|c: char| c.is_ascii_digit()) || is_keyword(&identifier) {
        identifier.push('_');
    }

    identifier
}

fn is_keyword(name: &str) -> bool {
    matches!(
        name,
        "as" | "break"
            | "const"
            | "continue"
            | "crate"
            | "dyn"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
    )
}
