use std::io::Write;
use std::process::{Command, Stdio};

pub fn exec(command: &str, args: Vec<String>) -> Result<std::process::ExitStatus, std::io::Error> {
    let returncode = Command::new(command).args(args).status();
    returncode
}

pub fn exec_chroot(
    command: &str,
    args: Vec<String>,
) -> Result<std::process::ExitStatus, std::io::Error> {
    let mut fullargs = vec![String::from("/mnt"), String::from(command)];
    fullargs.extend(args);
    let returncode = Command::new("arch-chroot").args(fullargs).status();
    returncode
}

pub fn exec_chroot_stdin(
    command: &str,
    args: Vec<String>,
    input: &str,
) -> Result<std::process::ExitStatus, std::io::Error> {
    let mut fullargs = vec![String::from("/mnt"), String::from(command)];
    fullargs.extend(args);
    let mut exec_child = Command::new("arch-chroot")
        .stdin(Stdio::piped())
        .args(fullargs)
        .spawn()
        .unwrap();
    let mut stdin = exec_child.stdin.take().unwrap();
    stdin.write_all(format!("{input}\n").as_bytes()).unwrap();
    drop(stdin);
    let returncode = exec_child.wait();
    returncode
}

pub fn exec_workdir(
    command: &str,
    workdir: &str,
    args: Vec<String>,
) -> Result<std::process::ExitStatus, std::io::Error> {
    let returncode = Command::new(command)
        .args(args)
        .current_dir(workdir)
        .status();
    returncode
}
