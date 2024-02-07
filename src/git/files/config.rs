use crate::rust::write_file;

pub fn generate_config(project_dir: &str) -> Result<(), String> {
    let main_rs_content = format!(
r##"[core]
	repositoryformatversion = 0
	filemode = true
	bare = false
	logallrefupdates = true
        "##,
    );

    let main_rs_path = format!("{}/.git/config", project_dir);
    write_file(&main_rs_path, &main_rs_content)
}
