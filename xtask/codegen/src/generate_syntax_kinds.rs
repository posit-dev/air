use crate::kind_src::KindsSrc;
use crate::language_kind::LanguageKind;
use crate::Result;
use biome_string_case::Case;
use proc_macro2::{Literal, Punct, Spacing};
use quote::{format_ident, quote};

pub fn generate_syntax_kinds(grammar: KindsSrc, language_kind: LanguageKind) -> Result<String> {
    let syntax_kind = language_kind.syntax_kind();
    let punctuation_values = grammar.punct.iter().map(|(token, _name)| {
        // These tokens, when parsed to proc_macro2::TokenStream, generates a stream of bytes
        // that can't be recognized by [quote].
        // Hence, they need to be thread differently
        // NOTE(air): we also add `[[`, `]]`, and `\\` to this list.
        if "{}[]()`".contains(token) {
            let c = token.chars().next().unwrap();
            quote! { #c }
        } else if matches!(*token, "$=" | "$_" | "[[" | "]]" | "\\") {
            let token = Literal::string(token);
            quote! { #token }
        } else {
            let cs = token.chars().map(|c| Punct::new(c, Spacing::Joint));
            quote! { #(#cs)* }
        }
    });
    let punctuation_strings = grammar.punct.iter().map(|(token, _name)| token);

    let punctuation = grammar
        .punct
        .iter()
        .map(|(_token, name)| format_ident!("{}", name))
        .collect::<Vec<_>>();

    // color-profile
    let all_keywords = &grammar.keywords;
    // color-profile => "color-profile"
    let all_keyword_strings = all_keywords.iter().map(|name| (*name).to_string());
    let all_keyword_to_strings = all_keywords.iter().map(|name| (*name).to_string()).clone();
    // we need to replace "-" with "_" for the keywords
    // e.g. we have `color-profile` in css but it's an invalid ident in rust code
    // color-profile => "color_profile"
    // also mark uppercase differently from lowercase
    // e.g. "query" => "QUERY", "QUERY" => "QUERY_UPPERCASE"
    let all_keywords_values = all_keywords
        .iter()
        .map(|kw| {
            let kw = kw.replace('-', "_");
            if kw.chars().all(|c| c.is_uppercase()) {
                "UPPER_".to_string() + kw.as_str()
            } else {
                kw
            }
        })
        .collect::<Vec<_>>();
    // "color_profile" => COLOR_PROFILE_KW
    let full_keywords = all_keywords_values
        .iter()
        .map(|kw| format_ident!("{}_KW", Case::Constant.convert(kw)))
        .collect::<Vec<_>>();

    // "color_profile" => color_profile
    let all_keywords_idents = all_keywords_values
        .iter()
        .map(|kw| format_ident!("{}", kw))
        .collect::<Vec<_>>();

    let literals = grammar
        .literals
        .iter()
        .map(|name| format_ident!("{}", name))
        .collect::<Vec<_>>();

    let tokens = grammar
        .tokens
        .iter()
        .map(|name| format_ident!("{}", name))
        .collect::<Vec<_>>();

    let nodes = grammar
        .nodes
        .iter()
        .map(|name| format_ident!("{}", name))
        .collect::<Vec<_>>();

    let lists = grammar
        .nodes
        .iter()
        .filter_map(|name| {
            if name.ends_with("_LIST") {
                Some(format_ident!("{}", name))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let syntax_kind_impl = match language_kind {
        LanguageKind::R => {
            quote! {
                pub const fn to_string(&self) -> Option<&'static str> {
                    let tok = match self {
                        #(#punctuation => #punctuation_strings,)*
                        #(#full_keywords => #all_keyword_to_strings,)*
                        R_STRING_VALUE => "string value",
                        _ => return None,
                    };
                    Some(tok)
                }
            }
        }
    };

    let ast = quote! {
        #![allow(clippy::all)]
        #![allow(bad_style, missing_docs, unreachable_pub)]
        /// The kind of syntax node, e.g. `IDENT`, `FUNCTION_KW`, or `FOR_STMT`.
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        #[repr(u16)]
        pub enum #syntax_kind {
            // Technical SyntaxKinds: they appear temporally during parsing,
            // but never end up in the final tree
            #[doc(hidden)]
            TOMBSTONE,
            /// Marks the end of the file. May have trivia attached
            EOF,
            /// Any Unicode BOM character that may be present at the start of
            /// a file.
            UNICODE_BOM,
            #(#punctuation,)*
            #(#full_keywords,)*
            #(#literals,)*
            #(#tokens,)*
            #(#nodes,)*

            // Technical kind so that we can cast from u16 safely
            #[doc(hidden)]
            __LAST,
        }
        use self::#syntax_kind::*;

        impl #syntax_kind {
            pub const fn is_punct(self) -> bool {
                match self {
                    #(#punctuation)|* => true,
                    _ => false,
                }
            }

            pub const fn is_literal(self) -> bool {
                match self {
                    #(#literals)|* => true,
                    _ => false,
                }
            }

            pub const fn is_list(self) -> bool {
                match self {
                    #(#lists)|* => true,
                    _ => false,
                }
            }

            pub fn from_keyword(ident: &str) -> Option<#syntax_kind> {
                let kw = match ident {
                    #(#all_keyword_strings => #full_keywords,)*
                    _ => return None,
                };
                Some(kw)
            }

            #syntax_kind_impl

        }

        /// Utility macro for creating a SyntaxKind through simple macro syntax
        #[macro_export]
        macro_rules! T {
            #([#punctuation_values] => { $crate::#syntax_kind::#punctuation };)*
            #([#all_keywords_idents] => { $crate::#syntax_kind::#full_keywords };)*
            [ident] => { $crate::#syntax_kind::IDENT };
            [EOF] => { $crate::#syntax_kind::EOF };
            [UNICODE_BOM] => { $crate::#syntax_kind::UNICODE_BOM };
            [#] => { $crate::#syntax_kind::HASH };
        }
    };

    xtask::reformat(ast)
}
