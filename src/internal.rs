use std::collections::HashMap;
use std::fmt;

use static_init::dynamic;

use crate::Logger;


pub enum LoggerLocation {
    Super,
    Here(Logger)
}
impl fmt::Display for LoggerLocation {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        return write!(f, "{}", match (self) {
            LoggerLocation::Super        => String::from("SUPER"),
            LoggerLocation::Here(logger) => format!("{}", logger)
        });
    }
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

pub static mut MAX_LEVEL_NAME_LEN : usize = 0;
pub static mut MAX_MODULE_LEN     : usize = 0;



pub fn run_module_logger<F>(module : String, first : bool, callback : F)
    where F : Fn(&Logger)
{
    let mut next_module_vec = module.split("::").collect::<Vec<&str>>();
    next_module_vec.remove(next_module_vec.len() - 1);
    let next_module = next_module_vec.join("::");

    #[allow(unused_unsafe)]
    match (unsafe {LOGGERS.read()}.get(&module)) {
        Some(location) => {
            match (location) {
                LoggerLocation::Super => {
                    run_module_logger(next_module, false, callback);
                },
                LoggerLocation::Here(logger) => {
                    callback(&logger);
                }
            }
        },
        None => {
            if (first) {
                panic!("Logger for module `{}` not registered.", module);
            } else {
                run_module_logger(next_module, false, callback);
            }
        }
    }
}



#[macro_export]
macro_rules! logger_internal {
    ($location:expr) => {
        pub mod LOGGER {
            use static_init::dynamic;
            #[dynamic]
            static __LOGGER : () = {
                let mut module_vec = module_path!().split("::").collect::<Vec<&str>>();
                module_vec.remove(module_vec.len() - 1);
                let module = module_vec.join("::");
                if (unsafe {$crate::internal::MAX_MODULE_LEN} < module.len()) {
                    unsafe {
                        $crate::internal::MAX_MODULE_LEN = module.len();
                    }
                }
                unsafe {$crate::internal::LOGGERS.write()}
                    .insert(module, $location);
            };
        }
    };
}
