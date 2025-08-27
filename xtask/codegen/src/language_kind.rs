use std::str::FromStr;

use crate::kind_src::KindsSrc;
use crate::r_kinds_src::R_KINDS_SRC;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};

pub const LANGUAGE_PREFIXES: [&str; 1] = ["r_"];

#[derive(Debug, Eq, Copy, Clone, PartialEq)]
pub enum LanguageKind {
    R,
}

impl std::fmt::Display for LanguageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LanguageKind::R => write!(f, "r"),
        }
    }
}

pub const ALL_LANGUAGE_KIND: [LanguageKind; 1] = [LanguageKind::R];

impl FromStr for LanguageKind {
    type Err = String;

    fn from_str(kind: &str) -> Result<Self, Self::Err> {
        match kind {
            "r" => Ok(LanguageKind::R),
            _ => Err(format!("Language {kind} not supported, please use: `r`.")),
        }
    }
}

/// A helper macro to make it easier to define functions that return tokens for a specific language kind.
macro_rules! define_language_kind_function {
    ([$($kind:ident),*],$func:ident,$out:expr) => {
        pub(crate) fn $func(&self) -> TokenStream {
            match self {
                $( LanguageKind::$kind => {
                    // HACK: workaround for $kind$out adding an extra space between the two
                    let ident = format_ident!("{}{}", stringify!($kind), stringify!($out));
                    quote! { #ident }
                },)*
            }
        }
    }
}

/// A helper macro to define functions for each language kind to make it slightly less tedious to add new languages.
macro_rules! define_language_kind_functions {
    ([$($kind:ident),*]) => {
        define_language_kind_function!([$($kind),*], syntax_kind, SyntaxKind);
        define_language_kind_function!([$($kind),*], syntax_factory, SyntaxFactory);
        define_language_kind_function!([$($kind),*], syntax_node, SyntaxNode);
        define_language_kind_function!([$($kind),*], syntax_element, SyntaxElement);
        define_language_kind_function!([$($kind),*], syntax_token, SyntaxToken);
        define_language_kind_function!([$($kind),*], syntax_element_children, SyntaxElementChildren);
        define_language_kind_function!([$($kind),*], syntax_list, SyntaxList);
        define_language_kind_function!([$($kind),*], language, Language);
    }
}

impl LanguageKind {
    define_language_kind_functions!([R]);

    pub(crate) fn syntax_crate_ident(&self) -> Ident {
        Ident::new(self.syntax_crate_name().as_str(), Span::call_site())
    }

    pub fn formatter_crate_name(&self) -> String {
        format!("air_{self}_formatter")
    }

    pub fn syntax_crate_name(&self) -> String {
        format!("air_{self}_syntax")
    }

    pub fn factory_crate_name(&self) -> String {
        format!("air_{self}_factory")
    }

    pub fn kinds(&self) -> KindsSrc<'_> {
        match self {
            LanguageKind::R => R_KINDS_SRC,
        }
    }

    pub fn load_grammar(&self) -> &'static str {
        match self {
            LanguageKind::R => include_str!("../r.ungram"),
        }
    }
}
