use std::{
    fs,
    path::{Path, PathBuf},
    process,
};

pub fn get_rs_files(dir: &Path) -> Vec<PathBuf> {
    let mut rs_files = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                // rs_files.extend(get_rs_files(&path));
                continue;
            } else if let Some(ext) = path.extension() {
                if ext == "rs" {
                    rs_files.push(path);
                }
            }
        }
    }
    rs_files
}

pub fn get_sysroot() -> String {
    let output = process::Command::new("rustc")
        .arg("--print")
        .arg("sysroot")
        .output()
        .expect("Failed to execute rustc command");

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}
