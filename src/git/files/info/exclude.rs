use crate::rust::write_file;

pub fn generate_info_exclude(src_dir: &str) -> Result<(), String> {
    let main_rs_content = format!(
        r##"
# git ls-files --others --exclude-from=.git/info/exclude
# Lines that start with '#' are comments.
# For a project mostly in C, the following would be a good set of
# exclude patterns (uncomment them if you want to use them):
# *.[oa]
# *~
        "##,
    );

    let main_rs_path = format!("{}/.git/info/exclude", src_dir);
    write_file(&main_rs_path, &main_rs_content)
}
