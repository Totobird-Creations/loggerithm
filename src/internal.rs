use std::collections::HashMap;
use std::sync::Mutex;
use std::lazy::Lazy;

use colored::Colorize;

use crate::Logger;
use crate::level;



pub static mut LOGGERS : Lazy<Mutex<HashMap<String, Logger>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(String::new(), Logger::new()
        .add_formatter(level::TRACE   , |v| v.bright_black()                 )
        .add_formatter(level::DEBUG   , |v| v.white().dimmed()               )
        .add_formatter(level::INFO    , |v| v.cyan().dimmed()                )
        .add_formatter(level::NOTICE  , |v| v.bright_cyan()                  )
        .add_formatter(level::SUCCESS , |v| v.green()                        )
        .add_formatter(level::FAILURE , |v| v.red()                          )
        .add_formatter(level::WARN    , |v| v.yellow()                       )
        .add_formatter(level::ERROR   , |v| v.bright_red().bold()            )
        .add_formatter(level::FATAL   , |v| v.bright_white().bold().on_red() )
    );
    return Mutex::new(map);
});
pub static mut MIN_SEVERITY : i32 = level::INFO.severity;


pub fn get_logger_name(mut name : String) -> String {
    let loggers = unsafe {
        LOGGERS.lock().unwrap()
    };
    loop {
        if (name == "" || loggers.contains_key(&name)) {
            return name;
        }
        let mut split = name.split("::").collect::<Vec<&str>>();
        split.remove(split.len() - 1);
        name = split.join("::");
    }
}
