use log::{Log, Level, Record, Metadata, SetLoggerError};

#[cfg(target_os = "linux")]
fn log(record: &Record) { }

#[cfg(target_os = "wasm32")]
extern crate sapp_wasm;
#[cfg(target_os = "wasm32")]
pub fn log(record: &Record) {
    // pick the console.log() variant for the appropriate logging level
    let console_log = match record.level() {
        Level::Error => panic!("console.error not supported"),
        Level::Warn => panic!("console.warn not supported"),
        Level::Info => panic!("console.info not supported"),
        Level::Debug => sapp_wasm::console_log,
        Level::Trace => panic!("console.debug not supported"),
    };

    console_log(&format!("{}", record.args()).into());
}

#[cfg(target_os = "wasm32")]
fn log(record: &Record) { }

#[cfg(not(any(target_os="linux", target_arch="wasm32", windows)))]
fn log(record: &Record) { }


static LOGGER: SappLogger = SappLogger {};

struct SappLogger {}

impl Log for SappLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        log(record);
    }

    fn flush(&self) {}
}

/// Initializes the global logger setting `max_log_level` to the given value.
pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)?;
    log::set_max_level(level.to_level_filter());
    Ok(())
}

/// Initializes the global logger with `max_log_level` set to `Level::Info` (a sensible default).
pub fn init() -> Result<(), SetLoggerError> {
    init_with_level(Level::Info)
}