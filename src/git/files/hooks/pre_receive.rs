use crate::rust::write_file;

pub fn generate_pre_receive(project_dir: &str) -> Result<(), String> {
    let main_rs_content = r##"
#!/nix/store/cjbyb45nxiqidj95c4k1mh65azn1x896-bash-5.2-p21/bin/bash
#
# An example hook script to make use of push options.
# The example simply echoes all push options that start with 'echoback='
# and rejects all pushes when the "reject" push option is used.
#
# To enable this hook, rename this file to "pre-receive".

if test -n "$GIT_PUSH_OPTION_COUNT"
then
	i=0
	while test "$i" -lt "$GIT_PUSH_OPTION_COUNT"
	do
		eval "value=\$GIT_PUSH_OPTION_$i"
		case "$value" in
		echoback=*)
			echo "echo from the pre-receive-hook: ${value#*=}" >&2
			;;
		reject)
			exit 1
		esac
		i=$((i + 1))
	done
fi

"##;

    let main_rs_path = format!("{}/.git/hooks/pre-receive.sample", project_dir);
    write_file(&main_rs_path, &main_rs_content)
}
