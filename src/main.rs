use hotwatch::{Event, EventKind, Hotwatch};
use serde::Serialize;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;

#[derive(Serialize)]
struct RustProject {
    sysroot: String,
    sysroot_src: String,
    crates: Vec<Crate>,
}

#[derive(Serialize)]
struct Crate {
    root_module: String,
    edition: String,
    deps: Vec<String>,
}

fn get_rs_files(dir: &Path) -> Vec<PathBuf> {
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

fn get_sysroot() -> String {
    let output = process::Command::new("rustc")
        .arg("--print")
        .arg("sysroot")
        .output()
        .expect("Failed to execute rustc command");

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn generate_rust_project_json(project_path: &Path) {
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

    // 创建 .vscode 文件夹并生成 settings.json
    let vscode_path = project_path.join(".vscode");
    fs::create_dir_all(&vscode_path).unwrap();

    // 构建 settings.json 的 JSON 对象
    let settings_json = serde_json::json!({
        "rust-analyzer.linkedProjects": [
            json_content
        ]
    });

    // 格式化 settings.json
    let settings = serde_json::to_string_pretty(&settings_json).unwrap();

    let settings_file_path = vscode_path.join("settings.json");
    let mut settings_file = File::create(settings_file_path).unwrap();
    settings_file.write_all(settings.as_bytes()).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: ragen <project_path>");
        std::process::exit(1);
    }

    let path = Box::new(args[1].clone());
    let path: &'static str = Box::leak(path);

    let project_path = Path::new(path);

    let mut hotwatch = Hotwatch::new().expect("hotwatch failed to initialize!");

    generate_rust_project_json(project_path); // 初次生成配置文件

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // 捕捉 Ctrl+C 信号以退出程序
    ctrlc::set_handler(move || {
        println!("Received Ctrl+C, exiting...");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // 启动文件监听器
    hotwatch
        .watch(project_path, move |event: Event| {
            if let EventKind::Create(_) | EventKind::Remove(_) = event.kind {
                for path in event.paths {
                    if let Some(ext) = path.extension() {
                        if ext == "rs" {
                            println!("Detected change in .rs file: {:?}", path);
                            generate_rust_project_json(project_path);
                        }
                    }
                }
            }
        })
        .expect("Failed to watch directory");

    println!("Watching for changes... Press Ctrl+C to stop.");

    // 主循环，等待退出信号
    while running.load(Ordering::SeqCst) {
        thread::sleep(std::time::Duration::from_millis(100));
    }

    println!("Exiting program.");
}
