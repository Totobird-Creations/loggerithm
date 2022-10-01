//#![doc(hidden)]
//! Internal objects and functions.
//! 
//! For the most part, unless you know 
//! what you're doing, don't touch this
//! module.



use std::collections::HashMap;

use static_init::dynamic;

use crate::Logger;



/// An object representing which logger
/// to use in a module.
pub enum LoggerLocation {
    /// Use the logger in the parent module.
    Super,
    /// Use a logger defined for the module.
    Here(Logger)
}
/// Thread safety.
unsafe impl Send for LoggerLocation {}


/// An object that stores loggers for
/// each registered module.
/// 
/// The key is the module path.
/// 
/// The value is the LoggerLocation pointing
/// to the logger for the module path.
/// 
/// Contains a default logger at the root.
#[dynamic]
pub static mut LOGGERS : HashMap<String, LoggerLocation> = {
    let mut map = HashMap::new();
    map.insert(String::new(), LoggerLocation::Here(Logger::default()));
    map
};

/// The length of the largest logging
/// level name.
/// 
/// Used in order to pad the level names
/// to a consistent value.
pub static mut MAX_LEVEL_NAME_LEN : usize = 0;

/// The length of the largest module path.
/// 
/// Used in order to pad the module paths
/// to a consistent value.
pub static mut MAX_MODULE_LEN : usize = 0;



/// Gets the logger of a module, then
/// calls the callback function.
/// 
/// # Arguments
/// 
/// * `module`   - The module to get the logger
///                of.
/// * `first`    - Always pass in `true`.
/// * `callback` - A callback function that will
///                be run when a logger is found.
/// 
/// # Generics
/// 
/// * `F` - The callback method.
pub fn run_module_logger<F>(module : String, first : bool, callback : F)
    where F : Fn(&Logger)
{
    let mut next_module_vec = module.split("::").collect::<Vec<&str>>();
    next_module_vec.remove(next_module_vec.len() - 1);
    let next_module = next_module_vec.join("::");

    match (LOGGERS.read().get(&module)) {
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


#[doc(hidden)]
/// Internal macro.
/// 
/// For the most part, unless you
/// know what you're doing, don't
/// touch this macro.
#[macro_export]
macro_rules! __logger_internal {
    ($location:expr) => {
        /// A logger.
        #[allow(non_snake_case)]
        mod __loggerithm_LOGGER {
            extern crate static_init;
            /// The logger handler object.
            #[static_init::dynamic]
            static LOGGER : () = {
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
            /// Used by the `log!` macro to
            /// check if there is a logger
            /// registered for the module.
            #[allow(dead_code)]
            pub fn void() {}
        }
    };
}
