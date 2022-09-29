#![allow(unused_parens)]
#![feature(once_cell)]


use std::collections::HashMap;

use colored::{ColoredString, Colorize};
use chrono;
use chrono::DateTime;

pub mod internal;
pub mod level;
use level::{LogLevel, LogLevelInside};



pub struct Logger<'l> {
    min_severity : u32,
    targets      : Vec<Box<dyn Fn(&LogContext)>>,
    formatters   : HashMap<&'l LogLevelInside, Box<dyn Fn(String) -> ColoredString>>
}
impl<'l> Logger<'l> {
    pub fn new() -> Logger<'l> {
        return Logger {
            min_severity : 0,
            targets      : vec![],
            formatters   : HashMap::new()
        }
            .default_formatting();
    }
}
impl<'l> Logger<'l> {
    pub fn set_min_severity(mut self, min_severity : &LogLevel) -> Logger<'l> {
        self.min_severity = min_severity.get_severity();
        return self;
    }
    pub fn add_target<F : 'static>(mut self, target : F) -> Logger<'l>
        where F : Fn(&LogContext)
    {
        self.targets.push(Box::new(target));
        return self;
    }
    pub fn default_formatting(self) -> Logger<'l> {
        return self
            .add_formatter(&level::TRACE   , |v| v.bright_black()                 )
            .add_formatter(&level::DEBUG   , |v| v.white().dimmed()               )
            .add_formatter(&level::INFO    , |v| v.cyan().dimmed()                )
            .add_formatter(&level::NOTICE  , |v| v.bright_cyan()                  )
            .add_formatter(&level::SUCCESS , |v| v.green()                        )
            .add_formatter(&level::FAILURE , |v| v.red()                          )
            .add_formatter(&level::WARN    , |v| v.yellow()                       )
            .add_formatter(&level::ERROR   , |v| v.bright_red().bold()            )
            .add_formatter(&level::FATAL   , |v| v.bright_white().bold().on_red() );
    }
    pub fn add_formatter<F : 'static>(mut self, level : &'l LogLevel, formatter : F) -> Logger<'l>
        where F : Fn(String) -> ColoredString
    {
        self.formatters.insert(&**level, Box::new(formatter));
        return self;
    }
}
impl<'l> Logger<'l> {
    fn format(&self, level : &LogLevel, text : String) -> String {
        return if (self.formatters.contains_key(&**level)) {
            (self.formatters.get(&**level).unwrap())(text).to_string()
        } else {text};
    }
    fn create_context(&'l self, level : &'l LogLevel, module : String, position : (u32, u32), text : String) -> LogContext<'l> {
        return LogContext::<'l> {
            logger   : self,
            time     : chrono::Utc::now(),
            module   : module,
            position : position,
            level    : level,
            text     : text
        };
    }
    pub fn log(&self, level : &LogLevel, module : String, position : (u32, u32), text : String) {
        if (level.get_severity() >= self.min_severity) {
            let context = self.create_context(level, module, position, text);
            for target in &self.targets {
                target(&context);
            }
        }
    }
}
impl Logger<'_> {
    pub fn default<'l>() -> Logger<'l> {
        return Logger::new()
            .set_min_severity(&level::INFO)
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
            .default_formatting();
    }
}


pub struct LogContext<'l> {
    logger   : &'l Logger<'l>,
    time     : DateTime<chrono::Utc>,
    module   : String,
    position : (u32, u32),
    level    : &'l LogLevel,
    text     : String
}
impl LogContext<'_> {
    pub fn time_utc(&self) -> DateTime<chrono::Utc> {
        return self.time;
    }
    pub fn time_local(&self) -> DateTime<chrono::Local> {
        return DateTime::from(self.time);
    }
    pub fn module(&self) -> String {
        return String::from(&self.module);
    }
    pub fn line(&self) -> u32 {
        return self.position.0;
    }
    pub fn column(&self) -> u32 {
        return self.position.1;
    }
    pub fn level(&self) -> &LogLevel {
        return self.level;
    }
    // Level name with no modification.
    pub fn level_name(&self) -> String {
        return self.level.get_name().to_string().normal().to_string();
    }
    // Level name: Padded.
    pub fn level_name_p(&self) -> String {
        return self.padded_level_name(self.level_name(), self.level_name().len());
    }
    // Level name: Formatted.
    pub fn level_name_f(&self) -> String {
        return self.formatted(self.level_name());
    }
    // Level name: Padded then formatted.
    pub fn level_name_pf(&self) -> String {
        return self.formatted(self.level_name_p());
    }
    // Level name: Formatted then padded.
    pub fn level_name_fp(&self) -> String {
        return self.padded_level_name(self.level_name_f(), self.level_name().len());
    }
    pub fn message(&self) -> String {
        return String::from(&self.text);
    }
}
impl LogContext<'_> {
    pub fn formatted(&self, text : String) -> String {
        return self.logger.format(self.level, text);
    }
    fn padded_level_name(&self, text : String, len : usize) -> String {
        let max_level_len = unsafe {internal::MAX_LEVEL_LEN};
        return format!("{:01$}", text, max_level_len + (text.len() - len));
    }
}



#[macro_export]
macro_rules! set_logger {
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
        unsafe {$crate::internal::LOGGERS.lock().unwrap()}
            .get(&name).unwrap()
            .log(
                $level,
                module_path!().to_string(), (line!(), column!()),
                text
            );
    }}
}
