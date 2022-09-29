use loggerithm::{log, new_log_level};
use loggerithm::level::{LogLevel, INFO};

pub static CUSTOM: LogLevel = new_log_level!("MY_AMAZING_CUSTOM_LOGGING_LEVEL", 5);

fn main() {
    log!(&INFO, "This example shows how to create a custom logging level.");
    log!(&CUSTOM, "This used my custom level! :)");
    log!(&INFO, "The level name in first log is not padded as much as this message.");
    log!(&INFO, "That's because your log level was not known about yet.");
    log!(&INFO, "Check out `examples/custom_level_padding_fix.rs` for a fix.");
}
