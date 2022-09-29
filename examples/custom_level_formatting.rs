use colored::Colorize;

use loggerithm::log;
use loggerithm::level::{LogLevel, INFO};
use loggerithm::ext::dynamic;

#[dynamic]
pub static CUSTOM: LogLevel = LogLevel::new("MY_AMAZING_CUSTOM_LOGGING_LEVEL", 30)
    .formatted(|v| v.magenta().on_white().reverse());

fn main() {
    log!(&INFO, "This example shows how to create a custom logging level and add formatting to it.");
    log!(&CUSTOM, "This used my custom level, now with formatting!");
}
