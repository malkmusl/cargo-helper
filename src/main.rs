use std::env;

use config::create_config_file;
use project::init::init_new_project;

use project::new::create_new_project;
use rust::get_parent_directory_name;

mod nix;
mod rust;
mod config;
mod project;
mod git;
mod console;

pub enum LICENSE {
    OpenSource,
    Proprietary
}

fn main() {
    let args: Vec<String> = env::args().collect();

    console::print_error("test");
    console::print_warn("warn");
    console::print_success("test");
    console::print_info("info");

    if let Err(e) = create_config_file() {
        console::print_info(&e);
    }

    if args.len() < 3 {
        println!("Usage: cargo-helper new <name>");
        println!("Usage: cargo-helper init");
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "-o" => setup(args, LICENSE::OpenSource),
        "-p" => setup(args, LICENSE::Proprietary),
            
        _ => {
            console::print_error("Unknown command. Usage: cargo-helper <new/init> <name(optional)>");
        }
    }
}

fn setup(args: Vec<String>, license: LICENSE){
    if args.len() < 4 {
        console::print_error("Missing project name. Usage: cargo-helper new <name>");
        return;
    }
    match args[2].as_str() {
      "new" => {
        let project_name = &args[3];
        if let Err(e) = create_new_project(project_name, license) {
            console::print_error(&e);
        }
      },
      "init" => {
        let _project_name = match args.get(2) {
            Some(name) => name.clone(),
            None => {
                match get_parent_directory_name() {
                    Some(name) => name,
                    None => {
                        console::print_error("Failed to determine project name.");
                        return;
                    }
                }
            }
        };
        if let Err(e) = init_new_project(license) {
            console::print_error(&e);
        }
      }
      _ => {

      }

    } 
}