use xtask::{project_root, pushd, Result};

use xtask::Mode::Overwrite;
use xtask_codegen::{
    generate_ast, generate_formatters, generate_json_schema, task_command, TaskCommand,
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
        TaskCommand::All => {
            generate_ast(Overwrite, vec![])?;
            generate_formatters();
        }
        TaskCommand::JsonSchema => {
            generate_json_schema()?;
        }
    }

    Ok(())
}
