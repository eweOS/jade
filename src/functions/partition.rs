use crate::args;
use crate::args::PartitionMode;
use crate::internal::exec::*;
use crate::internal::files::*;
use crate::internal::filesystem::*;
use crate::internal::*;
use std::path::{Path, PathBuf};

pub fn partition(
    device: PathBuf,
    mode: PartitionMode,
    efi: bool,
    partitions: &mut Vec<args::Partition>,
) {
    println!("{:?}", mode);
    match mode {
        PartitionMode::Auto => {
            if !device.exists() {
                crash(format!("The device {device:?} doesn't exist"), 1);
            }
            exec_eval(
                exec("modprobe", vec![String::from("btrfs")]),
                "Probing btrfs module",
            );
            log::debug!("automatically partitioning {device:?}");
            if efi {
                partition_with_efi(&device);
            } else {
                partition_no_efi(&device);
            }
            if device.to_string_lossy().contains("nvme")
                || device.to_string_lossy().contains("mmcblk")
            {
                part_disk(&device, efi, "p");
            } else {
                part_disk(&device, efi, "");
            }
            update_tinyramfs_config("LABEL=EWE_ROOT", "btrfs");
        }
        PartitionMode::Manual => {
            log::debug!("Manual partitioning");
            partitions.sort_by(|a, b| a.mountpoint.len().cmp(&b.mountpoint.len()));
            for i in 0..partitions.len() {
                println!("{:?}", partitions);
                println!("{}", partitions.len());
                println!("{}", &partitions[i].mountpoint);
                println!("{}", &partitions[i].filesystem);
                println!("{}", &partitions[i].blockdevice);
                if &partitions[i].mountpoint == "/mnt/" {
                    update_tinyramfs_config("LABEL=EWE_ROOT", &partitions[i].filesystem);
                    label(
                        &partitions[i].filesystem,
                        &partitions[i].blockdevice,
                        "EWE_ROOT",
                    );
                }
                fmt_mount(
                    &partitions[i].filesystem,
                    &partitions[i].blockdevice,
                    &partitions[i].mountpoint,
                );
            }
        }
    }
}

fn partition_no_efi(device: &Path) {
    let device = device.to_string_lossy().to_string();
    exec_eval(
        exec(
            "parted",
            vec![
                String::from("-s"),
                String::from(&device),
                String::from("mklabel"),
                String::from("msdos"),
            ],
        ),
        format!("Create msdos label on {}", device).as_str(),
    );
    exec_eval(
        exec(
            "parted",
            vec![
                String::from("-s"),
                String::from(&device),
                String::from("mkpart"),
                String::from("primary"),
                String::from("fat32"),
                String::from("1MIB"),
                String::from("512MIB"),
            ],
        ),
        "create bios boot partition",
    );

    exec_eval(
        exec(
            "parted",
            vec![
                String::from("-s"),
                device,
                String::from("mkpart"),
                String::from("primary"),
                String::from("btrfs"),
                String::from("512MIB"),
                String::from("100%"),
            ],
        ),
        "create btrfs root partition",
    );
}

fn partition_with_efi(device: &Path) {
    let device = device.to_string_lossy().to_string();
    exec_eval(
        exec(
            "parted",
            vec![
                String::from("-s"),
                String::from(&device),
                String::from("mklabel"),
                String::from("gpt"),
            ],
        ),
        format!("create gpt label on {}", &device).as_str(),
    );
    exec_eval(
        exec(
            "parted",
            vec![
                String::from("-s"),
                String::from(&device),
                String::from("mkpart"),
                String::from("fat32"),
                String::from("0"),
                String::from("300"),
            ],
        ),
        "create EFI partition",
    );

    exec_eval(
        exec(
            "parted",
            vec![
                String::from("-s"),
                device,
                String::from("mkpart"),
                String::from("primary"),
                String::from("btrfs"),
                String::from("512MIB"),
                String::from("100%"),
            ],
        ),
        "create btrfs root partition",
    );
}

fn part_disk(device: &Path, efi: bool, partpfx: &str) {
    let device = device.to_string_lossy().to_string();
    fmt_mount("vfat", format!("{}{}1", partpfx, device).as_str(), "");
    fmt_mount("btrfs", format!("{}{}2", partpfx, device).as_str(), "");
    label(
        "btrfs",
        format!("{}{}2", partpfx, device).as_str(),
        "EWE_ROOT",
    );
    mount(format!("{}{}2", partpfx, device).as_str(), "/mnt/", "");
    btrfs_create_subvolume("/mnt", "@");
    btrfs_create_subvolume("/mnt", "@home");
    umount("/mnt");
    mount(
        format!("{}{}2", partpfx, device).as_str(),
        "/mnt/",
        "subvol=@",
    );
    files_eval(files::create_directory("/mnt/boot"), "create /mnt/boot");
    mount(format!("{}{}1", partpfx, device).as_str(), "/mnt/boot", "");
    files_eval(files::create_directory("/mnt/home"), "create /mnt/home");
    mount(
        format!("{}{}2", partpfx, device).as_str(),
        "/mnt/home",
        "subvol=@home",
    );
    if efi {
        files_eval(
            files::create_directory("/mnt/boot/efi"),
            "create /mnt/boot/efi",
        );
    }
}

fn update_tinyramfs_config(root: &str, filesystem: &str) {
    exec_eval(
        exec_workdir(
            "mkdir",
            "/mnt",
            vec![String::from("-p"), String::from("etc/tinyramfs")],
        ),
        "Create tinyramfs initial directory",
    );
    create_file("/mnt/etc/tinyramfs/config");

    let config_root = format!("root={root}");
    let config_root_type = format!("root_type={filesystem}");
    let mut config = vec![
        "# generated by jade",
        "hooks=mdev,plymouth",
        "compress='gzip -9'",
        config_root.as_str(),
        config_root_type.as_str(),
    ];
    if filesystem == "btrfs" {
        config.push("root_opts=subvol=@");
    }
    files_eval(
        append_file(
            "/mnt/etc/tinyramfs/config",
            config.join("\n").as_str(),
            false,
        ),
        "write tinyramfs config",
    )
}
