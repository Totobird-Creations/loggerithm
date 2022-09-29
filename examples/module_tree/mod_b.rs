use loggerithm::log;
use loggerithm::level::FAILURE;

pub fn main() {
    log!(&FAILURE, "This message uses the default formatter as none is set for the `module_tree::mod_b` module or the `module_tree` module.");
}
