use loggerithm::{logger, log, log_level};
use loggerithm::level::INFO;

logger!(super);
log_level!(MY_AMAZING_CUSTOM_LOGGING_LEVEL, LogLevel::new(30));

fn main() {
    log!(INFO, "This example shows how to create a custom logging level.");
    log!(MY_AMAZING_CUSTOM_LOGGING_LEVEL, "This used my custom level! :)");
    log!(INFO, "Padding also works.");
}
