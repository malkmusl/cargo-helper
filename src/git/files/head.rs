use crate::rust::write_file;

pub fn generate_head(project_dir: &str) -> Result<(), String> {
    let main_rs_content = format!(
        r##"ref: refs/heads/master
        "##,
    );

    let main_rs_path = format!("{}/.git/HEAD", project_dir);
    write_file(&main_rs_path, &main_rs_content)
}
