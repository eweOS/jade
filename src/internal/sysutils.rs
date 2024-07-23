use crate::internal::exec::*;
use crate::internal::*;

pub fn dinit_enable(service: &str) {
    exec_eval(
        exec_chroot(
            "ln",
            vec![
                String::from("-s"),
                format!("/usr/lib/dinit.d/system/{service}"),
                format!("/etc/dinit.d/boot.d/"),
            ],
        ),
        format!("Enabling system service {service}").as_str(),
    )
}

pub fn dinit_enable_user(service: &str) {
    exec_eval(
        exec_chroot(
            "ln",
            vec![
                String::from("-s"),
                format!("/usr/lib/dinit.d/user/{service}"),
                format!("/usr/lib/dinit/user/boot.d/"),
            ],
        ),
        format!("Enabling user service {service}").as_str(),
    )
}

pub fn catnest_reload() {
    exec_eval(
        exec_chroot("catnest", vec![]),
        "Running catnest to create initial user/group",
    )
}

pub fn user_set_password(username: &str, password: &str) {
    exec_eval(
        exec_chroot_stdin(
            "chpasswd",
            vec![],
            format!("{}:{}", username, password)
                .replace('\n', "")
                .as_str(),
        ),
        format!("Set password for user {}", username).as_str(),
    );
}

pub fn user_add_group(username: &str, group: &str) {
    exec_eval(
        exec_chroot("adduser", vec![String::from(username), String::from(group)]),
        format!("Add user {} to group {}", username, group).as_str(),
    );
}
