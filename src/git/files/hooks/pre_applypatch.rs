use crate::rust::write_file;

pub fn generate_pre_applypatch(project_dir: &str) -> Result<(), String> {
    let main_rs_content = r##"
#!/nix/store/cjbyb45nxiqidj95c4k1mh65azn1x896-bash-5.2-p21/bin/bash
#
# An example hook script to verify what is about to be committed
# by applypatch from an e-mail message.
#
# The hook should exit with non-zero status after issuing an
# appropriate message if it wants to stop the commit.
#
# To enable this hook, rename this file to "pre-applypatch".

. git-sh-setup
precommit="$(git rev-parse --git-path hooks/pre-commit)"
test -x "$precommit" && exec "$precommit" ${1+"$@"}
:

"##;

    let main_rs_path = format!("{}/.git/hooks/pre-applypatch.sample", project_dir);
    write_file(&main_rs_path, &main_rs_content)
}
