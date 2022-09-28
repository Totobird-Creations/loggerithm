use std::collections::HashMap;
use std::sync::Mutex;

use once_cell::sync::Lazy;

use crate::level;
use crate::Logger;



pub static mut LOGGERS : Lazy<Mutex<HashMap<String, Logger>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(String::new(), Logger::new()
        .add_target(|context| {
            println!("[ {} ] [ {}:{} ]",
                context.level_npf(),
                context.module(),
                context.line()
            )
        })
        .default_formatting()
    );
    return Mutex::new(map);
});

pub static mut MIN_SEVERITY : u32 = level::INFO.get_severity();

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
