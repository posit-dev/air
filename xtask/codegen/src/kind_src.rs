use crate::language_kind::{LanguageKind, LANGUAGE_PREFIXES};
use quote::format_ident;
use std::collections::BTreeMap;

pub struct KindsSrc<'a> {
    /// Special characters of the language. Usually these are parenthesis, dots, commas, etc.
    pub punct: &'a [(&'a str, &'a str)],
    /// Reserved keywords of the language
    pub keywords: &'a [&'a str],
    /// Literals are special nodes that holds some **values** inside the language, for example: strings, numbers, etc.
    pub literals: &'a [&'a str],
    /// Whitespaces, comments, identifiers, etc.
    pub tokens: &'a [&'a str],
    /// Nodes of the CST. Usually you want to map these names from the `.ungram` file. For example:
    ///
    /// HtmlAttribute -> HTML_ATTRIBUTE
    /// HtmlBogus -> HTML_BOGUS
    pub nodes: &'a [&'a str],
}

#[derive(Default, Debug)]
pub struct AstSrc {
    pub nodes: Vec<AstNodeSrc>,
    pub unions: Vec<AstEnumSrc>,
    pub lists: BTreeMap<String, AstListSrc>,
    pub bogus: Vec<String>,
}

impl AstSrc {
    pub fn push_list(&mut self, name: &str, src: AstListSrc) {
        self.lists.insert(String::from(name), src);
    }

    pub fn lists(&self) -> std::collections::btree_map::Iter<String, AstListSrc> {
        self.lists.iter()
    }

    pub fn is_list(&self, name: &str) -> bool {
        self.lists.contains_key(name)
    }

    /// Sorts all nodes, enums, etc. for a stable code gen result
    pub fn sort(&mut self) {
        // No need to sort lists, they're stored in a btree
        self.nodes.sort_unstable_by(|a, b| a.name.cmp(&b.name));
        self.unions.sort_unstable_by(|a, b| a.name.cmp(&b.name));
        self.bogus.sort_unstable();

        for union in self.unions.iter_mut() {
            union.variants.sort_unstable();
        }
    }
}

#[derive(Debug)]
pub struct AstListSrc {
    pub element_name: String,
    pub separator: Option<AstListSeparatorConfiguration>,
}

#[derive(Debug)]
pub struct AstListSeparatorConfiguration {
    /// Name of the separator token
    pub separator_token: String,
    /// Whatever the list allows a trailing comma or not
    pub allow_trailing: bool,
}

#[derive(Debug)]
pub struct AstNodeSrc {
    #[allow(dead_code)]
    pub documentation: Vec<String>,
    pub name: String,
    // pub traits: Vec<String>,
    pub fields: Vec<Field>,
    /// Whether the fields of the node should be ordered dynamically using a
    /// slot map for accesses.
    pub dynamic: bool,
}

