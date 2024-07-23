use crate::internal::exec::*;
use crate::internal::*;

pub fn set_timezone(timezone: &str) {
    exec_eval(
        exec_chroot(
            "ln",
            vec![
                "-sf".to_string(),
                format!("/usr/share/zoneinfo/{}", timezone),
                "/etc/localtime".to_string(),
            ],
        ),
        "Set timezone",
    );
    exec_eval(
        exec_chroot("hwclock", vec!["--systohc".to_string()]),
        "Set system clock",
    );
}

pub fn set_locale(locale: String) {
    files::create_file("/mnt/etc/locale.conf");
    files_eval(
        files::append_file("/mnt/etc/locale.conf", "LANG=en_US.UTF-8\n", false),
        "edit locale.conf",
    );
    for i in (0..locale.split(' ').count()).step_by(2) {
        if locale.split(' ').collect::<Vec<&str>>()[i] != "en_US.UTF-8" {
            files_eval(
                files::sed_file(
                    "/mnt/etc/locale.conf",
                    "en_US.UTF-8",
                    locale.split(' ').collect::<Vec<&str>>()[i],
                ),
                format!(
                    "Set locale {} in /etc/locale.conf",
                    locale.split(' ').collect::<Vec<&str>>()[i]
                )
                .as_str(),
            );
        }
    }
}
