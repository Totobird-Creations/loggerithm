//! Everything related to loggers.


use colored::Colorize;

use crate::LogContext;
use crate::level;


/// An object that determines how logs
/// should be given to the user.
///
/// By default, `Logger::default()` is used
/// for logging text. For information on how
/// to create custom logger, see `examples/
/// custom_logger.rs`.
pub struct Logger {
    min_severity : u32,
    targets      : Vec<Box<dyn Fn(&LogContext)>>
}
/// Initialisation.
impl Logger {
    /// Create a new logger object.
    pub fn new() -> Logger {
        return Logger {
            min_severity : 0,
            targets      : vec![]
        };
    }
    /// Create a logger object with the
    /// default severity index and the
    /// log target.
    pub fn default<'l>() -> Logger {
        return Logger::new()
            .set_min_severity(level::INFO::SEVERITY)
            .add_target(|context| {
                println!(
                    " [ {:0>9} ] [ {} ] [ {} ] {}",
                    context.time_local()
                        .format("%Y-%m-%d %H:%M:%S.%f").to_string()
                        .bright_green().dimmed(),
                    context.module_p()
                        .green().dimmed(),
                    context.level_name_fp(),
                    context.formatted(context.message())
                )
            })
    }
    /// Sets the minimum severity index
    /// required for a message to be logged.
    pub fn set_min_severity<F>(mut self, min_severity : F) -> Logger
        where F : Fn() -> u32
    {
        self.min_severity = min_severity();
        return self;
    }
    /// Adds a function callback that
    /// will be run when a message is logged.
    pub fn add_target<F : 'static>(mut self, target : F) -> Logger
        where F : Fn(&LogContext)
    {
        self.targets.push(Box::new(target));
        return self;
    }
}
impl Logger {
    /// Create a LogContext object with
    /// all of the needed values.
    /// 
    /// # Arguments
    /// 
    /// * `level`    - The level that the message is
    ///                being logged at.
    /// * `module`   - The module that invoked the
    ///                `log!` macro.
    /// * `position` - A `u32` tuple containing the
    ///                line number, then the column
    ///                number.
    /// * `text`     - The message to log.
    /// 
    /// # Returns
    /// 
    /// The generated LogContext.
    pub fn create_context<'l>(&'l self, level : &'l level::LogLevel, module : String, position : (u32, u32), text : String) -> LogContext<'l> {
        return LogContext {
            logger   : self,
            time     : chrono::Utc::now(),
            module   : module,
            position : position,
            level    : level,
            text     : text
        };
    }
    /// Invoke all of the log targets.
    /// 
    /// # Arguments
    /// 
    /// * `context` : A `LogContext` containing information
    ///               about the log.
    pub fn log(&self, context : LogContext) {
        if (context.level().get_severity() >= self.min_severity) {
            for target in &self.targets {
                target(&context);
            }
        }
    }
}
