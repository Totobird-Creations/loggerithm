# Loggerithm
### Logging Library

#### Installation
```toml
# Cargo.toml

[dependencies]
loggerithm  = "1"
static_init = "1.0.3"
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
use loggerithm::level::INFO;

logger!(super);

log_level!(MY_AMAZING_CUSTOM_LOGGING_LEVEL, LogLevel::new(30)
    .formatted(|v| v.magenta().on_white().reverse())
);

fn main() {
    log!(INFO, "This example shows how to create a custom logging level and add formatting to it.");
    log!(MY_AMAZING_CUSTOM_LOGGING_LEVEL, "This used my custom level, now with formatting!");
}
```
See `examples/custom_level.rs` and `examples/custom_level_formatting.rs`.

#### Custom Logger
```rust
use loggerithm::{logger, log};
use loggerithm::level::{INFO, WARN};

logger!(Logger::new()
    .set_min_severity(WARN::severity())
    .add_target(|context| {
        println!("{} | {} | {}", context.time_local(), context.level_name_fp(), context.message())
    })
);

fn main() {
    log!(WARN, "This is logged.");
    log!(INFO, "This is not logged because it is below the minimum severity.");
}
```
See `examples/custom_logger.rs`

#### Modules
See `examples/module_tree.rs` for information on how loggers work across modules.
