use loggerithm::{Logger, set_logger, log};
use loggerithm::level::{INFO, WARN};

fn main() {
    set_logger!(Logger::new()
        .set_min_severity(&WARN)
        .add_target(|context| {
            println!("{} | {} | {}", context.time_local(), context.level_name_fp(), context.message())
        })
    );
    log!(&WARN, "This is logged.");
    log!(&INFO, "This is not logged because it is below the minimum severity.");
}
