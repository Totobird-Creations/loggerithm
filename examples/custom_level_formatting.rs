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
