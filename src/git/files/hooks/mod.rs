use self::{commit_msg::generate_commit_msg, fsmonitor_watchman::generate_fsmonitor_watchman, post_update::generate_post_update, pre_applypatch::generate_pre_applypatch, pre_commit::generate_pre_commit, pre_merge_commit::generate_pre_merge_commit, pre_push::generate_pre_push, pre_rebase::generate_pre_rebase, pre_receive::generate_pre_receive, prepare_commit_msg::generate_prepare_commit_msg, push_to_checkout::generate_push_to_checkout, sendemail_validate::generate_sendemail_validate, update::generate_update};

pub mod applypatch_msg;
pub mod commit_msg;
pub mod fsmonitor_watchman;
pub mod post_update;
pub mod pre_applypatch;
pub mod pre_commit;
pub mod pre_merge_commit;
pub mod pre_push;
pub mod pre_rebase;
pub mod pre_receive;
pub mod prepare_commit_msg;
pub mod push_to_checkout;
pub mod sendemail_validate;
pub mod update;

pub fn generate_hooks(project_dir: &str) {
    match generate_pre_applypatch(project_dir) {
        Ok(_) => match generate_commit_msg(project_dir) {
            Ok(_) => match generate_fsmonitor_watchman(project_dir) {
                Ok(_) => match generate_post_update(project_dir) {
                    Ok(_) => match generate_pre_applypatch(project_dir) {
                        Ok(_) => match generate_pre_commit(project_dir) {
                            Ok(_) => match generate_pre_merge_commit(project_dir) {
                                Ok(_) => match generate_pre_push(project_dir) {
                                    Ok(_) => match generate_pre_rebase(project_dir) {
                                        Ok(_) => match generate_pre_receive(project_dir) {
                                            Ok(_) => match generate_prepare_commit_msg(project_dir) {
                                                Ok(_) => match generate_push_to_checkout(project_dir) {
                                                    Ok(_) => match generate_sendemail_validate(project_dir) {
                                                        Ok(_) => match generate_update(project_dir) {
                                                            Ok(_) => println!("DONE"),
                                                            Err(_) => todo!(),
                                                        },
                                                        Err(_) => todo!(),
                                                    },
                                                    Err(_) => todo!(),
                                                },
                                                Err(_) => todo!(),
                                            },
                                            Err(_) => todo!(),
                                        },
                                        Err(_) => todo!(),
                                    },
                                    Err(_) => todo!(),
                                },
                                Err(_) => todo!(),
                            },
                            Err(_) => todo!(),
                        },
                        Err(_) => todo!(),
                    },
                    Err(_) => todo!(),
                },
                Err(_) => todo!(),
            },
            Err(_) => todo!(),
        },
        Err(_) => todo!(),
    }
}