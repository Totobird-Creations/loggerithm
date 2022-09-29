use once_cell::sync::Lazy;

use crate::internal;



pub type LogLevelInside = LogLevelHandler<'static>;
pub type LogLevel       = Lazy<LogLevelInside>;

pub static TRACE   : LogLevel = new_log_level!("TRACE"   , 5  );
pub static DEBUG   : LogLevel = new_log_level!("DEBUG"   , 10 );
pub static INFO    : LogLevel = new_log_level!("INFO"    , 20 );
pub static NOTICE  : LogLevel = new_log_level!("NOTICE"  , 25 );
pub static SUCCESS : LogLevel = new_log_level!("SUCCESS" , 25 );
pub static WARN    : LogLevel = new_log_level!("WARN"    , 30 );
pub static FAILURE : LogLevel = new_log_level!("FAILURE" , 35 );
pub static ERROR   : LogLevel = new_log_level!("ERROR"   , 40 );
pub static FATAL   : LogLevel = new_log_level!("FATAL"   , 50 );



#[macro_export]
macro_rules! new_log_level {
    ($name:tt, $severity:tt) => {{
        use once_cell::sync::Lazy;
        Lazy::new(|| {
            if ($name.len() > unsafe {$crate::internal::MAX_LEVEL_LEN}) {
                unsafe {
                    $crate::internal::MAX_LEVEL_LEN = $name.len();
                }
            }
            return $crate::level::LogLevelHandler::new($name, $severity);
        })
    }}
}
pub(crate) use new_log_level;



#[derive(Eq, PartialEq, Hash)]
pub struct LogLevelHandler<'l> {
    name     : &'l str,
    severity : u32
}
impl LogLevelHandler<'_> {
    pub fn new<'l, S : Into<&'l str>>(name_s : S, severity : u32) -> LogLevelHandler<'l> {
        let name  = name_s.into();
        let level = LogLevelHandler {
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
    pub fn void(&self) {}
}
impl LogLevelHandler<'_> {
    pub const fn get_name(&self) -> &str {
        return self.name;
    }
    pub const fn get_severity(&self) -> u32 {
        return self.severity;
    }
}
unsafe impl Sync for LogLevelHandler<'_> {}
