use loggerithm::{logger, log};
use loggerithm::logger::Logger;
use loggerithm::level::{INFO, WARN};

logger!(Logger::new()
    .set_min_severity(WARN::SEVERITY)
    .add_target(|context| {
        println!("{} | {} | {}", context.time_local(), context.level_name_fp(), context.message())
    })
);

fn main() {
    log!(WARN, "This is logged.");
    log!(INFO, "This is not logged because it is below the minimum severity.");
}
