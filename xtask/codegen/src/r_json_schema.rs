use std::path::PathBuf;

const ROOT_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");

pub fn generate_json_schema() -> anyhow::Result<()> {
    let schema = json_schema()?;
    let schema_path = schema_path();
    std::fs::write(schema_path, schema.as_bytes())?;
    Ok(())
}

fn json_schema() -> anyhow::Result<String> {
    let schema = schemars::schema_for!(workspace::toml_options::TomlOptions);
    let schema = serde_json::to_string_pretty(&schema)?;
    Ok(schema)
}

fn schema_path() -> PathBuf {
    PathBuf::from(ROOT_DIR).join("air.schema.json")
}

#[cfg(test)]
mod tests {
    use crate::r_json_schema::json_schema;
    use crate::r_json_schema::schema_path;

    #[test]
    fn test_schema_can_be_generated_and_hasnt_changed() -> anyhow::Result<()> {
        let schema = json_schema()?;
        let schema_path = schema_path();

        // Snapshot so you can easily see diffs when changes occur
        insta::assert_snapshot!(schema);

        // Assert nothing has changed when compared with the official schema file.
        // Run `just gen-schema` to update as needed.
        let contents = std::fs::read_to_string(schema_path)?;
        assert_eq!(schema, contents);

        Ok(())
    }
}
