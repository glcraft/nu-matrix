pub mod jrpc;
pub mod methods;
pub mod comm;

use log::{SetLoggerError, LevelFilter};
use simplelog::{TermLogger, Config, TerminalMode, ColorChoice};

#[inline]
pub fn init_log() -> Result<(), SetLoggerError> {
    TermLogger::init(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)
}