use loggerithm::log;
use loggerithm::ext::dynamic;
use loggerithm::level::{LogLevel, INFO};

#[dynamic]
pub static CUSTOM: LogLevel = LogLevel::new("MY_AMAZING_CUSTOM_LOGGING_LEVEL", 30);

fn main() {
    log!(&INFO, "This example shows how to create a custom logging level.");
    log!(&CUSTOM, "This used my custom level! :)");
    log!(&INFO, "Padding also works.");
}
