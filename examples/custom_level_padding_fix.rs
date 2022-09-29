use loggerithm::{log, new_log_level};
use loggerithm::level::{LogLevel, INFO};

pub static CUSTOM: LogLevel = new_log_level!("MY_AMAZING_CUSTOM_LOGGING_LEVEL", 5);

fn main() {
    CUSTOM.void();
    log!(&INFO, "This example shows how to create a custom logging level without the padding problems.");
    log!(&CUSTOM, "This used my custom level! :)");
    log!(&INFO, "The `LogLevel.void()` method does nothing, but it does expose its existence.");
}
