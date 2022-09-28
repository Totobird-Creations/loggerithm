use loggerithm;

struct MyLogger;
impl loggerithm::Logger for MyLogger {}

fn main() {
    loggerithm::init!(MyLogger);
}