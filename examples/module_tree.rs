mod module_test {
    mod module_subtest {
        use loggerithm::{logger, log};
        use loggerithm::level::INFO;

        logger!(super);

        pub fn main() {
            log!(INFO, "This message uses a custom logger set for the `module_tree::module_test::module_subtest` module.");
        }
    }

    use loggerithm::{logger, log};
    use loggerithm::level::INFO;

    logger!(Logger::new()
        .add_target(|context| {
            println!(
                " | {} | | {} | | {} | {} {}",
                context.time_local().naive_local(),
                context.module_pf(),
                context.level_name_p(),
                ">".bright_magenta(),
                context.message()
            );
        })
    );

    pub fn main() {
        log!(INFO, "This message uses a custom logger set for the `module_tree::module_test` module.");
        module_subtest::main();
    }
}

mod module_other {
    mod module_other_2 {
        use loggerithm::{logger, log};
        use loggerithm::level::INFO;

        logger!(super);

        pub fn main() {
            log!(INFO, "This message uses the logger from `module_tree` as none is set for the `module_tree::module_other::module_other_2`, `module_tree::module_other`");
        }
    }

    use loggerithm::log;
    use loggerithm::level::INFO;

    pub fn main() {
        // None is set for this module so `module_other_2` will get from the parent of this module.
        module_other_2::main();
        log!(INFO, "This will fail because no logger is set for `module_tree::module_other`.");
    }
}


use loggerithm::{logger, log};
use loggerithm::level::INFO;

logger!(super);

fn main() {
    log!(INFO, "This message uses the default logger as none is set for the `module_tree` module.");
    module_test::main();
    module_other::main();
}
