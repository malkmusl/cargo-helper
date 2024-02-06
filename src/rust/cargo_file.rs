use super::write_file;

pub fn create_cargo_toml(project_dir: &str, project_name: &str) -> Result<(), String> {
    let cargo_toml_content = generate_cargo_toml(project_name);
    let cargo_toml_path = format!("{}/Cargo.toml", project_dir);
    write_file(&cargo_toml_path, &cargo_toml_content)
}

pub fn generate_cargo_toml(project_name: &str) -> String {
    format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]"#,
        project_name
    )
}
