//! # Logging Macros
//!
//! A set of macros for logging at different levels (`INFO`, `ERROR`, `WARN`, `DEBUG`, and `TRACE`).
//!
//! These macros simplify logging by automatically specifying the log level and optionally formatting the message.
//! They rely on the global logging configuration (`CONFIG`) and the `log` function from the logging module.
//!
//! ## Usage
//!
//! Use these macros to log messages at different levels. Example:
//! ```rust
//! use lumalog::{info, error, warn, debug, trace};
//!
//! info!("This is an informational message.");
//! warn!("This is a warning.");
//! error!("This is an error: {}", "details here");
//! debug!("Debugging variable: {:?}", some_variable);
//! trace!("Tracing details.");
//! ```

/// Logs an informational message (`INFO` level).
///
/// - Messages at this level are styled and logged if the global configuration's log level is `INFO` or lower.
/// - Use the macro with or without arguments for formatting.
///
/// # Examples
/// ```rust
/// info!("This is an informational message.");
/// info!("User {} has logged in.", username);
/// ```
#[macro_export]
macro_rules! info {
    ($message: expr, $($arg:tt)+) => (
        $crate::log::log(
            $crate::LogLevel::INFO,
            format!($message, $($arg)+).as_str(),
        );
    );
    ($message: expr) => (
        $crate::log::log($crate::LogLevel::INFO, $message);
    )
}

/// Logs an error message (`ERROR` level).
///
/// - Messages at this level are logged regardless of the global configuration's log level.
/// - Use the macro with or without arguments for formatting.
///
/// # Examples
/// ```rust
/// error!("An error occurred.");
/// error!("Failed to open file: {}", file_path);
/// ```
#[macro_export]
macro_rules! error {
    ($message: expr, $($arg:tt)+) => (
        $crate::log::log(
            $crate::LogLevel::ERROR,
            format!($message, $($arg)+).as_str(),
        );
    );
    ($message: expr) => (
        $crate::log::log($crate::LogLevel::ERROR, $message);
    )
}

/// Logs a warning message (`WARN` level).
///
/// - Messages at this level are styled and logged if the global configuration's log level is `WARN` or lower.
/// - Use the macro with or without arguments for formatting.
///
/// # Examples
/// ```rust
/// warn!("This is a warning.");
/// warn!("Disk space is low: {}% remaining.", free_space_percentage);
/// ```
#[macro_export]
macro_rules! warn {
    ($message: expr, $($arg:tt)+) => (
        $crate::log::log(
            $crate::LogLevel::WARN,
            format!($message, $($arg)+).as_str(),
        );
    );
    ($message: expr) => (
        $crate::log::log($crate::LogLevel::WARN, $message);
    )
}

/// Logs a debug message (`DEBUG` level).
///
/// - Messages at this level are logged if the global configuration's log level is `DEBUG` or lower.
/// - Use the macro with or without arguments for formatting.
///
/// # Examples
/// ```rust
/// debug!("Debugging variable: {:?}", some_variable);
/// debug!("Entered function: {}", function_name);
/// ```
#[macro_export]
macro_rules! debug {
    // Formatted message case
    ($message:expr, $($arg:tt)+) => {
        $crate::log::log(
            $crate::LogLevel::DEBUG,
            format!($message, $($arg)+).as_str(),
        );
    };
    // Simple message case
    ($message:expr) => {
        $crate::log::log($crate::LogLevel::DEBUG, $message);
    };
}

/// Logs a trace message (`TRACE` level).
///
/// - Messages at this level are logged if the global configuration's log level is `TRACE` or lower.
/// - Use this macro for highly detailed logs intended for tracing program flow.
///
/// # Examples
/// ```rust
/// trace!("Entering function.");
/// trace!("Current state: {:?}", state);
/// ```
///
/// # Note
/// This is currently a simple implementation and may need enhancements for advanced tracing needs.
#[macro_export]
macro_rules! trace {
    // Formatted message case
    ($message:expr, $($arg:tt)+) => {
        $crate::log::log(
            $crate::LogLevel::TRACE,
            format!($message, $($arg)+).as_str(),
        );
    };
    // Simple message case
    ($message:expr) => {
        $crate::log::log($crate::LogLevel::TRACE, $message);
    };
}