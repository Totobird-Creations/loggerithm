use loggerithm;
use loggerithm::Logger;
use loggerithm::{register, log};
use loggerithm::level::{TRACE, DEBUG, INFO, NOTICE, SUCCESS, FAILURE, WARN, ERROR, FATAL};

fn main() {
    loggerithm::log!(TRACE   , "Test message");
    loggerithm::log!(DEBUG   , "Test message");
    loggerithm::log!(INFO    , "Test message");
    loggerithm::log!(NOTICE  , "Test message");
    loggerithm::log!(SUCCESS , "Test message");
    loggerithm::log!(FAILURE , "Test message");
    loggerithm::log!(WARN    , "Test message");
    loggerithm::log!(ERROR   , "Test message");
    loggerithm::log!(FATAL   , "Test message");
}