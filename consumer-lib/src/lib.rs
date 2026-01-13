use std::{fs, path::PathBuf};

pub fn worker(id: usize, rx: std::sync::mpsc::Receiver<PathBuf>) {
    while let Ok(path) = rx.recv() {
        println!("Worker {id} processing {:?}", path);

        if let Ok(contents) = fs::read_to_string(&path) {
            //process file content at this point. Save to the database

            println!("Worker {id} read {} bytes", contents.len());

            //Delete the file after the processing.
            match fs::remove_file(&path) {
                Ok(_) => println!("Worker {id} deleted {:?}", path),
                Err(e) => eprintln!("Worker {id} failed to delete {:?}: {}", path, e),
            }
        } else {
            eprintln!("Worker {id} processing failed {:?}", path);
        }
    }
}
