use crate::Result;
use air_r_parser_tests::{check, corpus, Document};
use std::fs;

pub fn accept() -> Result<()> {
    for input_path in corpus::input_files() {
        let display = corpus::relative(&input_path);
        let mut document = Document::parse(&fs::read_to_string(&input_path)?)
            .map_err(|err| crate::anyhow!("{display}: {err}"))?;

        for section in &mut document.sections {
            let expectation = section.expectation;
            for block in section.blocks_mut() {
                // On failure write what we can and let the test run report it:
                // a regeneration should be maximally informative
                block.generated = check(block, expectation).unwrap_or_default();
            }
        }

        let snapshot_path = corpus::snapshot_of(&input_path);
        let rendered = document.render_snapshot();

        if fs::read_to_string(&snapshot_path).ok().as_deref() != Some(&rendered) {
            fs::write(&snapshot_path, rendered)?;
            println!("updated {}", corpus::relative(&snapshot_path));
        }
    }

    remove_orphan_snapshots()
}

/// A deleted input file leaves its snapshot behind
fn remove_orphan_snapshots() -> Result<()> {
    for snapshot in corpus::snapshot_files() {
        let name = snapshot
            .file_name()
            .and_then(|name| name.to_str())
            .expect("corpus paths are UTF-8");

        if !snapshot
            .with_file_name(name.replace(".snap.md", ".md"))
            .exists()
        {
            fs::remove_file(&snapshot)?;
            println!("removed orphan {}", corpus::relative(&snapshot));
        }
    }

    Ok(())
}
