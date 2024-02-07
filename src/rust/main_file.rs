use super::write_file;

pub fn generate_main_rs(src_dir: &str) -> Result<(), String> {
    let main_rs_content = r#"
#

    fn main() {
    println!("Hello, world!");
}"#;
    let main_rs_path = format!("{}/main.rs", src_dir);
    write_file(&main_rs_path, main_rs_content)
}