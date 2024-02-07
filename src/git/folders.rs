#![allow(dead_code)]
use crate::rust::create_project_directory;

fn create_git_folder(project_dir: &str) {
    let dir_path = format!("{}/.git", project_dir);
    match create_project_directory(&dir_path) {
        Ok(_) => println!(".git sucessfull crated"),
        Err(err) => {
            println!("Failed to create .git");
            println!("{}", err);
        },
    }
}

fn create_branches_folder(project_dir: &str) {
    let dir_path = format!("{}/.git/branches", project_dir);
    match create_project_directory(&dir_path) {
        Ok(_) => println!(".git/branches sucessfull crated"),
        Err(err) => {
            println!("Failed to create .git/branches");
            println!("{}", err);
        },
    }
}

fn create_hooks_folder(project_dir: &str) {
    let dir_path = format!("{}/.git/hooks", project_dir);
    match create_project_directory(&dir_path) {
        Ok(_) => println!(".git/hooks sucessfull crated"),
        Err(err) => {
            println!("Failed to create .git/hooks");
            println!("{}", err);
        },
    }
}

fn create_info_folder(project_dir: &str) {
    let dir_path = format!("{}/.git/info", project_dir);
    match create_project_directory(&dir_path) {
        Ok(_) => println!(".git/info sucessfull crated"),
        Err(err) => {
            println!("Failed to create .git/info");
            println!("{}", err);
        },
    }
}

fn create_objects_folder(project_dir: &str) {
    let dir_path = format!("{}/.git/objects", project_dir);
    match create_project_directory(&dir_path) {
        Ok(_) => println!(".git/objects sucessfull crated"),
        Err(err) => {
            println!("Failed to create .git/objects");
            println!("{}", err);
        },
    }
}

fn create_objects_info_folder(project_dir: &str) {
    let dir_path = format!("{}/.git/objects/info", project_dir);
    match create_project_directory(&dir_path) {
        Ok(_) => println!(".git/objects/info sucessfull crated"),
        Err(err) => {
            println!("Failed to create .git/objects/info");
            println!("{}", err);
        },
    }
}

fn create_objects_pack_folder(project_dir: &str) {
    let dir_path = format!("{}/.git/objects/pack", project_dir);
    match create_project_directory(&dir_path) {
        Ok(_) => println!(".git/objects/pack sucessfull crated"),
        Err(err) => {
            println!("Failed to create .git/objects/pack");
            println!("{}", err);
        },
    }
}

fn create_refs_folder(project_dir: &str) {
    let dir_path = format!("{}/.git/refs", project_dir);
    match create_project_directory(&dir_path) {
        Ok(_) => println!(".git/refs sucessfull crated"),
        Err(err) => {
            println!("Failed to create .git/refs");
            println!("{}", err);
        },
    }
}

pub fn create_folders(project_dir: &str) {
    create_git_folder(project_dir);
    create_branches_folder(project_dir);
    create_hooks_folder(project_dir);
    create_info_folder(project_dir);
    create_objects_folder(project_dir);
    create_objects_info_folder(project_dir);
    create_objects_pack_folder(project_dir);
    create_refs_folder(project_dir)
}