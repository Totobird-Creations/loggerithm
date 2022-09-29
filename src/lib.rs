#![allow(unused_parens)]

use std::fmt;

use colored::Colorize;
use chrono;
use chrono::DateTime;

pub mod internal;
pub mod level;
pub mod logger;
use logger::Logger;
use level::LogLevel;



// Passed as an argument when the log
// target callback is called.
pub struct LogContext<'l> {
    #[allow(dead_code)]
    logger   : &'l Logger,
    time     : DateTime<chrono::Utc>,
    module   : String,
    position : (u32, u32),
    level    : &'l LogLevel,
    text     : String
}
impl LogContext<'_> {
    // Get the current UTC time.
    pub fn time_utc(&self) -> DateTime<chrono::Utc> {
        return self.time;
    }
    // Get the current local time.
    pub fn time_local(&self) -> DateTime<chrono::Local> {
        return DateTime::from(self.time);
    }
    // Get the module that the log command was run in.
    pub fn module(&self) -> String {
        return String::from(&self.module);
    }
    // Get the level name that has been padded.
    pub fn module_p(&self) -> String {
        return self.padded_module(self.module(), self.module().len());
    }
    // Get the level name that has been formatted.
    pub fn module_f(&self) -> String {
        return self.formatted(self.module());
    }
    // Get the level name that has been padded then formatted.
    pub fn module_pf(&self) -> String {
        return self.formatted(self.module_p());
    }
    // Get the level name that has been formatted then padded.
    pub fn module_fp(&self) -> String {
        return self.padded_module(self.module_f(), self.module().len());
    }
    // Get the line number that the log command was run at.
    pub fn line(&self) -> u32 {
        return self.position.0;
    }
    // Get the column number that the log command was run at.
    pub fn column(&self) -> u32 {
        return self.position.1;
    }
    // Get the log level passed into the log command.
    pub fn level(&self) -> &LogLevel {
        return self.level;
    }
    // Get the level name.
    pub fn level_name(&self) -> String {
        return self.level.get_name().to_string().normal().to_string();
    }
    // Get the level name that has been padded.
    pub fn level_name_p(&self) -> String {
        return self.padded_level_name(self.level_name(), self.level_name().len());
    }
    // Get the level name that has been formatted.
    pub fn level_name_f(&self) -> String {
        return self.formatted(self.level_name());
    }
    // Get the level name that has been padded then formatted.
    pub fn level_name_pf(&self) -> String {
        return self.formatted(self.level_name_p());
    }
    // Get the level name that has been formatted then padded.
    pub fn level_name_fp(&self) -> String {
        return self.padded_level_name(self.level_name_f(), self.level_name().len());
    }
    // Get the message passed into the log command.
    pub fn message(&self) -> String {
        return String::from(&self.text);
    }
}
impl LogContext<'_> {
    // Format text based on the logging level passed into the log command.
    pub fn formatted(&self, text : String) -> String {
        return self.level.format(text);
    }
    fn padded_level_name(&self, text : String, len : usize) -> String {
        return self.padded(text, len, unsafe {internal::MAX_LEVEL_NAME_LEN});
    }
    fn padded_module(&self, text : String, len : usize) -> String {
        return self.padded(text, len, unsafe {internal::MAX_MODULE_LEN});
    }
    fn padded(&self, text : String, len : usize, target_len : usize) -> String {
        return format!("{:01$}", text, target_len + (text.len() - len));
    }
}
impl fmt::Display for LogContext<'_> {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        return write!(f, "LOGCONTEXT");
    }
}




// Call the target callbacks of the logger in the current module.
#[macro_export]
macro_rules! log {
    ($level:ident, $($fmt:tt)*) => {{
        let module = module_path!().to_string();
        let id_opt = $crate::internal::run_module_logger(module, true, |logger| {
            logger.log(
                $level::level(),
                module_path!().to_string(), (line!(), column!()),
                format!($($fmt)*)
            )
        });
    }};
}
