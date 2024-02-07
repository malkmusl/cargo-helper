use crate::rust::write_file;

pub fn generate_pre_merge_commit(project_dir: &str) -> Result<(), String> {
    let main_rs_content = format!(
        r##"
#!/nix/store/cjbyb45nxiqidj95c4k1mh65azn1x896-bash-5.2-p21/bin/bash
#
# An example hook script to verify what is about to be committed.
# Called by "git merge" with no arguments.  The hook should
# exit with non-zero status after issuing an appropriate message to
# stderr if it wants to stop the merge commit.
#
# To enable this hook, rename this file to "pre-merge-commit".

. git-sh-setup
test -x "$GIT_DIR/hooks/pre-commit" &&
        exec "$GIT_DIR/hooks/pre-commit"
:

        "##,
    );

    let main_rs_path = format!("{}/.git/hooks/pre-merge-commit.sample", project_dir);
    write_file(&main_rs_path, &main_rs_content)
}
