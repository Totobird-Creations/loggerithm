use backtrace;

pub fn test_func() {
    backtrace::trace(|frame| {
        let ip   = frame.ip();
        let addr = frame.symbol_address();
        backtrace::resolve_frame(frame, |symbol| {
            if (let Some(name) = symbol.name()) {
                println!("{}", name);
            }
        });
        return true;
    });
}
