use loggerithm::{logger, log};
use loggerithm::level::INFO;

mod module_a {
    pub fn main() {}
}

mod module_b {
    pub fn main() {}
}


logger!(super);

fn main() {
    log!(INFO, "This message uses the default formatter as none is set for the `module_tree` module.");
    module_a::main();
    module_b::main();
}
