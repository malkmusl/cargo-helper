use crate::rust::write_file;

pub fn generate_post_update(project_dir: &str) -> Result<(), String> {
    let main_rs_content = format!(
        r##"
#!/nix/store/cjbyb45nxiqidj95c4k1mh65azn1x896-bash-5.2-p21/bin/bash
#
# An example hook script to prepare a packed repository for use over
# dumb transports.
#
# To enable this hook, rename this file to "post-update".

exec git update-server-info

        "##,
    );

    let main_rs_path = format!("{}/.git/hooks/post-update.sample", project_dir);
    write_file(&main_rs_path, &main_rs_content)
}
