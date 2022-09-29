use std::collections::HashMap;

use static_init::dynamic;

use crate::Logger;


pub enum LoggerLocation {
    Super,
    Here(Logger)
}
unsafe impl Send for LoggerLocation {}

pub struct LoggerRef {
    pub module : String
}


#[dynamic]
pub static mut LOGGERS : HashMap<String, LoggerLocation> = {
    let mut map = HashMap::new();
    map.insert(String::new(), LoggerLocation::Here(Logger::default()));
    map
};

pub static mut MAX_LEVEL_LEN : usize = 0;



pub fn run_module_logger<F>(module : String, callback : F)
    where F : Fn(&Logger)
{
    #[allow(unused_unsafe)]
    match (unsafe {LOGGERS.write()}.get(&module)) {
        Some(location) => {
            match (location) {
                LoggerLocation::Super => {
                    // ERRORS HAPPEN HERE
                    let mut next_module = module.split("::").collect::<Vec<&str>>();
                    next_module.remove(next_module.len() - 1);
                    run_module_logger(next_module.join("::"), callback);
                },
                LoggerLocation::Here(logger) => {
                    callback(&logger);
                }
            }
        },
        None => panic!("Logger for module `{}` not registered.", module)
    }
}