#[derive(Debug, Eq, PartialEq)]
pub enum TokenKind {
    Single(String),
    Many(Vec<String>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Field {
    Token {
        name: String,
        kind: TokenKind,
        optional: bool,
        unordered: bool,
    },
    Node {
        name: String,
        ty: String,
        optional: bool,
        unordered: bool,
    },
}

#[derive(Debug, Clone)]
pub struct AstEnumSrc {
    #[allow(dead_code)]
    pub documentation: Vec<String>,
    pub name: String,
    // pub traits: Vec<String>,
    pub variants: Vec<String>,
}

impl Field {
    pub fn method_name(&self, language_kind: LanguageKind) -> proc_macro2::Ident {
        match self {
            Field::Token { name, .. } => {
                let name = match (name.as_str(), language_kind) {
                    (";", _) => "semicolon",
                    ("'{'", _) => "l_curly",
                    ("'}'", _) => "r_curly",
                    ("'('", _) => "l_paren",
                    ("')'", _) => "r_paren",
                    ("'['", _) => "l_brack",
                    ("']'", _) => "r_brack",
                    // NOTE(air): We also added `[[` and `]]` here
                    ("[[", _) => "l_brack2",
                    ("]]", _) => "r_brack2",
                    ("'`'", _) => "backtick",
                    ("<", _) => "l_angle",
                    (">", _) => "r_angle",
                    ("=", _) => "eq",
                    ("!", _) => "excl",
                    ("*", _) => "star",
                    ("&", _) => "amp",
                    (".", _) => "dot",
                    ("...", _) => "dotdotdot",
                    ("=>", _) => "fat_arrow",
                    (":", _) => "colon",
                    ("::", _) => "double_colon",
                    ("?", _) => "question_mark",
                    ("+", _) => "plus",
                    ("-", _) => "minus",
                    ("#", _) => "hash",
                    ("@", _) => "at",
                    ("+=", _) => "add_assign",
                    ("-=", _) => "subtract_assign",
                    ("*=", _) => "times_assign",
                    ("%=", _) => "remainder_assign",
                    ("**=", _) => "exponent_assign",
                    (">>=", _) => "left_shift_assign",
                    ("<<=", _) => "right_shift_assign",
                    (">>>=", _) => "unsigned_right_shift_assign",
                    ("~", _) => "bitwise_not",
                    ("&=", _) => "bitwise_and_assign",
                    ("|=", _) => "bitwise_or_assign",
                    ("^=", _) => "bitwise_xor_assign",
                    ("&&=", _) => "bitwise_logical_and_assign",
                    ("||=", _) => "bitwise_logical_or_assign",
                    ("??=", _) => "bitwise_nullish_coalescing_assign",
                    ("++", _) => "increment",
                    ("--", _) => "decrement",
                    ("<=", _) => "less_than_equal",
                    (">=", _) => "greater_than_equal",
                    ("==", _) => "equality",
                    ("===", _) => "strict_equality",
                    ("!=", _) => "inequality",
                    ("!==", _) => "strict_inequality",
                    ("/", _) => "slash",
                    ("\\", _) => "backslash",
                    ("%", _) => "remainder",
                    ("**", _) => "exponent",
                    ("<<", _) => "left_shift",
                    (">>", _) => "right_shift",
                    (">>>", _) => "unsigned_right_shift",
                    ("|", _) => "bitwise_or",
                    ("^", _) => "bitwise_xor",
                    ("??", _) => "nullish_coalescing",
                    ("||", _) => "logical_or",
                    ("&&", _) => "logical_and",
                    ("$=", _) => "suffix",
                    ("~=", _) => "whitespace_like",
                    (",", _) => "comma",
                    _ => name,
                };

                let kind_source = language_kind.kinds();

                // we need to replace "-" with "_" for the keywords
                // e.g. we have `color-profile` in css but it's an invalid ident in rust code
                if kind_source.keywords.contains(&name) {
                    format_ident!("{}_token", name.replace('-', "_"))
                } else {
                    format_ident!("{}_token", name)
                }
            }
            Field::Node { name, .. } => {
                let (prefix, tail) = name.split_once('_').unwrap_or(("", name));
                let final_name = if LANGUAGE_PREFIXES.contains(&prefix) {
                    tail
                } else {
                    name.as_str()
                };

                // this check here is to avoid emitting methods called "type()",
                // where "type" is a reserved word
                if final_name == "type" {
                    format_ident!("ty")
                } else {
                    format_ident!("{}", final_name)
                }
            }
        }
    }
    #[allow(dead_code)]
    pub fn ty(&self) -> proc_macro2::Ident {
        match self {
            Field::Token { .. } => format_ident!("SyntaxToken"),
            Field::Node { ty, .. } => format_ident!("{}", ty),
        }
    }

    pub fn is_optional(&self) -> bool {
        match self {
            Field::Node { optional, .. } => *optional,
            Field::Token { optional, .. } => *optional,
        }
    }

    pub fn is_unordered(&self) -> bool {
        match self {
            Field::Node { unordered, .. } => *unordered,
            Field::Token { unordered, .. } => *unordered,
        }
    }
}
