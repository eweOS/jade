use crate::internal::exec::*;
use crate::internal::files::copy_file;
use crate::internal::sysutils::{catnest_reload, dinit_enable};
use crate::internal::*;
use log::warn;
use std::path::PathBuf;

pub fn install_base_packages(kernel: String) {
    std::fs::create_dir_all("/mnt/etc").unwrap();
    let kernel_to_install = if kernel.is_empty() {
        "linux"
    } else {
        match kernel.as_str() {
            "linux" => "linux",
            "linux-lts" => "linux-lts",
            _ => {
                warn!("Unknown kernel: {}, using default instead", kernel);
                "linux"
            }
        }
    };
    install::install(vec![
        "base",
        kernel_to_install,
        format!("{kernel_to_install}-headers").as_str(),
        "linux-firmware",
        "sudo",
        "dinit",
        "tinyramfs",
        "catnest",
        "pawprint",
        "connman",
    ]);
    dinit_enable("connman");
    files::copy_file("/etc/pacman.conf", "/mnt/etc/pacman.conf");
    catnest_reload();
}

pub fn genfstab() {
    exec_eval(
        exec(
            "bash",
            vec![
                String::from("-c"),
                String::from("genfstab -U /mnt >> /mnt/etc/fstab"),
            ],
        ),
        "Generate fstab",
    );
}

fn install_bootloader_common() {
    install::install(vec!["limine", "tinyramfs"]);
    exec_eval(
        exec_chroot(
            "limine-mkconfig",
            vec![String::from("-o"), String::from("/boot/limine.cfg")],
        ),
        "create limine.cfg",
    );
}

pub fn install_bootloader_efi() {
    install_bootloader_common();
    let efidir = std::path::Path::new("/mnt").join("/boot");
    let efi_str = efidir.to_str().unwrap();
    if !std::path::Path::new(&format!("/mnt{efi_str}")).exists() {
        crash(format!("The efidir {efidir:?} doesn't exist"), 1);
    }
    exec_eval(
        exec_chroot(
            "limine-install",
            vec![
                String::from("--target=auto-efi"),
                format!("--efi-directory={}", efi_str),
                String::from("--bootloader-id=eweos"),
                String::from("--removable"),
            ],
        ),
        "install limine as efi with --removable",
    );
    exec_eval(
        exec_chroot(
            "limine-install",
            vec![
                String::from("--target=auto-efi"),
                format!("--efi-directory={}", efi_str),
                String::from("--bootloader-id=eweos"),
            ],
        ),
        "install limine as efi without --removable",
    );
}

pub fn install_bootloader_legacy(device: PathBuf) {
    install_bootloader_common();
    if !device.exists() {
        crash(format!("The device {device:?} does not exist"), 1);
    }
    copy_file("/mnt/usr/share/limine/limine-bios.sys", "/mnt/boot/limine-bios.sys");
    let device = device.to_string_lossy().to_string();
    exec_eval(
        exec_chroot("limine", vec![String::from("bios-install"), device]),
        "install limine as legacy",
    );
}

pub fn install_flatpak() {
    install(vec!["flatpak"]);
    exec_eval(
        exec_chroot(
            "flatpak",
            vec![
                String::from("remote-add"),
                String::from("--if-not-exists"),
                String::from("flathub"),
                String::from("https://flathub.org/repo/flathub.flatpakrepo"),
            ],
        ),
        "add flathub remote",
    )
}

pub fn install_mimalloc() {
    install(vec!["mimalloc"]);
    files_eval(
        files::append_file(
            "/mnt/etc/environment",
            "LD_PRELOAD=/usr/lib/libmimalloc.so\n",
            false,
        ),
        "add LD_PRELOAD for mimalloc",
    );
}
