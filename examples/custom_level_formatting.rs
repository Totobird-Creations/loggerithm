use colored::Colorize;

use loggerithm::{log, new_log_level, set_logger, Logger};
use loggerithm::level::{LogLevel, INFO};

pub static CUSTOM: LogLevel = new_log_level!("MY_AMAZING_CUSTOM_LOGGING_LEVEL", 30);

fn main() {
    set_logger!(Logger::default()
        .add_formatter(&CUSTOM, |v| v.bright_magenta().on_white().reverse())
    );
    log!(&INFO, "This example shows how to create a custom logging level and add formatting to it.");
    log!(&CUSTOM, "This used my custom level, now with formatting!");
}
