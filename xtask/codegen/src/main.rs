use xtask::{project_root, pushd, Result};

use xtask::Mode::Overwrite;
use xtask_codegen::{
    generate_ast, generate_crate, generate_formatters,
    generate_parser_tests, generate_tables, task_command, TaskCommand,
};

fn main() -> Result<()> {
    let _d = pushd(project_root());
    let result = task_command().fallback_to_usage().run();

    match result {
        TaskCommand::Formatter => {
            generate_formatters();
        }
        TaskCommand::Grammar(language_list) => {
            generate_ast(Overwrite, language_list)?;
        }
        TaskCommand::Test => {
            generate_parser_tests(Overwrite)?;
        }
        TaskCommand::Unicode => {
            generate_tables()?;
        }
        TaskCommand::All => {
            generate_tables()?;
            generate_ast(Overwrite, vec![])?;
            generate_parser_tests(Overwrite)?;
            generate_formatters();
        }
        TaskCommand::NewCrate { name } => {
            generate_crate(name)?;
        }
    }

    Ok(())
}
