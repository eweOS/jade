use crate::internal::exec::*;
use crate::internal::sysutils::{user_add_group, user_set_password};
use crate::internal::*;
use std::process::Command;

pub fn new_user(username: &str, hasroot: bool, password: &str, do_hash_pass: bool, shell: &str) {
    let shell: &str = shell;
    if do_hash_pass {
        let hashed_pass = &*hash_pass(password).stdout;
        let _password = match std::str::from_utf8(hashed_pass) {
            Ok(v) => v,
            Err(e) => panic!("Failed to hash password, invalid UTF-8 sequence {}", e),
        };
    }
    let shell_to_install = match shell {
        "bash" => "bash",
        "csh" => "tcsh",
        "fish" => "fish",
        "tcsh" => "tcsh",
        "zsh" => "zsh",
        &_ => "bash",
    };
    install::install(vec![shell_to_install]);
    let shell_path = match shell {
        "bash" => "/bin/bash",
        "csh" => "/usr/bin/csh",
        "fish" => "/usr/bin/fish",
        "tcsh" => "/usr/bin/tcsh",
        "zsh" => "/usr/bin/zsh",
        &_ => "/usr/bin/bash",
    };
    exec_eval(
        exec_chroot(
            "adduser",
            vec![
                String::from("-D"),
                String::from("-s"),
                String::from(shell_path),
                String::from(username),
            ],
        ),
        format!("Create user {}", username).as_str(),
    );
    user_set_password(username, password);
    if hasroot {
        user_add_group(username, "wheel");
        files_eval(
            files::sed_file(
                "/mnt/etc/sudoers",
                "# %wheel ALL=(ALL:ALL) ALL",
                "%wheel ALL=(ALL:ALL) ALL",
            ),
            "Add wheel group to sudoers",
        );
    }

    user_add_group(username, "video");
    user_add_group(username, "input");
    user_add_group(username, "audio");
    user_add_group(username, "seat");
}

pub fn hash_pass(password: &str) -> std::process::Output {
    let output = Command::new("openssl")
        .args(["passwd", "-1", password])
        .output()
        .expect("Failed to hash password");
    output
}

pub fn root_pass(root_pass: &str) {
    user_set_password("root", root_pass);
}
