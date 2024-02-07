pub mod folders;
pub mod files;

pub fn init(project_dir: &str) {
    folders::create_folders(project_dir);
    files::generate_files(project_dir);
}