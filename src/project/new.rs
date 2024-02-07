use std::path::Path;
use crate::{
    console,
    git,
    nix::build::{
        build_debug::create_build_debug_nix_file,
        build_release::create_build_release_nix_file,
    },
    rust::{
        cargo_file::create_cargo_toml,
        create_project_directory,
        get_current_directory,
        main_file::generate_main_rs,
        toolchain::create_toolchain_links,
    },
    LICENSE,
};

pub fn create_new_project(project_name: &str, license: LICENSE) -> Result<(), String> {
    match get_current_directory() {
        Ok(current_dir) => {
            console::print_info(&format!("Current directory: {}", current_dir.display()));
            let project_dir = format!("{}/{}", current_dir.display(), project_name);
            if Path::new(&project_dir).exists() {
                return Err(format!("Project directory '{}' already exists.", project_name));
            }
            console::print_info(&format!("Project directory: {}", project_dir));
            console::print_info(&format!("Creatating Cargo.toml..."));
            match create_project_directory(&project_dir) {
                Ok(_) => {
                    match create_cargo_toml(&project_dir, project_name) {
                        Ok(_) => {
                            console::print_success("Cargo.toml sucessfull created");
                            console::print_info(&format!("Creating buildDebug.nix..."));
                            match create_build_debug_nix_file(&project_dir, project_name) {
                                Ok(_) => {
                                    console::print_success("buildDebug.nix sucessfull created");
                                    console::print_info(&format!("Creating buildRelease.nix..."));
                                    match create_build_release_nix_file(&project_dir, project_name) {
                                        Ok(_) => {
                                            console::print_success(
                                                "buildRelease.nix sucessfull created"
                                            );
                                            let source_dir = format!(
                                                "{}/{}/src",
                                                current_dir.display(),
                                                project_name
                                            );
                                            if Path::new(&source_dir).exists() {
                                                return Err(
                                                    console::print_error(
                                                        &format!("Project directory '{}' already exists.", project_name)
                                                    )
                                                );
                                            }
                                            match create_project_directory(&source_dir) {
                                                Ok(_) => {
                                                    console::print_success(
                                                        "Source Directory created!"
                                                    );
                                                    match create_toolchain_links(&project_dir) {
                                                        Ok(_) => {
                                                            match generate_main_rs(&source_dir) {
                                                                Ok(_) => {
                                                                    println!(
                                                                        "/src/main.rs created"
                                                                    );
                                                                    match license {
                                                                        LICENSE::OpenSource => {
                                                                            match
                                                                                crate::project::license::open_source_license::generate_license(
                                                                                    &project_dir,
                                                                                    project_name
                                                                                )
                                                                            {
                                                                                Ok(_) => {
                                                                                    match
                                                                                        crate::project::readme::open_source_readme::generate_readme(
                                                                                            &project_dir,
                                                                                            project_name
                                                                                        )
                                                                                    {
                                                                                        Ok(_) => {
                                                                                            git::init(
                                                                                                &project_dir
                                                                                            );
                                                                                        }
                                                                                        Err(
                                                                                            err,
                                                                                        ) => {
                                                                                            println!(
                                                                                                "Failed to create README.md (Open Source)"
                                                                                            );
                                                                                            return Err(
                                                                                                err
                                                                                            );
                                                                                        }
                                                                                    }
                                                                                }
                                                                                Err(err) => {
                                                                                    println!(
                                                                                        "Failed to create LICENSE.md (Open Source)"
                                                                                    );
                                                                                    return Err(err);
                                                                                }
                                                                            }
                                                                        }
                                                                        LICENSE::Proprietary => {
                                                                            match
                                                                                crate::project::license::proprietary_license::generate_license(
                                                                                    &project_dir,
                                                                                    project_name
                                                                                )
                                                                            {
                                                                                Ok(_) => {
                                                                                    match
                                                                                        crate::project::readme::proprietary_readme::generate_readme(
                                                                                            &project_dir,
                                                                                            project_name
                                                                                        )
                                                                                    {
                                                                                        Ok(_) => {
                                                                                            git::init(
                                                                                                &project_dir
                                                                                            );
                                                                                        }
                                                                                        Err(
                                                                                            err,
                                                                                        ) => {
                                                                                            println!(
                                                                                                "Failed to create README.md (Proprietary)"
                                                                                            );
                                                                                            return Err(
                                                                                                err
                                                                                            );
                                                                                        }
                                                                                    }
                                                                                }
                                                                                Err(err) => {
                                                                                    println!(
                                                                                        "Failed to create LICENSE.md (Proprietary)"
                                                                                    );
                                                                                    return Err(err);
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                Err(err) => {
                                                                    println!(
                                                                        "Failed to create /src/main.rs"
                                                                    );
                                                                    return Err(err);
                                                                }
                                                            }
                                                        }
                                                        Err(err) => {
                                                            println!(
                                                                "Failed to create Rust Toolchain Links"
                                                            );
                                                            return Err(err);
                                                        }
                                                    }
                                                }
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
                        }
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
