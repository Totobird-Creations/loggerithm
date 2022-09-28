#![allow(unused_parens)]
#![feature(once_cell, const_fn_trait_bound)]

use colored::ColoredString;

pub mod internal;



pub static TRACE    : LogLevel = LogLevel::new_const("TRACE");
pub static DEBUG    : LogLevel = LogLevel::new_const("DEBUG");
pub static INFO     : LogLevel = LogLevel::new_const("INFO");
pub static SUCCESS  : LogLevel = LogLevel::new_const("SUCCESS");
pub static WARN     : LogLevel = LogLevel::new_const("WARN");
pub static ERROR    : LogLevel = LogLevel::new_const("ERROR");
pub static CRITICAL : LogLevel = LogLevel::new_const("CRITICAL");

pub trait Logger {
}

pub struct LogLevel<'l> {
    name      : &'l str,
    formatter : Option<Box<dyn Fn(String) -> ColoredString>>
}
impl LogLevel<'_> {
    pub const fn new_const(name : &str) -> LogLevel {
        return LogLevel {
            name      : name,
            formatter : None
        };
    }
    pub fn new<'l, S : Into<&'l str>, F : 'static>(name : S, formatter : F) -> LogLevel<'l>
        where F : Fn(String) -> ColoredString
    {
        return LogLevel {
            name      : name.into(),
            formatter : Some(Box::new(formatter))
        };
    }
    pub fn set_formatter<F : 'static>(&mut self, formatter : F)
        where F : Fn(String) -> ColoredString
    {
        self.formatter = Some(Box::new(formatter));
    }
}
unsafe impl Sync for LogLevel<'_> {}


#[macro_export]
macro_rules! init {
    ($logger:expr) => {unsafe {
        println!("{} {}", module_path!(), $crate::internal::get_path());
        $crate::internal::LOGGERS.lock().unwrap().insert(module_path!().to_string(), Box::new($logger));
    }}
}
