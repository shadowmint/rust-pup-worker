use ::base_logging::Logger;
pub use ::base_logging::Level;
use ::base_logging::LogFormatter;
use ::base_logging::loggers::ConsoleLogger;
use std::collections::HashMap;
use ::time::Tm;
use std::sync::Mutex;
use errors::PupWorkerError;

/// The global logging configuration
struct LogConfig {
    level: Level,
    factory: fn() -> Logger,
}

/// The global logging configuration instance
lazy_static! {
    static ref CONFIG: Mutex<LogConfig> = Mutex::new(LogConfig {
        level: Level::Info,
        factory: default_logger
    });
}

pub fn get_logger() -> Result<Logger, PupWorkerError> {
    let level_ref = PupWorkerError::wrap(CONFIG.lock())?;
    return Ok((level_ref.factory)());
}

pub fn default_logger() -> Logger {
    return Logger::new().with_format(PupFormatter {}).with(ConsoleLogger::new());
}

pub fn set_logger_level(level: Level) -> Result<(), PupWorkerError> {
    let mut level_ref = PupWorkerError::wrap(CONFIG.lock())?;
    level_ref.level = level;
    Ok(())
}

pub fn set_logger(factory: fn() -> Logger) -> Result<(), PupWorkerError> {
    let mut level_ref = PupWorkerError::wrap(CONFIG.lock())?;
    level_ref.factory = factory;
    Ok(())
}

/// A custom formatter type.
struct PupFormatter {}

impl LogFormatter for PupFormatter {
    fn log_format(&self, level: Level, _timestamp: Tm, message: Option<&str>, _properties: Option<HashMap<&str, &str>>) -> String {
        if level <= CONFIG.lock().unwrap().level {
            return match message {
                Some(m) => String::from(m),
                None => String::new()
            };
        };
        return String::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_logger() {
        set_logger_level(Level::Debug).unwrap();
        let mut l = get_logger().unwrap();
        l.log(Level::Debug, "Debug!");
        l.log(Level::Info, "Info");
    }

    #[test]
    fn test_custom_logger() {
        set_logger(custom_logger).unwrap();
        set_logger_level(Level::Info).unwrap();
        let mut l = get_logger().unwrap();
        l.log(Level::Debug, "Debug!");
        l.log(Level::Info, "Info");
    }

    fn custom_logger() -> Logger {
        return Logger::new().with(ConsoleLogger::new());
    }
}
