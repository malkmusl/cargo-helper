use crate::rust::write_file;

pub fn generate_applypatch_msg(project_dir: &str) -> Result<(), String> {
    let main_rs_content = format!(
        r##"
#!/nix/store/cjbyb45nxiqidj95c4k1mh65azn1x896-bash-5.2-p21/bin/bash
#
# An example hook script to check the commit log message taken by
# applypatch from an e-mail message.
#
# The hook should exit with non-zero status after issuing an
# appropriate message if it wants to stop the commit.  The hook is
# allowed to edit the commit message file.
#
# To enable this hook, rename this file to "applypatch-msg".

. git-sh-setup
commitmsg="$(git rev-parse --git-path hooks/commit-msg)"
test -x "$commitmsg" && exec "$commitmsg" ${{1+"$@"}}
:
        "##,
    );

    let main_rs_path = format!("{}/.git/info/exclude", project_dir);
    write_file(&main_rs_path, &main_rs_content)
}
