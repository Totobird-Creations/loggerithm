use loggerithm::log;
use loggerithm::level::WARN;

pub fn main() {
    log!(&WARN, "This message uses the logger for the `module_tree::mod_a` module as there is none set for `module_tree::mod_a::mod_aa`.");
}
