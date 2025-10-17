use std::sync::LazyLock;

use settings::Table;

static DEFAULT_TABLE_NAMES: &[&str] = &["tribble", "fcase"];

pub static DEFAULT_TABLE: LazyLock<Table> = LazyLock::new(|| {
    let names: Vec<String> = DEFAULT_TABLE_NAMES.iter().map(|&s| s.to_string()).collect();
    Table::new(names)
});
