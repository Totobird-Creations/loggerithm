use std::fmt;

use colored::{ColoredString, Colorize};

use crate::internal;



pub struct LogLevel {
    name      : String,
    severity  : u32,
    formatter : Box<dyn Fn(String) -> ColoredString>
}
impl LogLevel {
    pub fn new<S : Into<String>>(name_s : S, severity : u32) -> LogLevel {
        let name  = name_s.into();
        let level = LogLevel {
            name,
            severity,
            formatter : Box::new(|v| v.normal())
        };
        level.init();
        return level;
    }
    fn init(&self) {
        if (unsafe {internal::MAX_LEVEL_NAME_LEN} < self.name.len()) {
            unsafe {
                internal::MAX_LEVEL_NAME_LEN = self.name.len();
            }
        }
    }
    pub fn void(&self) {}
    pub fn formatted<F : 'static>(mut self, formatter : F) -> LogLevel
        where F : Fn(String) -> ColoredString
    {
        self.formatter = Box::new(formatter);
        return self;
    }
}
impl LogLevel {
    pub fn get_name(&self) -> String {
        return String::from(&self.name);
    }
    pub fn get_severity(&self) -> u32 {
        return self.severity;
    }
    pub fn format(&self, text : String) -> String {
        return (self.formatter)(text).to_string();
    }
}
impl fmt::Display for LogLevel {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        return write!(f, "{}.{}", self.get_name(), self.get_severity());
    }
}
unsafe impl Sync for LogLevel {}



#[macro_export]
macro_rules! log_level {
    ($name:ident, $logger:expr) => {
        #[allow(non_snake_case)]
        pub mod $name {
            use static_init::dynamic;
            use $crate::level::LogLevel;
            #[allow(unused_imports)]
            use colored::{ColoredString, Colorize};
            #[dynamic]
            static OBJECT : LogLevel = $logger;
            pub fn severity() -> u32 {
                return OBJECT.severity;
            }
        }
    };
}
pub(crate) use log_level;



log_level!(TRACE, LogLevel::new("TRACE", 5)
    .formatted(|v| v.bright_black())
);
log_level!(DEBUG, LogLevel::new("DEBUG", 10)
    .formatted(|v| v.white().dimmed())
);
log_level!(INFO, LogLevel::new("INFO", 20)
    .formatted(|v| v.cyan().dimmed())
);
log_level!(NOTICE, LogLevel::new("NOTICE", 25)
    .formatted(|v| v.bright_cyan())
);
log_level!(SUCCESS, LogLevel::new("SUCCESS", 25)
    .formatted(|v| v.green())
);
log_level!(WARN, LogLevel::new("WARN", 30)
    .formatted(|v| v.yellow())
);
log_level!(FAILURE, LogLevel::new("FAILURE", 35)
    .formatted(|v| v.red())
);
log_level!(ERROR, LogLevel::new("ERROR", 40)
    .formatted(|v| v.bright_red().bold())
);
log_level!(FATAL, LogLevel::new("FATAL", 50)
    .formatted(|v| v.bright_white().bold().on_red())
);
