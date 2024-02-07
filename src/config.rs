#![allow(deprecated)]
use std::{env, fs, path::Path};

use crate::console;

pub fn get_config() -> Result<(bool, String, String), String> {
    let home_dir = match env::home_dir() {
        Some(path) => path,
        None => return Err(console::print_error("Failed to get the user's home directory.")),
    };

    let config_file_path = home_dir.join(".config").join("cargo-helper").join("config");

    if !config_file_path.exists() {
        return Err(console::print_error(&format!("Config file '{}' does not exist.", config_file_path.display())));
    }

    let content = fs::read_to_string(&config_file_path)
        .map_err(|e| format!("Failed to read config file '{}': {}", config_file_path.display(), e))?;

    let mut link_rust_to_project = false;
    let mut rustup_path = String::new();
    let mut cargo_path = String::new();

    for line in content.lines() {
        if let Some((key, value)) = line.split_once('=') {
            match key.trim() {
                "link_rust_to_project" => link_rust_to_project = value.trim().parse().unwrap_or(false),
                "rustup_path" => rustup_path = value.trim().to_string(),
                "cargo_path" => cargo_path = value.trim().to_string(),
                _ => {} // Ignore unknown keys
            }
        }
    }

    Ok((link_rust_to_project, rustup_path, cargo_path))
}

pub fn create_config_file() -> Result<(), String> {
    // Get the user's home directory
    let home_dir = match env::home_dir() {
        Some(path) => path,
        None => return Err("Failed to get the user's home directory.".to_string()),
    };

    // Define the directory path for the config file
    let config_dir = home_dir.join(".config").join("cargo-helper");
    let config_file_path = config_dir.join("config");

    // Create the directory if it doesn't exist
    fs::create_dir_all(&config_dir)
        .map_err(|e| format!("Failed to create directory '{}': {}", config_dir.display(), e))?;

    // Check if the config file already exists
    if Path::new(&config_file_path).exists() {
        return Err(format!("Config file '{}' already exists.", config_file_path.display()));
    }

    // Define the content of the config file
    let config_content = format!(
        r#"
# Cargo Helper Configuration

# Whether to link the Rust toolchain to the project directory
link_rust_to_project = false

# The path to the Rustup installation directory
rustup_path = "{}/.rustup"

# The path to the Cargo installation directory
cargo_path = "{}/.cargo"
"#,
        home_dir.display(),
        home_dir.display()
    );

    // Write the content to the config file
    fs::write(&config_file_path, config_content)
        .map_err(|e| format!("Failed to write to file '{}': {}", config_file_path.display(), e))?;

    println!("Config file created successfully at '{}'.", config_file_path.display());

    Ok(())
}