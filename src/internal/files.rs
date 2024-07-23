use crate::internal::*;
use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

pub fn create_file(path: &str) {
    let returncode = File::create(path);
    match returncode {
        Ok(_) => {
            log::info!("Create {}", path);
        }
        Err(e) => {
            crash(format!("Create {}: Failed with error {}", path, e), 1);
        }
    }
}

pub fn copy_file(path: &str, destpath: &str) {
    let return_code = std::fs::copy(path, destpath);
    match return_code {
        Ok(_) => {
            log::info!("Copy {} to {}", path, destpath);
        }
        Err(e) => {
            crash(
                format!("Copy {} to {}: Failed with error {}", path, destpath, e),
                1,
            );
        }
    }
}

pub fn copy_file_wildcard(str_path: &str, str_destpath: &str, ext: &str) {
    let path = Path::new(str_path);
    let destpath = Path::new(str_destpath);

    if !path.exists() {
        crash(
            format!(
                "Copy {} to {}: source dir not exists",
                str_path, str_destpath
            ),
            1,
        );
    }

    if !destpath.exists() {
        crash(
            format!("Copy {} to {}: dest dir not exists", str_path, str_destpath),
            1,
        );
    }

    for entry in path.read_dir().unwrap() {
        let entry = entry.unwrap();
        let file_path = entry.path();

        if !file_path.is_file() {
            continue;
        }

        if !file_path.extension().unwrap().eq_ignore_ascii_case(ext) {
            continue;
        }

        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        let dest_file_path = destpath.join(file_name);

        if dest_file_path.exists() {
            continue;
        }

        match fs::copy(file_path, dest_file_path) {
            Ok(_) => {}
            Err(e) => {
                crash(format!("Copy {} to {}: {}", str_path, str_destpath, e), 1);
            }
        }
    }
}

pub fn append_file(path: &str, content: &str, linebreak: bool) -> std::io::Result<()> {
    log::info!("Append '{}' to file {}", content.trim_end(), path);
    let mut file = OpenOptions::new().append(true).open(path)?;
    if linebreak {
        file.write_all(format!("\n{content}\n").as_bytes())?;
    } else {
        file.write_all(format!("{content}").as_bytes())?;
    }
    Ok(())
}

pub fn sed_file(path: &str, find: &str, replace: &str) -> std::io::Result<()> {
    log::info!("Sed '{}' to '{}' in file {}", find, replace, path);
    let contents = fs::read_to_string(path)?;
    let new_contents = contents.replace(find, replace);
    let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;
    file.write_all(new_contents.as_bytes())?;
    Ok(())
}

pub fn create_directory(path: &str) -> std::io::Result<()> {
    std::fs::create_dir_all(path)
}
