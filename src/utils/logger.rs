use log::{LevelFilter, SetLoggerError};
use simplelog::{Config, SimpleLogger};

pub fn setup_logger(verbosity: u64) -> Result<(), SetLoggerError> {
    let level = match verbosity {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        3 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    SimpleLogger::init(level, Config::default())
}
