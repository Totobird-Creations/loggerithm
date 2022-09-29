use std::collections::HashMap;
use std::sync::Mutex;

use once_cell::sync::Lazy;
use colored::Colorize;

use crate::Logger;



pub static mut LOGGERS : Lazy<Mutex<HashMap<String, Logger>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(String::new(), Logger::default());
    return Mutex::new(map);
});

pub static mut MAX_LEVEL_LEN : usize = 0;



pub fn get_logger_name(mut name : String) -> String {
    let loggers = unsafe {LOGGERS.lock().unwrap()};
    loop {
        if (name == "" || loggers.contains_key(&name)) {
            return name;
        }
        let mut split = name.split("::").collect::<Vec<&str>>();
        split.remove(split.len() - 1);
        name = split.join("::");
    }
}
