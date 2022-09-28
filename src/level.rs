use once_cell::sync::Lazy;

use crate::internal;



pub static TRACE   : &'static LogLevel = &LogLevel::new_const("TRACE"   , 5  );
pub static DEBUG   : &'static LogLevel = &LogLevel::new_const("DEBUG"   , 10 );
pub static INFO    : &'static LogLevel = &LogLevel::new_const("INFO"    , 20 );
pub static NOTICE  : &'static LogLevel = &LogLevel::new_const("NOTICE"  , 25 );
pub static SUCCESS : &'static LogLevel = &LogLevel::new_const("SUCCESS" , 25 );
pub static FAILURE : &'static LogLevel = &LogLevel::new_const("FAILURE" , 25 );
pub static WARN    : &'static LogLevel = &LogLevel::new_const("WARN"    , 30 );
pub static ERROR   : &'static LogLevel = &LogLevel::new_const("ERROR"   , 40 );
pub static FATAL   : &'static LogLevel = &LogLevel::new_const("FATAL"   , 50 );



pub fn set_global_min_severity<I : Into<u32>>(level : I) {
    unsafe {
        internal::MIN_SEVERITY = level.into();
    }
}



#[derive(PartialEq, Eq, Hash)]
pub struct LogLevel<'l> {
    name     : &'l str,
    severity : u32
}
impl LogLevel<'_> {
    pub const fn new_const(name : &str, severity : u32) -> LogLevel {
        let level = LogLevel {
            name,
            severity
        };
        let lazy = Lazy::<(), _>::new(|| {level.init();});
        return level;
    }
    pub fn new<'l, S : Into<&'l str>>(name_s : S, severity : u32) -> LogLevel<'l> {
        let name  = name_s.into();
        let level = LogLevel {
            name,
            severity
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
}
impl LogLevel<'_> {
    pub const fn get_name(&self) -> &str {
        return self.name;
    }
    pub const fn get_severity(&self) -> u32 {
        return self.severity;
    }
}
impl Into<u32> for LogLevel<'_> {
    fn into(self) -> u32 {
        return self.severity;
    }
}
unsafe impl Sync for LogLevel<'_> {}
