use std::collections::HashMap;
use std::sync::Mutex;
use std::lazy::Lazy;

use crate::Logger;



pub static mut LOGGERS : Lazy<Mutex<HashMap<String, Box<dyn Logger>>>> = Lazy::new(|| {
    return Mutex::new(HashMap::new());
});

pub fn get_path() -> String {
    return module_path!().to_string();
}
