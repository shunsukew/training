#[macro_use]
extern crate log;

use log::{Record, Level, Metadata, LevelFilter, SetLoggerError};

static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("Rust says: {} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

fn main() -> Result<(), SetLoggerError> {
    log::set_logger(&CONSOLE_LOGGER)?;
    log::set_max_level(LevelFilter::Info);

    info!("hello log");
    warn!("warning");
    error!("oops");
    Ok(())
}


// use a custom enviroment
// #[macro_use]
// extern crate log;
// extern crate env_logger;

// use std::env;
// use env_logger::Builder;

// fn main() {
//     Builder::new()
//         .parse(&env::var("MY_APP_LOG").unwrap_or_default())
//         .init();

//     info!("informational message");
//     warn!("warning message");
//     error!("this is an error {}", "message");
// }
