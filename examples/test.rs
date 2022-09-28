use loggerithm::log;
use loggerithm::level::{TRACE, DEBUG, INFO, NOTICE, SUCCESS, FAILURE, WARN, ERROR, FATAL};

fn main() {
    log!(TRACE   , "Test message");
    log!(DEBUG   , "Test message");
    log!(INFO    , "Test message");
    log!(NOTICE  , "Test message");
    log!(SUCCESS , "Test message");
    log!(FAILURE , "Test message");
    log!(WARN    , "Test message");
    log!(ERROR   , "Test message");
    log!(FATAL   , "Test message");
}