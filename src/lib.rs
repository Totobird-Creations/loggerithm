#![allow(unused_parens)]
#![feature(once_cell, const_fn_trait_bound)]


use std::{collections::HashMap, hash::Hash};

use colored::{ColoredString, Colorize};

pub mod internal;
pub mod level;



pub struct Logger<'l> {
    min_severity : Option<i32>,
    formatters   : HashMap<&'l LogLevel<'l>, Box<dyn Fn(String) -> ColoredString>>
}
impl<'l> Logger<'l> {
    pub fn new() -> Logger<'l> {
        return Logger {
            min_severity : None,
            formatters   : HashMap::new()
        }
    }
}
impl<'l> Logger<'l> {
    pub fn set_min_severity<I : Into<i32>>(mut self, min_severity : I) -> Logger<'l> {
        self.min_severity = Some(min_severity.into());
        return self;
    }
    pub fn add_formatter<F : 'static + 'l>(mut self, level : &'l LogLevel, formatter : F) -> Logger<'l>
        where F : Fn(String) -> ColoredString
    {
        self.formatters.insert(level, Box::new(formatter));
        return self;
    }
}
impl Logger<'_> {
    fn get_min_severity<I : Into<i32>>(&self) -> i32 {
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
        println!("{} {}", self.format(level, level.name), text);
    }
}


#[derive(PartialEq, Eq, Hash)]
pub struct LogLevel<'l> {
    name     : &'l str,
    severity : i32
}
impl LogLevel<'_> {
    pub const fn new_const(name : &str, severity : i32) -> LogLevel {
        return LogLevel {
            name,
            severity
        };
    }
    pub fn new<'l, S : Into<&'l str>>(name : S, severity : i32) -> LogLevel<'l> {
        return LogLevel {
            name     : name.into(),
            severity
        };
    }
}
impl Into<i32> for LogLevel<'_> {
    fn into(self) -> i32 {
        return self.severity;
    }
}
unsafe impl Sync for LogLevel<'_> {}



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
