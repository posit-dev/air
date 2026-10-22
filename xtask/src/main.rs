use xtask::{accept, project_root, pushd, Result};

fn main() -> Result<()> {
    let _d = pushd(project_root());

    match xtask::task_command().fallback_to_usage().run() {
        xtask::TaskCommand::Accept => accept::accept(),
    }
}
