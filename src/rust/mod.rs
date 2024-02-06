use std::fs;

pub mod cargo_file;
pub mod main_file;
pub mod toolchain;

fn write_file(file_path: &str, content: &str) -> Result<(), String> {
    fs::write(file_path, content)
        .map_err(|e| format!("Failed to write file '{}': {}", file_path, e))
}
