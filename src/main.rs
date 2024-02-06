use std::env;

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use config::create_config_file;
use rust::cargo_file::create_cargo_toml;
use rust::main_file::generate_main_rs;
use rust::toolchain::create_toolchain_links;

use crate::nix::build::build_debug::create_build_debug_nix_file;
use crate::nix::build::build_release::create_build_release_nix_file;

mod nix;
mod rust;
mod config;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Err(e) = create_config_file() {
        println!("Error: {}", e);
    }

    if args.len() < 2 {
        println!("Usage: cargo-helper <new/init> <name(optional)>");
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "new" => {
            if args.len() < 3 {
                println!("Error: Missing project name. Usage: cargo-helper new <name>");
                return;
            }
            let project_name = &args[2];
            if let Err(e) = create_new_project(project_name) {
                println!("Error: {}", e);
            }
        }
        "init" => {
            let _project_name = match args.get(2) {
                Some(name) => name.clone(),
                None => {
                    match get_parent_directory_name() {
                        Some(name) => name,
                        None => {
                            println!("Error: Failed to determine project name.");
                            return;
                        }
                    }
                }
            };
            if let Err(e) = init_new_project() {
                println!("Error: {}", e);
            }
        }
        _ => {
            println!("Error: Unknown command. Usage: cargo-helper <new/init> <name(optional)>");
        }
    }
}

fn create_new_project(project_name: &str) -> Result<(), String> {

    match get_current_directory() {
        Ok(current_dir) => {
            println!("Current directory: {}", current_dir.display());
            let project_dir = format!("{}/{}", current_dir.display(), project_name);
            if Path::new(&project_dir).exists() {
                return Err(format!("Project directory '{}' already exists.", project_name));
            }
            println!("Project directory: {}", project_dir);
            match create_project_directory(&project_dir) {
                Ok(_) => {
                    match  create_cargo_toml(&project_dir, project_name){
                        Ok(_) => {
                            println!("Cargo.toml sucessfull created");
                            match create_build_debug_nix_file(&project_dir, project_name) {
                                Ok(_) => {
                                    println!("buildDebug.nix sucessfull created");
                                    match create_build_release_nix_file(&project_dir, project_name) {
                                        Ok(_) => {
                                            println!("buildRelease.nix sucessfull created");
                                            let source_dir = format!("{}/{}/src", current_dir.display(), project_name);
                                            if Path::new(&source_dir).exists() {
                                                return Err(format!("Project directory '{}' already exists.", project_name));
                                            }
                                            match create_project_directory(&source_dir){
                                                Ok(_) => {
                                                    println!("Source Directory created!");
                                                    match create_toolchain_links(&project_dir) {
                                                        Ok(_) => {
                                                            match  generate_main_rs(&source_dir){
                                                                Ok(_) => {
                                                                    println!("/src/main.rs created");
                                                                    return Ok(());
                                                                },
                                                                Err(err) => {
                                                                    println!("Failed to create /src/main.rs");
                                                                    return Err(err);
                                                                },
                                                            }
                                                        }
                                                        Err(err) => {
                                                            println!("Failed to create Rust Toolchain Links");
                                                            return Err(err);
                                                        }
                                                    }
                                                },
                                                Err(err) => {
                                                    println!("Failed to create Source Directory");
                                                    return Err(err);
                                                }
                                            }
                                        }
                                        Err(_) => todo!(),
                                    }
                                }
                                Err(_) => todo!(),
                            }
                        },
                        Err(_) => todo!(),
                    }
                }
                Err(_) => todo!(),
            }
        }
        Err(err) => eprintln!("{}", err),
    }
    Ok(())
}


fn init_new_project() -> Result<(), String> {

    match get_current_directory() {
        Ok(current_dir) => {

            let project_name = current_dir.file_name().unwrap().to_str().unwrap();
            println!("Project name: {}", project_name);

            println!("Current directory: {}", current_dir.display());
            let project_dir = format!("{}/{}", current_dir.display(), "");
            if Path::new(&project_dir).exists() {
                return Err(format!("Project directory '{}' already exists.", project_name));
            }
            println!("Project directory: {}", project_dir);
            match create_project_directory(&project_dir) {
                Ok(_) => {
                    match  create_cargo_toml(&project_dir, project_name){
                        Ok(_) => {
                            println!("Cargo.toml sucessfull created");
                            match create_build_debug_nix_file(&project_dir, project_name) {
                                Ok(_) => {
                                    println!("buildDebug.nix sucessfull created");
                                    match create_build_release_nix_file(&project_dir, project_name) {
                                        Ok(_) => {
                                            println!("buildRelease.nix sucessfull created");
                                            let source_dir = format!("{}/src", current_dir.display());
                                            if Path::new(&source_dir).exists() {
                                                return Err(format!("Project directory '{}' already exists.", project_name));
                                            }
                                            match create_project_directory(&source_dir){
                                                Ok(_) => {
                                                    println!("Source Directory created!");
                                                    match create_toolchain_links(&project_dir) {
                                                        Ok(_) => {
                                                            match  generate_main_rs(&source_dir){
                                                                Ok(_) => {
                                                                    println!("/src/main.rs created");
                                                                    return Ok(());
                                                                },
                                                                Err(err) => {
                                                                    println!("Failed to create /src/main.rs");
                                                                    return Err(err);
                                                                },
                                                            }
                                                        }
                                                        Err(err) => {
                                                            println!("Failed to create Rust Toolchain Links");
                                                            return Err(err);
                                                        }
                                                    }
                                                },
                                                Err(err) => {
                                                    println!("Failed to create Source Directory");
                                                    return Err(err);
                                                }
                                            }
                                        }
                                        Err(_) => todo!(),
                                    }
                                }
                                Err(_) => todo!(),
                            }
                        },
                        Err(_) => todo!(),
                    }
                }
                Err(_) => todo!(),
            }
        }
        Err(err) => eprintln!("{}", err),
    }
    Ok(())
}


fn create_project_directory(dir_path: &str) -> Result<(), String> {
    fs::create_dir_all(dir_path)
        .map_err(|e| format!("Failed to create directory '{}': {}", dir_path, e))
}


fn get_current_directory() -> Result<PathBuf, String> {
    env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))
}

fn get_parent_directory_name() -> Option<String> {
    env::current_dir()
        .ok()?
        .parent()?
        .file_name()?
        .to_string_lossy()
        .into_owned()
        .into()
}
