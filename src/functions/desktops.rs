use files::sed_file;

use crate::args::DesktopSetup;
use crate::internal::sysutils::{catnest_reload, dinit_enable, dinit_enable_user, user_add_group};
use crate::internal::*;

pub fn install_desktop_setup(desktop_setup: DesktopSetup) {
    log::debug!("Installing {:?}", desktop_setup);
    match desktop_setup {
        DesktopSetup::Hyprland => install_hyprland(),
        DesktopSetup::FBCli => install_fbcli(),
        DesktopSetup::None => log::debug!("No desktop setup selected"),
    }
}

fn install_desktop_common() {
    install(vec![
        // desktop app
        "rofi",
        "foot",
        "swww",
        "waybar",
        // desktop service
        "pipewire",
        "wireplumber",
        "xdg-user-dirs",
        // system service
        "greetd",
        "turnstile",
        // TODO: selectable greeter
        "greetd-regreet",
        "cage",
        // TODO: fonts
        "ttf-noto-fonts",
    ]);
    dinit_enable("greetd");
    dinit_enable_user("pipewire-pulse");
    dinit_enable_user("wireplumber");

    // TODO: no m option for catnest workaround
    catnest_reload();
    user_add_group("greeter", "video");
    user_add_group("greeter", "input");
    user_add_group("greeter", "seat");

    // TODO: need modular greeter config
    files_eval(
        sed_file(
            "/mnt/etc/greetd/config.toml",
            "agreety --cmd /bin/bash",
            "cage -s -d -- regreet",
        ),
        "Setting up greeter config",
    )
}

fn install_hyprland() {
    install_desktop_common();
    install(vec!["hyprland"]);
}

fn install_fbcli() {
    install(vec!["yaft", "xdg-user-dirs", "greetd-tui"]);
    dinit_enable("greetd");

    // TODO: no m option for catnest workaround
    catnest_reload();
    user_add_group("greeter", "video");
    user_add_group("greeter", "input");
    user_add_group("greeter", "seat");

    // TODO: need modular greeter config
    files_eval(
        sed_file(
            "/mnt/etc/greetd/config.toml",
            "agreety --cmd /bin/bash",
            "yaft tuigreet -t -r --asterisks -c yaft",
        ),
        "Setting up greeter config",
    )
}
