# Loggerithm
### Logging Library

#### Installation
```toml
# Cargo.toml

[dependencies]
loggerithm = "1"
```

#### Preparation
At the top of each module that logs text, you must add one of the following:
```rust
use loggerithm::logger;
// This will tell the module to use the logger from the parent module.
// If there is no parent module, it will use the default logger.
logger!(super);
```
```rust
use loggerithm::logger;
logger!(Logger::new()
    // All of the settings here. See `src/logger.rs`.
);
```

#### Basic Logging
Loggerithm provides the standard logging levels, plus `TRACE` `NOTICE` `SUCCESS` and `FAILURE`.
The `log!` macro is similar to the `format!` macro, but takes a logging level as the first argument.
```rust
use loggerithm::level::{TRACE, DEBUG, INFO, NOTICE, SUCCESS, FAILURE, WARN, ERROR, FATAL};
use loggerithm::{log, logger};

logger!(super);

fn main() {
    let x = 25;
    log!(SUCCESS, "The value of x is equal to {}.", x);
}
```
See `examples/basic_logging.rs`.

#### Custom Logging Levels
```rust
use loggerithm::{logger, log_level, log};

logger!(super);

log_level!(MY_LEVEL, LogLevel::new("AMAZING_LEVEL_NAME", 20));
//                                  |                    ^ The severity of the level. To be seen, this must be above the minimum set for the active logger.
//                                  ^ The name of the logger. This is what's displayed in the console.

fn main() {
    log!(MY_LEVEL, "Hello, this is my spaghetti code.");
}
```
See `examples/custom_level.rs` and `examples/custom_level_formatting.rs`.

#### Custom Logger
```rust

```
See `examples/custom_logger.rs`
