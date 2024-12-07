//! # Logging Configuration Module
//! 
//! This module provides a configurable logging system with support for different log levels, 
//! time-stamping logs, and file logging in multiple formats.
//!
//! ## Features
//! - Adjustable log levels (`ERROR`, `WARN`, `INFO`, `DEBUG`, `TRACE`).
//! - Optional time-stamping for log output.
//! - Configurable file-based logging with support for JSON and text formats.
//! - Centralized configuration using a static instance (`OnceCell`).
//!
//! ## Usage
//! 1. Build a configuration using `ConfigBuilder`.
//! 2. Use macros for logging (`info!`, `debug!`, etc.).

use once_cell::sync::OnceCell;

#[macro_use]
mod macros;

#[doc(hidden)]
pub mod log;

/// Global configuration instance.
static CONFIG: OnceCell<Config> = OnceCell::new();

/// Represents the severity level of logs.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum LogLevel {
    /// Errors that cause premature termination of operations.
    ERROR,
    /// Warnings that indicate potential issues.
    WARN,
    /// Informational messages for normal operation.
    INFO,
    /// Debug messages for detailed troubleshooting.
    DEBUG,
    /// Trace-level logs for in-depth diagnostics.
    TRACE
}

/// Represents the file log format.
#[derive(Clone, Debug)]
pub enum FileLoggerFormat {
    /// JSON-formatted log output.
    JSON,
    /// Plain text log output.
    TEXT
}

/// Builder for configuring a file logger.
pub struct FileLoggerBuilder {
    /// Enables or disables file logging.
    pub enabled: bool,
    /// Path to the log file.
    pub path: Option<String>,
    /// Format of the log file.
    pub log_format: Option<FileLoggerFormat>,
}

impl FileLoggerBuilder {
    /// Creates a new `FileLoggerBuilder` with default settings.
    pub fn new() -> FileLoggerBuilder {
        FileLoggerBuilder {
            enabled: false,
            path: None,
            log_format: Some(FileLoggerFormat::TEXT)
        }
    }

    /// Sets the path for the log file.
    pub fn path(mut self, path: String) -> FileLoggerBuilder {
        self.path = Some(path);
        self
    }

    /// Sets the format for the log file.
    pub fn log_format(mut self, log_format: FileLoggerFormat) -> FileLoggerBuilder {
        self.log_format = Some(log_format);
        self
    }
    
    /// Builds the `FileLogger` instance.
    pub fn build(self) -> Result<FileLogger, String> {
        Ok(FileLogger {
            enabled: self.enabled,
            path: if self.path.is_none() {String::from("log.txt")} else {self.path.unwrap()},
            log_format: if self.log_format.is_none() {FileLoggerFormat::TEXT} else {self.log_format.unwrap()}
        })
    }
}

/// Represents a file logger configuration.
#[derive(Clone, Debug)]
pub struct FileLogger {
    /// Indicates whether file logging is enabled.
    enabled: bool,
    /// Path to the log file.
    path: String,
    /// Format of the log file.
    log_format: FileLoggerFormat,
}

/// Builder for configuring the logging system.
pub struct ConfigBuilder {
    /// Log level for the logger.
    pub log_level: Option<LogLevel>,
    /// Enables or disables logging in release mode.
    pub log_in_release: bool,
    /// Enables or disables time-stamping in logs.
    pub log_with_time: Option<bool>,
    /// Enables or disables standard output logging.
    pub std: bool,
    /// Configuration for file-based logging.
    pub file_logger_config: Option<FileLoggerBuilder>,
}

/// Represents the full logging configuration.
#[derive(Clone, Debug)]
pub struct Config {
    /// Log level for the logger.
    log_level: LogLevel,

    log_in_release: bool,
    /// Whether time-stamping is enabled.
    log_with_time: bool,
    /// Whether standard output logging is enabled.
    std: bool,
    /// File logger configuration.
    file_logger_config: FileLogger,
}

impl ConfigBuilder {
    /// Creates a new `ConfigBuilder` with default settings.
    pub fn new() -> ConfigBuilder {
        ConfigBuilder {
            log_level: Some(LogLevel::INFO),
            log_in_release: false,
            log_with_time: Some(true),
            std: true,
            file_logger_config: None,
        }
    }

    pub fn log_level(mut self, level: Option<LogLevel>) -> Self {
        self.log_level = level;
        self
    }

    pub fn log_with_time(mut self, log_with_time: Option<bool>) -> Self {
        self.log_with_time = log_with_time;
        self
    }

    pub fn std(mut self, std: bool) -> Self {
        self.std = std;
        self
    }

    pub fn file_logger_config(mut self, file_logger_config: Option<FileLoggerBuilder>) -> Self {
        self.file_logger_config = file_logger_config;
        self
    }

    /// Builds the `Config` instance and sets it in the global `CONFIG`.
    pub fn build(self) -> Result<Config, String>{
        let config: Config = Config {
            log_level: if self.log_level.is_none() {
                    LogLevel::INFO
                } else {
                    self.log_level.unwrap()
                },
            log_in_release: self.log_in_release,
            log_with_time: self.log_with_time.unwrap(),
            std: self.std,
            file_logger_config: if self.file_logger_config.is_none() {
                    FileLoggerBuilder::new().build().unwrap()
                } else {
                    self.file_logger_config.unwrap().build().unwrap()
                },
        };

        CONFIG.set(config.clone()).unwrap();
        Ok(config)
    }
}