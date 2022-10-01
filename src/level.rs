//! Everything related to logging levels.
//! 
//! # Examples
// 
//! ```
//! use loggerithm::level::{TRACE, DEBUG, INFO, NOTICE, SUCCESS, FAILURE, WARN, ERROR, FATAL};
//! log!(TRACE, "Test message");
//! log!(DEBUG, "Test message");
//! log!(INFO, "Test message");
//! log!(NOTICE, "Test message");
//! log!(SUCCESS, "Test message");
//! log!(FAILURE, "Test message");
//! log!(WARN, "Test message");
//! log!(ERROR, "Test message");
//! log!(FATAL, "Test message");
//! ```



use colored::{ColoredString, Colorize};

use crate::internal;



/// An object containing information
/// about how to print a log message.
pub struct LogLevel {
    name      : String,
    severity  : u32,
    formatter : Box<dyn Fn(String) -> ColoredString>,
    init      : bool
}
/// Initialisation.
impl LogLevel {
    /// Create a new log level.
    /// 
    /// # Arguments
    /// 
    /// * `severity` - The severity of the log level. When
    ///                logged, must be equal to or higher
    ///                than the minimum severity level of
    ///                the active logger in order to be
    ///                seen.
    /// 
    /// # Returns
    /// 
    /// The log level that was created.
    /// 
    /// # Examples
    /// 
    /// ```
    /// log_level!(ALERT, LogLevel::new(30));
    /// ```
    pub fn new(severity : u32) -> LogLevel {
        let level = LogLevel {
            name      : String::new(),
            severity,
            formatter : Box::new(|v| v.normal()),
            init      : false
        };
        return level;
    }
    /// Register the log level and attach
    /// a name.
    /// 
    /// This function will panic if called
    /// more than once.
    /// 
    /// This function is called automatically
    /// with the `log_level!` macro.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the log level.
    /// 
    /// # Returns
    /// 
    /// `self`
    pub fn init(mut self, name : &str) -> LogLevel {
        if (self.init) {
            panic!("`init` already called.");
        }
        self.init = true;
        self.name = String::from(name);
        if (unsafe {internal::MAX_LEVEL_NAME_LEN} < self.name.len()) {
            unsafe {
                internal::MAX_LEVEL_NAME_LEN = self.name.len();
            }
        }
        return self;
    }
    /// Set the formatting of the log level.
    /// 
    /// # Arguments
    /// 
    /// * `formatter` - The function that formats the level.
    ///                 It takes a `String` as an argument
    ///                 and should return a
    ///                 `colored::ColoredString`.
    /// 
    /// # Generics
    /// 
    /// * `F` - The formatting function.
    /// 
    /// # Returns
    /// 
    /// `self`
    pub fn formatted<F : 'static>(mut self, formatter : F) -> LogLevel
        where F : Fn(String) -> ColoredString
    {
        self.formatter = Box::new(formatter);
        return self;
    }
}
/// Data access functions.
impl LogLevel {
    /// Get the name of the log level.
    /// 
    /// # Returns
    /// 
    /// The name of the log level.
    pub fn get_name(&self) -> String {
        return String::from(&self.name);
    }
    /// Get the severity of the log level.
    /// 
    /// # Returns
    /// 
    /// The severity of the log level.
    pub fn get_severity(&self) -> u32 {
        return self.severity;
    }
    /// Calls the formatter on a string.
    /// 
    /// # Arguments
    /// 
    /// * `text` - The text to format.
    /// 
    /// # Returns
    /// 
    /// The formatted text.
    pub fn format(&self, text : String) -> String {
        return (self.formatter)(text).to_string();
    }
}
/// Thread safety.
unsafe impl Sync for LogLevel {}



/// Registers a new log level.
/// 
/// # Arguments
/// 
/// * `name`   - The identifier to store the level at.
///              This will be the name of the level
///              and the name used to access it.
/// * `logger` - The level to register and store.
/// 
/// # Returns
/// 
/// A module containing everything
/// needed for the level.
/// 
/// # Examples
/// 
/// ```
/// log_level!(ALERT, LogLevel::new(30));
/// fn main() {
///     log!(ALERT, "This is an alert!");
/// }
/// ```
#[macro_export]
macro_rules! log_level {
    ($name:ident, $level:expr) => {
        /// A logging level.
        #[allow(non_snake_case)]
        pub mod $name {
            use $crate::level::LogLevel;
            #[allow(unused_imports)]
            use colored::{ColoredString, Colorize};
            /// The log level handler object.
            #[$crate::ext::dynamic]
            static OBJECT : LogLevel = ($level).init(stringify!($name));
            /// Get a reference to the log level handler.
            pub fn level<'l>() -> &'l LogLevel {
                return &OBJECT;
            }
            /// Get the severity of the log level.
            pub fn severity() -> u32 {
                return OBJECT.get_severity();
            }
        }
    };
}
pub(crate) use log_level;



log_level!(TRACE, LogLevel::new(5)
    .formatted(|v| v.bright_black())
);
log_level!(DEBUG, LogLevel::new(10)
    .formatted(|v| v.white().dimmed())
);
log_level!(INFO, LogLevel::new(20)
    .formatted(|v| v.cyan().dimmed())
);
log_level!(NOTICE, LogLevel::new(25)
    .formatted(|v| v.bright_cyan())
);
log_level!(SUCCESS, LogLevel::new(25)
    .formatted(|v| v.green())
);
log_level!(WARN, LogLevel::new(30)
    .formatted(|v| v.yellow())
);
log_level!(FAILURE, LogLevel::new(35)
    .formatted(|v| v.red())
);
log_level!(ERROR, LogLevel::new(40)
    .formatted(|v| v.bright_red().bold())
);
log_level!(FATAL, LogLevel::new(50)
    .formatted(|v| v.bright_white().bold().on_red())
);
