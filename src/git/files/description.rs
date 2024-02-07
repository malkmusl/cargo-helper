use crate::rust::write_file;

pub fn generate_description(project_dir: &str) -> Result<(), String> {
    let main_rs_content = format!(
        r##"Unnamed repository; edit this file 'description' to name the repository.
        "##,
    );

    let main_rs_path = format!("{}/.git/description", project_dir);
    write_file(&main_rs_path, &main_rs_content)
}
