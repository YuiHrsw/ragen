use crate::project::generate_rust_project_json;
use hotwatch::{Event, EventKind, Hotwatch};
use std::path::Path;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;

pub fn set_exit_handler(running: Arc<AtomicBool>) {
    ctrlc::set_handler(move || {
        println!("Received Ctrl+C, exiting...");
        running.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");
}

pub fn start_watcher(project_path: &'static Path, running: Arc<AtomicBool>) {
    let mut hotwatch = Hotwatch::new().expect("hotwatch failed to initialize!");

    let project_path_clone: &Path = Box::leak(Box::new(project_path));

    hotwatch
        .watch(project_path_clone, |event: Event| {
            if let EventKind::Create(_) | EventKind::Remove(_) = event.kind {
                for path in event.paths {
                    if let Some(ext) = path.extension() {
                        if ext == "rs" {
                            println!("Detected change in .rs file: {:?}", path);
                            generate_rust_project_json(project_path_clone);
                        }
                    }
                }
            }
        })
        .expect("Failed to watch directory");

    println!("Watching for changes... Press Ctrl+C to stop.");

    while running.load(Ordering::SeqCst) {
        thread::sleep(std::time::Duration::from_millis(100));
    }

    println!("Exiting program.");
}
