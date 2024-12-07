# LumeLog

LumeLog is a lightweight, flexible, and configurable logging library for Rust. It supports different log levels, runtime configuration, and styled log outputs, making it suitable for both development and production environments.

## Features
- **Log Levels**: Supports `ERROR`, `WARN`, `INFO`, `DEBUG`, and `TRACE`.
- **Styled Output**: Provides color-coded logs for enhanced readability.
- **File Logging**: Optionally logs messages to a file in text or JSON format.
- **Runtime Configuration**: Configure logging behavior without recompiling.
- **Build Mode Detection**: Automatically adjusts logging for debug or release builds.

## Installation

Add `lumelog` to your `Cargo.toml`:

```toml
[dependencies]
lumelog = "0.1.1"
```

Then, include it in your code:

```rust
use lumelog::{info, warn, error, ConfigBuilder, LogLevel};
```

## Quick Start
Here’s a basic example to get started:

```rust
use lumelog::{info, error, warn, debug, ConfigBuilder, LogLevel};

fn main() {
    // Configure LumeLog
    let config = ConfigBuilder::new().build().unwrap();

    // Example log messages
    info!("This is an informational message");
    warn!("This is a warning!");
    error!("This is an error!");

    //Example with variables
    let test_var = "some data to test"
    debug!("This is debug variable value: {}", test_var)
}
```

## Configuration
You can customize LumeLog using the ConfigBuilder. Here’s what you can configure:

### Log Levels
- ERROR
- WARN
- INFO
- DEBUG
- TRACE

Example:
```rust
let config = ConfigBuilder::new()
    .log_level(Some(LogLevel::DEBUG)) // Log all messages down to DEBUG
    .build()
    .unwrap();
```

### File Logging
Enable file logging and specify a path and format:

```rust
use lumelog::{FileLoggerBuilder, FileLoggerFormat};

let config = ConfigBuilder::new()
    .file_logger_config(Some(
        FileLoggerBuilder::new()
        .path("./app.log".to_string())       // Specify log file path
        .log_format(FileLoggerFormat::TEXT)  // Set file format to JSON
    ))
    .build()
    .unwrap();
```

### Logging Macros
LumeLog provides easy-to-use macros for logging messages at various levels:

- info!: Logs informational messages.
- warn!: Logs warnings.
- error!: Logs errors.
- debug!: Logs debug messages.
- trace!: Logs trace-level messages.

Example:
```rust
info!("User {} has logged in", "Alice");
warn!("Memory usage is high: {}%", 90);
error!("Failed to save data: {}", "Disk full");
```

### Full config struct settings
```rust
ConfigBuilder{
    log_level: Some(lumelog::LogLevel::INFO),     // Log level for the logger.
    log_in_release: true,                         // Enables or disables logging in release mode.
    log_with_time: Some(true),                    // Enables or disables time-stamping in logs.
    std: true,                                    // Enables or disables standard output logging.
    file_logger_config: Some(FileLoggerBuilder{   // Configuration for file-based logging.,
        enabled: true,                            // Enables or disables file-based logging.
        path: Some("./my_app.log".to_string()),   // Path to the log file.
        log_format: Some(FileLoggerFormat::TEXT), // Format of the log file.
    })
}
```

## Build Mode Detection
LumeLog automatically detects build mode (debug or release) and adjusts logging behavior accordingly. You can override this behavior using the log_in_release configuration.

!NOTE: If `log_in_release` is enabled, trace and debug messages will not be logged.

## License
LumeLog is dual-licensed under the MIT or Apache 2.0 license. See LICENSE-MIT or LICENSE-APACHE for details.