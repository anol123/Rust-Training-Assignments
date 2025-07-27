mod logger;
mod panic_behavior;

use logger::{ConsoleLogger, FileLogger, Logger, LogLevel};

fn main() {
    // Console logger
    let mut console_logger = ConsoleLogger;
    console_logger.log(LogLevel::Info, "Console log: App started.");
    console_logger.log(LogLevel::Warn, "Console log: Low disk space.");
    console_logger.log(LogLevel::Error, "Console log: Crash imminent.");

    // File logger
    match FileLogger::new("log.txt") {
        Ok(mut file_logger) => {
            file_logger.log(LogLevel::Info, "File log: App started.");
            file_logger.log(LogLevel::Warn, "File log: Disk usage 90%.");
            file_logger.log(LogLevel::Error, "File log: Unrecoverable error.");
        }
        Err(e) => {
            panic!("Failed to initialize file logger: {e}");
        }
    }

    // Test panic behavior if desired
    panic_behavior::trigger_panic_unwind();
    panic_behavior::trigger_panic_abort();
}
