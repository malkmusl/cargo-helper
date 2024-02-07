use self::{config::generate_config, description::generate_description, head::generate_head, hooks::generate_hooks, info::exclude::generate_info_exclude};

mod head;
mod config;
mod description;
mod info;
mod hooks;

pub fn generate_files(project_dir: &str){
    match generate_head(project_dir) {
        Ok(_) => match generate_config(project_dir) {
            Ok(_) => match generate_description(project_dir) {
                Ok(_) => match generate_info_exclude(project_dir) {
                    Ok(_) => generate_hooks(project_dir),
                    Err(err) => {
                        println!("Failed to generate .git/info/exclude");
                        println!("{}", err);
                    },
                }
                Err(err) => {
                    println!("Failed to generate .git/description");
                    println!("{}", err);
                },
            },
            Err(err) => {
                println!("Failed to generate .git/config");
                println!("{}", err)
            },
        },
        Err(err) => {
            println!("Failed to generate .git/HEAD");
            println!("{}", err)
        },
    }
}