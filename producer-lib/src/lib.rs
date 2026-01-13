use std::{
    fs,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::Sender,
    },
    thread,
    time::Duration,
};

pub fn scan_directories(dirs: Vec<PathBuf>, shutdown: Arc<AtomicBool>, tx: Sender<PathBuf>) {
    let mut seen = std::collections::HashSet::new();

    while !shutdown.load(Ordering::SeqCst) {
        for dir in &dirs {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();

                    if path.is_file() && seen.insert(path.clone()) {
                        let _ = tx.send(path);
                    }
                }
            }
        }
    }

    thread::sleep(Duration::from_secs(2));
}
