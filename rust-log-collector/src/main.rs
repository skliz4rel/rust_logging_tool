use rust_log_collector::{Config, Directory, Logstore};
use std::fs::read_dir;
use std::sync::Arc;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let filename: String = String::from("config.json");

    let result = rust_log_collector::read_config(filename);

    match result {
        Ok(list) => {
            let dirCount: usize = list.len();

            println!("got here dir count {dirCount}");

            if dirCount > 0 {
                //read_dir(&list[0].application_name, &list[0].log_location);

                multiple_transmitter_receiver(dirCount, &list);
            }
        }
        Err(error) => {
            panic!("At this point {error}");
        }
    }
}

pub fn multiple_transmitter_receiver(count: usize, list: &Vec<Config>) {
    let (tx, rx) = mpsc::channel();

    for i in 0..count {
        // Clone the specific Config so the thread owns it
        let cfg = list[i].clone();
        let producer = tx.clone();

        thread::spawn(move || {
            let mut dir = Directory {
                application_name: cfg.application_name.clone(),
                files: Vec::new(),
            };

            dir.read_dir(&cfg.application_name, &cfg.log_location);

            producer.send(dir).unwrap();
        });
    }

    drop(tx); // Close the original sender so rx will end after all clones drop

    for dir in rx {
        read_files_store_in_db(&dir);
    }
}

fn read_files_store_in_db(dir: &Directory) {
    let mut store: Logstore = Logstore {
        application_name: dir.application_name.to_string(),
        logs: Vec::new(),
    };

    for path in &dir.files {
        store.read_file_logs(&path);
    }

    //store the values in the database

    store.store_in_db();
}
