use loggerithm::{Logger, log, set_logger};
use loggerithm::level::SUCCESS;

mod mod_aa;

pub fn main() {
    set_logger!(Logger::new()
        .add_target(|context| {
            println!("{} | {} | {}", context.time_local(), context.level_name_fp(), context.message())
        })
    );
    log!(&SUCCESS, "This message uses a custom formatter as one is set for the `module_tree::mod_a` module.");
    mod_aa::main();
}
