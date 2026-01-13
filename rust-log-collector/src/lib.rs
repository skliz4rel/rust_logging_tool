use chrono::Local;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::io::{self, BufRead};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Logstore {
    pub application_name: String,
    pub logs: Vec<Log>,
}

impl Logstore {
    pub fn store_in_db(&mut self) {

        //store the logs in the database at this point
        // self.logs
    }

    ///read the database
    pub fn read_file_logs(&mut self, filepath: &PathBuf) {
        let result = fs::read_to_string(filepath);

        match result {
            Ok(content) => {
                let lines = {
                    let mut v: Vec<Log> = content
                        .lines()
                        .map(|l| Log {
                            level: LogLevel::INFO,
                            log_line: l.to_string(),
                            date_time: Local::now().to_string(),
                        })
                        .collect();

                    println!("got here friend {:?}", v);
                    self.logs = v;
                };
            }
            Err(error) => {
                println!("Error reading file {error}");
            }
        };

        //return vec![];
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Directory {
    pub application_name: String,
    pub files: Vec<PathBuf>,
}

impl Directory {
    ///This module is going to read the directory for all the files that exits there
    pub fn read_dir(&mut self, app_name: &str, dir: &str) -> Result<(), Box<dyn Error>> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if path.extension().and_then(|e| e.to_str()) == Some("log") {
                    println!("Log file: {:?}", path);

                    self.files.push(path);
                }

                // println!("Files in the directory location {:?}", store.logs);
            }
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum LogLevel {
    INFO,
    ERROR,
    DEBUG,
    WARN,
    TRACE,
    OTHER,
}

#[derive(Debug)]
pub struct Log {
    pub level: LogLevel,
    pub log_line: String,
    pub date_time: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub application_name: String,
    pub log_location: String,
}

///This would read the config file for application name and log location
pub fn read_config(config_filename: String) -> Result<Vec<Config>, Box<dyn Error>> {
    let data: String = fs::read_to_string(config_filename)?;

    // Parse JSON string into struct
    let config: Vec<Config> = serde_json::from_str(&data).unwrap();
    println!("list here {:?}", config);

    return Ok(config);
}

fn return_log_level(str: &str) -> LogLevel {
    if str.contains("INFO") || str.contains("info") {
        LogLevel::INFO
    } else if str.contains("ERROR") || str.contains("error") {
        LogLevel::ERROR
    } else if str.contains("WARN") || str.contains("warn") {
        LogLevel::WARN
    } else if str.contains("TRACE") || str.contains("trace") {
        LogLevel::TRACE
    } else if str.contains("DEBUG") || str.contains("debug") {
        LogLevel::DEBUG
    } else {
        LogLevel::OTHER
    }
}

/**************************************************TEST MODULES BELOW********************************************************* */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_dir() {
        let filename: String = String::from("config.json");

        let result = read_config(filename);

        match result {
            Ok(list) => {
                if list.len() > 0 {
                    let mut dir: Directory = Directory {
                        application_name: list[0].application_name.clone(),
                        files: Vec::new(),
                    };

                    dir.read_dir(&list[0].application_name, &list[0].log_location);
                }
            }
            Err(error) => {
                panic!("{error}");
            }
        }
    }

    #[test]
    fn test_return_log_level() {
        let line: String =
            String::from("INFO this is the best day of my life friend. Rust is great");

        let result: LogLevel = return_log_level(&line);

        assert_eq!(result, LogLevel::INFO);
    }

    #[test]
    fn test_read_file_logs() {
        let mut store: Logstore = Logstore {
            application_name: String::from("webclient"),
            logs: Vec::new(),
        };

        let path: PathBuf = PathBuf::from("log.txt");
        store.read_file_logs(&path);

        let size: usize = store.logs.len();

        assert_eq!(size, 3);
    }

    #[test]
    fn test_addition() {
        let a = 10;
        let b = 20;
        let result = a + b;

        assert_eq!(result, 30);
    }

    #[test]
    fn test_json_file() {
        let filename: String = String::from("config.json");

        let result: Result<Vec<Config>, Box<dyn Error>> = read_config(filename);

        match result {
            Ok(config_list) => {
                println!("config object \n {:?}", config_list);

                assert_eq!(config_list[0].application_name, "apigateway");
            }
            Err(error) => {
                panic!("Error reading config.json: {error}");
            }
        }
    }
}
