use std::{env, fs, path::PathBuf};

pub mod cargo_file;
pub mod main_file;
pub mod toolchain;

pub fn write_file(file_path: &str, content: &str) -> Result<(), String> {
    fs::write(file_path, content)
        .map_err(|e| format!("Failed to write file '{}': {}", file_path, e))
}

pub fn create_project_directory(dir_path: &str) -> Result<(), String> {
    fs::create_dir_all(dir_path)
        .map_err(|e| format!("Failed to create directory '{}': {}", dir_path, e))
}

pub fn get_current_directory() -> Result<PathBuf, String> {
    env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))
}

pub fn get_parent_directory_name() -> Option<String> {
    env::current_dir()
        .ok()?
        .parent()?
        .file_name()?
        .to_string_lossy()
        .into_owned()
        .into()
}