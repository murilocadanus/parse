#[macro_use]
extern crate log;
extern crate env_logger;
extern crate i9;
extern crate time;

use std::env;
use log::{LogRecord, LogLevelFilter};
use env_logger::LogBuilder;

fn main() {
    let format = |record: &LogRecord| {
        let t = time::now();
        format!("[{}.{:03}] - [{}] - {}",
            time::strftime("%d-%m-%Y %H:%M:%S", &t).unwrap(),
            t.tm_nsec / 1000_000,
            record.level(),
            record.args()
        )
    };

    let mut builder = LogBuilder::new();
    builder.format(format).filter(None, LogLevelFilter::Info);

    if env::var("RUST_LOG").is_ok() {
        builder.parse(&env::var("RUST_LOG").unwrap());
    }

    builder.init().unwrap();

    info!(target: "tag", "Initialize");
    i9::init();
}