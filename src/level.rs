use crate::internal;
use crate::LogLevel;



pub static TRACE   : &'static LogLevel = &LogLevel::new_const("TRACE"   , 5  );
pub static DEBUG   : &'static LogLevel = &LogLevel::new_const("DEBUG"   , 10 );
pub static INFO    : &'static LogLevel = &LogLevel::new_const("INFO"    , 20 );
pub static NOTICE  : &'static LogLevel = &LogLevel::new_const("NOTICE"  , 25 );
pub static SUCCESS : &'static LogLevel = &LogLevel::new_const("SUCCESS" , 25 );
pub static FAILURE : &'static LogLevel = &LogLevel::new_const("FAILURE" , 25 );
pub static WARN    : &'static LogLevel = &LogLevel::new_const("WARN"    , 30 );
pub static ERROR   : &'static LogLevel = &LogLevel::new_const("ERROR"   , 40 );
pub static FATAL   : &'static LogLevel = &LogLevel::new_const("FATAL"   , 50 );



pub fn set_global_min_severity<I : Into<i32>>(level : I) {
    unsafe {
        internal::MIN_SEVERITY = level.into();
    }
}
