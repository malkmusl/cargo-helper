
use std::path::Path;
use std::process::Command;

use crate::config::get_config;

pub fn create_toolchain_links(project_dir: &str) -> Result<(), String> {
    let (link_rust_to_project, rustup_path, cargo_path) = match get_config() {
        Ok(config) => config,
        Err(e) => return Err(format!("Error getting config: {}", e)),
    };

    if link_rust_to_project {
        // Create symbolic link for .rustup directory
        let rustup_in_project = Path::new(project_dir).join(".rustup");
        if rustup_in_project.exists() {
            return Err(format!("Symbolic link '{}' already exists.", rustup_in_project.display()));
        }

        execute_ln_command(&rustup_path.to_string(), &rustup_in_project.to_str().unwrap())?;

        // Create symbolic link for .cargo directory
        let cargo_in_project = Path::new(project_dir).join(".cargo");
        if cargo_in_project.exists() {
            return Err(format!("Symbolic link '{}' already exists.", cargo_in_project.display()));
        }

        execute_ln_command(&cargo_path, &cargo_in_project.to_str().unwrap())?;
    }

    Ok(())
}

fn execute_ln_command(target: &str, link: &str) -> Result<(), String> {
    let t = target.replace("\"", "");
    let l = link;
    let output = Command::new("ln")
        .args(&["-s", &t, l])
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Command failed: {}", stderr));
    }

    Ok(())
}
