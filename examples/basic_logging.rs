use loggerithm::{log, logger};
use loggerithm::level::{TRACE, DEBUG, INFO, NOTICE, SUCCESS, FAILURE, WARN, ERROR, FATAL};

logger!(super);

fn main() {
    log!(&TRACE, "Test message");
    log!(&DEBUG, "Test message");
    log!(&INFO, "This is similar to format!.");
    let x = 3;
    log!(&NOTICE, "Value of x is {value_of_x}", value_of_x=x);
    log!(&SUCCESS, "Test message");
    log!(&FAILURE, "Test message");
    log!(&WARN, "Test message");
    log!(&ERROR, "Test message");
    log!(&FATAL, "Test message");
}