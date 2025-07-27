use std::fs::{OpenOptions, File};
use std::io::{self, Write};
use std::path::Path;

pub enum LogLevel {
    Info,
    Warn,
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Info => write!(f, "[INFO]"),
            LogLevel::Warn => write!(f, "[WARN]"),
            LogLevel::Error => write!(f, "[ERROR]"),
        }
    }
}

pub trait Logger {
    fn log(&mut self, level: LogLevel, message: &str);
}

pub struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&mut self, level: LogLevel, message: &str) {
        println!("{} {}", level, message);
    }
}

pub struct FileLogger {
    file: File,
}

impl FileLogger {
    pub fn new(path: impl AsRef<Path>) -> io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        Ok(FileLogger { file })
    }
}

impl Logger for FileLogger {
    fn log(&mut self, level: LogLevel, message: &str) {
        let formatted = format!("{} {}\n", level, message);
        if let Err(e) = self.file.write_all(formatted.as_bytes()) {
            panic!("Critical error writing to file: {e}");
        }
    }
}
