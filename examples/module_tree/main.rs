use loggerithm::log;
use loggerithm::level::INFO;

mod mod_a;
mod mod_b;

fn main() {
    log!(&INFO, "This message uses the default formatter as none is set for the `module_tree` module.");
    mod_a::main();
    mod_b::main();
}
