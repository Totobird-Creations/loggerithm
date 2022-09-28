#![allow(unused_parens)]
#![feature(once_cell)]


use std::collections::HashMap;

use colored::{ColoredString, Colorize};

pub mod internal;
pub mod level;
pub use level::LogLevel;



pub struct Logger<'l> {
    min_severity : Option<u32>,
    targets      : Vec<Box<dyn Fn(&LogContext)>>,
    formatters   : HashMap<&'l LogLevel<'l>, Box<dyn Fn(String) -> ColoredString>>
}
impl<'l> Logger<'l> {
    pub fn new() -> Logger<'l> {
        return Logger {
            min_severity : None,
            targets      : vec![],
            formatters   : HashMap::new()
        }
    }
}
impl<'l> Logger<'l> {
    pub fn set_min_severity<I : Into<u32>>(mut self, min_severity : I) -> Logger<'l> {
        self.min_severity = Some(min_severity.into());
        return self;
    }
    pub fn add_target<F : 'static + 'l>(mut self, target : F) -> Logger<'l>
        where F : Fn(&LogContext)
    {
        self.targets.push(Box::new(target));
        return self;
    }
    pub fn default_formatting(self) -> Logger<'l> {
        return self
            .add_formatter(level::TRACE   , |v| v.bright_black()                 )
            .add_formatter(level::DEBUG   , |v| v.white().dimmed()               )
            .add_formatter(level::INFO    , |v| v.cyan().dimmed()                )
            .add_formatter(level::NOTICE  , |v| v.bright_cyan()                  )
            .add_formatter(level::SUCCESS , |v| v.green()                        )
            .add_formatter(level::FAILURE , |v| v.red()                          )
            .add_formatter(level::WARN    , |v| v.yellow()                       )
            .add_formatter(level::ERROR   , |v| v.bright_red().bold()            )
            .add_formatter(level::FATAL   , |v| v.bright_white().bold().on_red() );
    }
    pub fn add_formatter<F : 'static + 'l>(mut self, level : &'l LogLevel, formatter : F) -> Logger<'l>
        where F : Fn(String) -> ColoredString
    {
        self.formatters.insert(level, Box::new(formatter));
        return self;
    }
}
impl Logger<'_> {
    fn get_min_severity(&self) -> u32 {
        return match (self.min_severity) {
            Some(level) => level,
            None        => unsafe {internal::MIN_SEVERITY}
        };
    }
    fn format<S : Into<String>>(&self, level : &LogLevel, text_s : S) -> ColoredString {
        let text = text_s.into();
        return if (self.formatters.contains_key(level)) {
            (self.formatters.get(level).unwrap())(text)
        } else {
            text.normal()
        };
    }
    pub fn log(&self, level : &LogLevel, text : String) {
        let context = LogContext {
            logger : self,
            level  : level
        };
        for target in &self.targets {
            target(&context);
        }
    }
}


pub struct LogContext<'l> {
    logger : &'l Logger<'l>,
    level  : &'l LogLevel<'l>
}
impl LogContext<'_> {
    // Level
    pub fn level(&self) -> &LogLevel {
        return self.level;
    }
    // Level name
    pub fn level_n(&self) -> String {
        return self.level.get_name().to_string();
    }
    // Padded level name
    pub fn level_np(&self) -> String {
        return format!("{:01$}", self.level_n(), unsafe {internal::MAX_LEVEL_LEN});
    }
    // Formatted level name
    pub fn level_nf(&self) -> ColoredString {
        return self.logger.format(self.level, self.level_n());
    }
    // Padded formatted level name
    pub fn level_npf(&self) -> ColoredString {
        return self.logger.format(self.level, self.level_np());
    }
}



#[macro_export]
macro_rules! register {
    ($logger:expr) => {
        unsafe {
            $crate::internal::LOGGERS.lock().unwrap().insert(module_path!().to_string(), $logger);
        }
    }
}

#[macro_export]
macro_rules! log {
    ($level:expr, $($fmt:tt)*) => {{
        let text = format!($($fmt)*);
        let name = $crate::internal::get_logger_name(module_path!().to_string());
        unsafe {$crate::internal::LOGGERS.lock().unwrap().get(&name).unwrap().log($level, text)};
    }}
}
