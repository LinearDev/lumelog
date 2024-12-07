//! # LumaLog Logging System
//!
//! A flexible logging system that supports:
//! - Multiple log levels (`INFO`, `WARN`, `ERROR`, etc.).
//! - Optional styling for console output using `colored`.
//! - File-based logging with text or JSON formats.
//!
//! ## Features
//! - Styled console logs with colors for different log levels.
//! - Configurable file-based logging.
//! - Automatic inclusion of timestamps.
//!
//! ## Usage
//! 1. Initialize the global `CONFIG` using a `ConfigBuilder`.
//! 2. Use the `log` function to log messages.
//!
//! Example:
//! ```rust
//! use lumalog::{info, error, warn, debug, trace};
//!
//! info!("This is an informational message.");
//! warn!("This is a warning.");
//! error!("This is an error: {}", "details here");
//! debug!("Debugging variable: {:?}", some_variable);
//! trace!("Tracing details.");
//! ```
use std::{fs::OpenOptions, io::Write};
use colored::Colorize;

pub use std::{format_args, time::{Duration, SystemTime}};

use crate::{Config, FileLogger, FileLoggerFormat, LogLevel, CONFIG};

/// Logs a message at the specified log level.
///
/// This function checks the global configuration (`CONFIG`) to determine if the
/// log level is enabled and directs the message to the console and/or a file
/// based on the configuration.
///
/// - If logging to `std` is enabled, the log will be styled and printed to the console.
/// - If file logging is enabled, the log will be written to the specified file.
///
/// # Arguments
/// * `level` - The log level for the message.
/// * `message` - The message to be logged.
pub fn log(level: LogLevel, message: &str) {
    let config_ref: &Config = match CONFIG.get() {
        Some(c) => c,
        None => {
            print!("[ FATAL ] LumaLog: Config has not been initialized!\n");
            return;
        }
    };
    
    if !cfg!(debug_assertions) && !config_ref.log_in_release && level >= LogLevel::DEBUG {
        return;
    }

    if level > config_ref.log_level {
        return;
    }
    
    let (log_format_styled, log_format) = formatter(config_ref, level, message);
    
    if config_ref.std {
        print!("{}\n", log_format_styled);
    }

    if config_ref.file_logger_config.enabled {
        match file_log(config_ref.file_logger_config.clone(), log_format) {
            Ok(_) => {},
            Err(err) => {
                print!(
                    "{}\n", 
                    formatter(
                        config_ref,
                        LogLevel::ERROR,
                        &format!("Can not save log information in file: {}", err)
                    ).0
                );
            }
        };
    }
}

/// Styles a log level with colors for console output.
///
/// # Arguments
/// * `level` - The log level to be styled.
///
/// # Returns
/// A styled string representation of the log level.
fn style_level(level: LogLevel) -> String {
    if level == LogLevel::INFO {
        return format!(" {:?} ", level).on_white().to_string();
    }

    if level == LogLevel::WARN {
        return format!(" {:?} ", level).on_bright_yellow().to_string();
    }

    if level == LogLevel::ERROR {
        return format!(" {:?} ", level).on_red().to_string();
    }

    if level == LogLevel::DEBUG {
        return format!(" {:?} ", level).on_cyan().to_string();
    }

    if level == LogLevel::TRACE {
        return format!(" {:?} ", level).on_bright_black().to_string();
    }

    return " UNKNOWN ".to_string();
}

/// Generates a styled and plain-text timestamp.
///
/// # Returns
/// A tuple of `(styled_time, plain_time)`.
fn style_time() -> (String, String) {
    let time: Duration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let time_format: Vec<String> = chrono::DateTime::from_timestamp_nanos(time.as_nanos() as i64).to_rfc3339().split(".").map(|s| s.to_string()).collect();
    let to_print: String = time_format[0].replace("T", " ").replace("-", "/");

    (format!("{}", to_print).on_bright_black().to_string(), format!(" {} ", to_print).to_string())
}

/// Formats the log message for console and file output.
///
/// # Arguments
/// * `config_ref` - A reference to the global configuration.
/// * `level` - The log level of the message.
/// * `message` - The message to be logged.
///
/// # Returns
/// A tuple of `(styled_message, plain_message)`.
fn formatter(config_ref: &Config, level: LogLevel, message: &str) -> (String, String) {
    match config_ref.file_logger_config.log_format {
        FileLoggerFormat::TEXT => {
            if config_ref.log_with_time {
                return (
                    format!("{} {} {}", style_time().0, style_level(level.clone()), message),
                    format!("{} {:?} {}", style_time().1, level, message)
                );
            } else {
                return (
                    format!("{} {}", style_level(level.clone()), message),
                    format!("{:?} {}", level, message)
                );
            }
        },
        FileLoggerFormat::JSON => {
            //TODO: now its empty implementation
            if config_ref.log_with_time {
                return (
                    format!("{} {} {}", style_time().0, style_level(level.clone()), message),
                    format!("{} {:?} {}", style_time().1, level, message)
                );
            } else {
                return (
                    format!("{} {}", style_level(level.clone()), message),
                    format!("{:?} {}", level, message)
                );
            }
        },
    }
}

/// Writes a log message to a file.
///
/// # Arguments
/// * `config_ref` - The file logger configuration.
/// * `message` - The log message to write.
///
/// # Returns
/// `Ok(())` if the message was written successfully, or an error string if writing fails.
fn file_log(config_ref: FileLogger, message: String) -> Result<(), String> {
    let mut file = match OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .read(true)
        .open(&config_ref.path) {
            Ok(f) => f,
            Err(err) => {
                return Err(err.to_string())
            }
        };

    match file.write(format!("{}\n", message).as_bytes()) {
        Ok(_) => {},
        Err(err) => {
            return Err(err.to_string());
        }
    };

    Ok(())
}