use xtask::{Result, project_root, pushd};

use xtask::Mode::Overwrite;
use xtask_codegen::{TaskCommand, generate_ast, generate_formatters, task_command};

fn main() -> Result<()> {
    let _d = pushd(project_root());
    let result = task_command().fallback_to_usage().run();

    match result {
        TaskCommand::Formatter {
            allow_dirty,
            allow_staged,
        } => {
            generate_formatters(allow_dirty, allow_staged);
        }
        TaskCommand::Grammar(language_list) => {
            generate_ast(Overwrite, language_list)?;
        }
        TaskCommand::All => {
            generate_ast(Overwrite, vec![])?;
            generate_formatters(false, false);
        }
    }

    Ok(())
}
