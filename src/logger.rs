use std::fmt;

use colored::Colorize;

use crate::LogContext;
use crate::level::{LogLevel, INFO};


// An object that determines how
// logging should be displayed.
//
// By default, `Logger::default()` is used
// for logging text. For information on how
// to create custom logger, see `examples/
// custom_logger.rs`.
pub struct Logger {
    min_severity : u32,
    targets      : Vec<Box<dyn Fn(&LogContext)>>
}
impl Logger {
    pub fn new() -> Logger {
        return Logger {
            min_severity : 0,
            targets      : vec![]
        };
    }
}
impl Logger {
    // Sets the minimum severity index
    // required for a message to be logged.
    pub fn set_min_severity(mut self, min_severity : &LogLevel) -> Logger {
        self.min_severity = min_severity.get_severity();
        return self;
    }
    // Adds a function callback that
    // will be run when a message is logged.
    pub fn add_target<F : 'static>(mut self, target : F) -> Logger
        where F : Fn(&LogContext)
    {
        self.targets.push(Box::new(target));
        return self;
    }
}
impl Logger {
    fn create_context<'l>(&'l self, level : &'l LogLevel, module : String, position : (u32, u32), text : String) -> LogContext<'l> {
        return LogContext {
            logger   : self,
            time     : chrono::Utc::now(),
            module   : module,
            position : position,
            level    : level,
            text     : text
        };
    }
    // see `loggerithm::log!`
    pub fn log(&self, level : &LogLevel, module : String, position : (u32, u32), text : String) {
        if (level.get_severity() >= self.min_severity) {
            let context = self.create_context(level, module, position, text);
            for target in &self.targets {
                target(&context);
            }
        }
    }
}
impl Logger {
    // Create a logger object with the
    // default severity index and the
    // log target.
    pub fn default<'l>() -> Logger {
        return Logger::new()
            .set_min_severity(INFO::get())
            .add_target(|context| {
                println!(
                    " [ {} ] [ {} ] {}",
                    context.time_local()
                        .format("%y-%m-%d %H:%M:%S.%f").to_string()
                        .green().dimmed(),
                    context.level_name_fp(),
                    context.formatted(context.message())
                )
            })
    }
}
impl fmt::Display for Logger {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        return write!(f, "LOGGER");
    }
}