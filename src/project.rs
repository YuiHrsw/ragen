use crate::{
    models::{crate_model::Crate, rust_project_model::RustProject},
    utils::{get_rs_files, get_sysroot},
};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

pub fn generate_rust_project_json(project_path: &Path) {
    let sysroot = get_sysroot();
    let sysroot_src = format!("{}\\lib\\rustlib\\src\\rust\\library", sysroot);
    let rs_files = get_rs_files(project_path);

    let crates: Vec<Crate> = rs_files
        .iter()
        .map(|path| Crate {
            root_module: path.to_string_lossy().to_string(),
            edition: "2021".to_string(),
            deps: Vec::new(),
        })
        .collect();

    let rust_project = RustProject {
        sysroot,
        sysroot_src,
        crates,
    };

    let json_content = serde_json::to_value(&rust_project).unwrap();

    // create .vscode folder & generate settings.json
    let vscode_path = project_path.join(".vscode");
    fs::create_dir_all(&vscode_path).unwrap();

    let settings_json = serde_json::json!({
        "rust-analyzer.linkedProjects": [
            json_content
        ]
    });

    // format settings.json
    let settings = serde_json::to_string_pretty(&settings_json).unwrap();

    let settings_file_path = vscode_path.join("settings.json");
    let mut settings_file = File::create(settings_file_path).unwrap();
    settings_file.write_all(settings.as_bytes()).unwrap();
}
