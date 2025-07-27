use super::record::Record;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub struct KVStore {
    records: Vec<Box<Record>>,
}

impl KVStore {
    pub fn load_from_file(path: &str) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut records = Vec::new();

        for line in reader.lines() {
            if let Ok(line) = line {
                if let Some((k, v)) = line.trim().split_once('=') {
                    records.push(Box::new(Record::new(k.to_string(), v.to_string())));
                }
            }
        }

        Ok(Self { records })
    }

    pub fn insert(&mut self, key: String, value: String, file: &mut File) -> std::io::Result<()> {
        let record = Box::new(Record::new(key.clone(), value.clone()));
        writeln!(file, "\"{}={}\"", key, value)?;
        self.records.push(record);
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.records.iter().rev().find(|r| r.key == key).map(|r| r.value.as_str())
    }

    pub fn all_records(&self) -> &[Box<Record>] {
        &self.records
    }
}

impl Drop for KVStore {
    fn drop(&mut self) {
        println!("KVStore dropped. {} records were in memory.", self.records.len());
    }
}
