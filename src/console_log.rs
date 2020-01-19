use log::{Log, Level, Record, Metadata, SetLoggerError};

extern crate sapp_wasm;
fn log_record(record: &Record) {
    // pick the console.log() variant for the appropriate logging level
    let console_log = match record.level() {
        Level::Error => panic!("console.error not supported"),
        Level::Warn => panic!("console.warn not supported"),
        Level::Info => panic!("console.info not supported"),
        Level::Debug => sapp_wasm::console_log,
        Level::Trace => panic!("console.debug not supported"),
    };

    console_log(&format!("{}", record.args()));
}

static LOGGER: WasmLogger = WasmLogger {};

struct WasmLogger {}

impl Log for WasmLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        log_record(record);
    }

    fn flush(&self) {}
}

/// Initializes the global logger setting `max_log_level` to the given value.
pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)?;
    log::set_max_level(level.to_level_filter());
    Ok(())
}

/// Initializes the global logger with `max_log_level` set to `Level::Debug` (the only supported level for now).
pub fn init() -> Result<(), SetLoggerError> {
    init_with_level(Level::Debug)
}