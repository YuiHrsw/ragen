mod project;
mod utils;
mod watcher;
mod models {
    pub mod crate_model;
    pub mod rust_project_model;
}

use project::generate_rust_project_json;
use std::env;
use std::path::Path;
use std::sync::{atomic::AtomicBool, Arc};
use watcher::{set_exit_handler, start_watcher};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: ragen \x1b[36m<project_path>\x1b[0m");
        std::process::exit(1);
    }

    let path = Box::new(args[1].clone());
    let path: &'static str = Box::leak(path);

    let project_path = Path::new(path);

    generate_rust_project_json(project_path);

    let r1 = Arc::new(AtomicBool::new(true));
    let r2 = r1.clone();

    set_exit_handler(r1);

    start_watcher(project_path, r2);
}
