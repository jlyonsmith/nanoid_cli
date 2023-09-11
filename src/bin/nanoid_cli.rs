use core::fmt::Arguments;
use nanoid_cli::{error, NanoidCliLog, NanoidCliTool};
use yansi::Paint;

struct NanoidCliLogger;

impl NanoidCliLogger {
    fn new() -> NanoidCliLogger {
        NanoidCliLogger {}
    }
}

impl NanoidCliLog for NanoidCliLogger {
    fn output(self: &Self, args: Arguments) {
        println!("{}", args);
    }
    fn warning(self: &Self, args: Arguments) {
        eprintln!("{}", format!("warning: {}", Paint::yellow(args)));
    }
    fn error(self: &Self, args: Arguments) {
        eprintln!("{}", format!("error: {}", Paint::red(args)));
    }
}

fn main() {
    let logger = NanoidCliLogger::new();

    if let Err(error) = NanoidCliTool::new(&logger).run(std::env::args_os()) {
        error!(logger, "{}", error);
        std::process::exit(1);
    }
}
