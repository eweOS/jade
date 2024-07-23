use crate::internal::exec::*;
use crate::internal::*;

pub fn fmt_mount(filesystem: &str, blockdevice: &str, mountpoint: &str) {
    match filesystem {
        "vfat" => exec_eval(
            exec(
                "mkfs.vfat",
                vec![String::from("-F32"), String::from(blockdevice)],
            ),
            format!("Formatting {blockdevice} as vfat").as_str(),
        ),
        "bfs" => exec_eval(
            exec("mkfs.bfs", vec![String::from(blockdevice)]),
            format!("Formatting {blockdevice} as bfs").as_str(),
        ),
        "cramfs" => exec_eval(
            exec("mkfs.cramfs", vec![String::from(blockdevice)]),
            format!("Formatting {blockdevice} as cramfs").as_str(),
        ),
        "ext3" => exec_eval(
            exec("mkfs.ext3", vec![String::from(blockdevice)]),
            format!("Formatting {blockdevice} as ext3").as_str(),
        ),
        "fat" => exec_eval(
            exec("mkfs.fat", vec![String::from(blockdevice)]),
            format!("Formatting {blockdevice} as fat").as_str(),
        ),
        "msdos" => exec_eval(
            exec("mkfs.msdos", vec![String::from(blockdevice)]),
            format!("Formatting {blockdevice} as msdos").as_str(),
        ),
        "xfs" => exec_eval(
            exec("mkfs.xfs", vec![String::from(blockdevice)]),
            format!("Formatting {blockdevice} as xfs").as_str(),
        ),
        "btrfs" => exec_eval(
            exec(
                "mkfs.btrfs",
                vec![String::from("-f"), String::from(blockdevice)],
            ),
            format!("Formatting {blockdevice} as btrfs").as_str(),
        ),
        "ext2" => exec_eval(
            exec("mkfs.ext2", vec![String::from(blockdevice)]),
            format!("Formatting {blockdevice} as ext2").as_str(),
        ),
        "ext4" => exec_eval(
            exec("mkfs.ext4", vec![String::from(blockdevice)]),
            format!("Formatting {blockdevice} as ext4").as_str(),
        ),
        "minix" => exec_eval(
            exec("mkfs.minix", vec![String::from(blockdevice)]),
            format!("Formatting {blockdevice} as minix").as_str(),
        ),
        "f2fs" => exec_eval(
            exec("mkfs.f2fs", vec![String::from(blockdevice)]),
            format!("Formatting {blockdevice} as f2fs").as_str(),
        ),
        "don't format" => {
            log::debug!("Not formatting {}", blockdevice);
        }
        "noformat" => {
            log::debug!("Not formatting {}", blockdevice);
        }
        _ => {
            crash(
                format!("Unknown filesystem {filesystem}, used in partition {blockdevice}"),
                1,
            );
        }
    }
    if !mountpoint.is_empty() {
        exec_eval(
            exec("mkdir", vec![String::from("-p"), String::from(mountpoint)]),
            format!("Creating mountpoint {mountpoint} for {blockdevice}").as_str(),
        );
        mount(blockdevice, mountpoint, "");
    }
}

pub fn mount(partition: &str, mountpoint: &str, options: &str) {
    if !options.is_empty() {
        exec_eval(
            exec(
                "mount",
                vec![
                    String::from(partition),
                    String::from(mountpoint),
                    String::from("-o"),
                    String::from(options),
                ],
            ),
            format!(
                "mount {} with options {} at {}",
                partition, options, mountpoint
            )
            .as_str(),
        );
    } else {
        exec_eval(
            exec(
                "mount",
                vec![String::from(partition), String::from(mountpoint)],
            ),
            format!("mount {} with no options at {}", partition, mountpoint).as_str(),
        );
    }
}

pub fn umount(mountpoint: &str) {
    exec_eval_noerr(
        exec("umount", vec![String::from(mountpoint)]),
        format!("unmount {}", mountpoint).as_str(),
        false,
    );
}

pub fn label(filesystem: &str, blockdevice: &str, label: &str) {
    match filesystem {
        "vfat" => exec_eval(
            exec(
                "fatlabel",
                vec![String::from(blockdevice), String::from(label)],
            ),
            format!("Setting {blockdevice} label to {label}").as_str(),
        ),
        "btrfs" => exec_eval(
            exec(
                "btrfs",
                vec![
                    String::from("filesystem"),
                    String::from("label"),
                    String::from(blockdevice),
                    String::from(label),
                ],
            ),
            format!("Setting {blockdevice} label to {label}").as_str(),
        ),
        "ext4" => exec_eval(
            exec(
                "e2label",
                vec![String::from(blockdevice), String::from(label)],
            ),
            format!("Setting {blockdevice} label to {label}").as_str(),
        ),
        "don't label" => {
            log::debug!("Not setting label to {}", blockdevice);
        }
        "nolabel" => {
            log::debug!("Not setting label to {}", blockdevice);
        }
        _ => {
            crash(
                format!("Unknown filesystem {filesystem}, used in partition {blockdevice}"),
                1,
            );
        }
    }
}

pub fn btrfs_create_subvolume(workdir: &str, volume: &str) {
    exec_eval(
        exec_workdir(
            "btrfs",
            workdir,
            vec![
                String::from("subvolume"),
                String::from("create"),
                String::from(volume),
            ],
        ),
        "Create btrfs subvolume {volume} on {workdir}",
    );
}
