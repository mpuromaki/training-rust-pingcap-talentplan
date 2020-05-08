#![allow(dead_code)]
#![allow(unused_variables)]

use anyhow::{self, Result};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*, BufRead, BufReader, SeekFrom, Write};
use std::path::PathBuf;
use std::{thread, time};

/// Serialisation datatype definition
#[derive(Serialize, Deserialize, Debug)]
struct LogLine {
    cmd: String,
    key: String,
    value: String,
}

impl LogLine {
    pub fn new(cmd: String, key: String, value: String) -> LogLine {
        LogLine { cmd, key, value }
    }
}

/// Key-value store
///
/// Contains datastore and functions to set and get values.
#[derive(Debug)]
pub struct KvStore {
    key_row_map: HashMap<String, i32>,
    wal_path: PathBuf,
    wal_handle: File,
}

impl KvStore {
    /// Creates new key-value store.
    pub fn new(filepath: PathBuf) -> KvStore {
        // Open write-ahead-log file
        let wal_path = filepath.join("kvdb.wal");
        //println!("Trying to open path: {:?}", wal_path);
        let wal_handle = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(&wal_path)
            .expect("Could not open file.");

        //println!("Opened write-ahead-log: {:?}", &wal_path);

        // Create and return the kv-store
        KvStore {
            key_row_map: HashMap::new(),
            wal_path: wal_path,
            wal_handle: wal_handle,
        }
    }

    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        // Create kvstore
        let filepath = path.into();
        let kvstore = KvStore::new(filepath);

        // Create KvStore and return it
        Ok(kvstore)
    }

    /// Save value to key-value store.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        //println!("SET {} to {}", &key, &value);

        // Create "SET" command
        let cmd = LogLine::new("set".to_owned(), key.clone(), value.clone());

        // Serialise that to a string
        let cmd_string = serde_json::to_string(&cmd).unwrap();
        let cmd_string = [cmd_string, "\n".to_owned()].join(""); // Add line break

        // Seek to end of file
        self.wal_handle.seek(SeekFrom::End(0)).unwrap();

        // Write the command to the write-ahead-log
        self.wal_handle.write_all(cmd_string.as_bytes()).unwrap();

        // Return
        Ok(())
    }

    /// Return value from key-value store.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        //println!("GET {}", &key);

        // Load the WAL file
        self.load_wal_to_memory();

        // Get pointer from key_row_map for requested key
        let fetch_line = self.key_row_map.get(&key);

        match fetch_line {
            Some(fetch_line) => {
                // Fetch value from write-ahead-log based on pointer
                self.wal_handle.seek(SeekFrom::Start(0)).unwrap();
                let mut reader = BufReader::new(&self.wal_handle);
                let line = reader
                    .lines()
                    .nth(fetch_line.to_owned().try_into().unwrap())
                    .unwrap()
                    .unwrap();
                let decoded: LogLine = serde_json::from_str(&line).unwrap();
                // Return the requested value
                return Ok(Some(decoded.value));
            }
            None => return Ok(None),
        }
    }

    /// Remove value from key-value store.
    pub fn remove(&mut self, key: String) -> Result<()> {
        // Load the WAL file
        self.load_wal_to_memory();

        // Check if the key is in the index
        //println!("Finding key {:?} to remove", &key);
        let fetch_line = self.key_row_map.get(&key);
        //println!("removing {:?}", fetch_line);

        match fetch_line {
            Some(line) => {
                // Key exists
                // Remove key from index
                let _ = self.key_row_map.remove(&key);

                // Add RM line to WAL
                let rmline = LogLine {
                    cmd: "rm".to_owned(),
                    key: key,
                    value: "".to_owned(),
                };
                let jsonstr = serde_json::to_string(&rmline).expect("Could not serialize");
                let _ = self.wal_handle.seek(SeekFrom::End(0));
                let _ = self.wal_handle.write_all(jsonstr.as_ref());

                Ok(())
            }
            None => anyhow::bail!(""),
        }
    }

    fn load_wal_to_memory(&mut self) {
        // Seek to beginning of file
        self.wal_handle.seek(SeekFrom::Start(0)).unwrap();

        // Read the entire write-ahead-log and update key_row_map
        let mut reader = BufReader::new(&self.wal_handle);

        for (row, line) in reader.lines().enumerate() {
            match line {
                Ok(line) => {
                    // Decode the line
                    let decoded: LogLine = serde_json::from_str(&line).unwrap();

                    // Update key_row_map based on lines
                    match decoded.cmd.as_str() {
                        "set" => {
                            let _ = self.key_row_map.insert(decoded.key.clone(), row as i32);
                        }
                        "rm" => {
                            let _ = self.key_row_map.remove(&decoded.key);
                        }
                        _ => {}
                    }
                }
                Err(e) => panic!(e),
            }
        }
    }
}
