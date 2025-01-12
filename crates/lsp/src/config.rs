//
// config.rs
//
// Copyright (C) 2024 Posit Software, PBC. All rights reserved.
//
//

/// Configuration of the LSP
#[derive(Clone, Debug, Default)]
pub(crate) struct LspConfig {}

/// Configuration of a document.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DocumentConfig {
    pub indent_style: Option<settings::IndentStyle>,
    pub indent_width: Option<settings::IndentWidth>,
    pub line_width: Option<settings::LineWidth>,
}
