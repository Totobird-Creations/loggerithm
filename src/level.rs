use static_init::dynamic;
use colored::{ColoredString, Colorize};

use crate::internal;



#[dynamic]
pub static TRACE : LogLevel = LogLevel::new("TRACE", 5)
    .formatted(|v| v.bright_black());
#[dynamic]
pub static DEBUG : LogLevel = LogLevel::new("DEBUG", 10)
    .formatted(|v| v.white().dimmed());
#[dynamic]
pub static INFO : LogLevel = LogLevel::new("INFO", 20)
    .formatted(|v| v.cyan().dimmed());
#[dynamic]
pub static NOTICE : LogLevel = LogLevel::new("NOTICE", 25)
    .formatted(|v| v.bright_cyan());
#[dynamic]
pub static SUCCESS : LogLevel = LogLevel::new("SUCCESS", 25)
    .formatted(|v| v.green());
#[dynamic]
pub static WARN : LogLevel = LogLevel::new("WARN", 30)
    .formatted(|v| v.yellow());
#[dynamic]
pub static FAILURE : LogLevel = LogLevel::new("FAILURE", 35)
    .formatted(|v| v.red());
#[dynamic]
pub static ERROR : LogLevel = LogLevel::new("ERROR", 40)
    .formatted(|v| v.bright_red().bold());
#[dynamic]
pub static FATAL : LogLevel = LogLevel::new("FATAL", 50)
    .formatted(|v| v.bright_white().bold().on_red());



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
        if (unsafe {internal::MAX_LEVEL_LEN} < self.name.len()) {
            unsafe {
                internal::MAX_LEVEL_LEN = self.name.len();
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
unsafe impl Sync for LogLevel {}
