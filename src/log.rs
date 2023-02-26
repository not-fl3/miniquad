/// This module is disabled by default
///
/// Most of the code gleaned from log-rs crate
///
/// Will send log calls like debug!(), warn!() and error!() to appropriate console_* call on wasm
/// and just println! on PC.
/// If you need better control of log messages - just dont use "log-impl" feature and use appropriate loggers from log-rs
use std::cmp;

#[repr(usize)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Level {
    /// The "error" level.
    ///
    /// Designates very serious errors.
    Error = 1, // This way these line up with the discriminants for LevelFilter below
    /// The "warn" level.
    ///
    /// Designates hazardous situations.
    Warn,
    /// The "info" level.
    ///
    /// Designates useful information.
    Info,
    /// The "debug" level.
    ///
    /// Designates lower priority information.
    Debug,
    /// The "trace" level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    Trace,
}

impl PartialOrd for Level {
    #[inline]
    fn partial_cmp(&self, other: &Level) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }

    #[inline]
    fn lt(&self, other: &Level) -> bool {
        (*self as usize) < *other as usize
    }

    #[inline]
    fn le(&self, other: &Level) -> bool {
        *self as usize <= *other as usize
    }

    #[inline]
    fn gt(&self, other: &Level) -> bool {
        *self as usize > *other as usize
    }

    #[inline]
    fn ge(&self, other: &Level) -> bool {
        *self as usize >= *other as usize
    }
}

impl Ord for Level {
    #[inline]
    fn cmp(&self, other: &Level) -> cmp::Ordering {
        (*self as usize).cmp(&(*other as usize))
    }
}

#[macro_export(local_inner_macros)]
macro_rules! log {
    (target: $target:expr, $lvl:expr, $message:expr) => ({
        let lvl = $lvl;
        //if lvl <= $crate::STATIC_MAX_LEVEL && lvl <= $crate::max_level() {
            // ensure that $message is a valid format string literal
            let _ = __log_format_args!($message);
            $crate::log::__private_api_log_lit(
                $message,
                lvl,
                &($target, __log_module_path!(), __log_file!(), __log_line!()),
            );
        //}
    });
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => ({
        let lvl = $lvl;
        //if lvl <= $crate::STATIC_MAX_LEVEL && lvl <= $crate::max_level() {
            $crate::log::__private_api_log_lit(
                &__log_format_args!($($arg)+),
                lvl,
                &($target, __log_module_path!(), __log_file!(), __log_line!()),
            );
        //}
    });
    ($lvl:expr, $($arg:tt)+) => (log!(target: __log_module_path!(), $lvl, $($arg)+))
}

#[macro_export(local_inner_macros)]
macro_rules! error {
    (target: $target:expr, $($arg:tt)+) => (
        log!(target: $target, $crate::Level::Error, $($arg)+);
    );
    ($($arg:tt)+) => (
        log!($crate::log::Level::Error, $($arg)+);
    )
}

#[macro_export(local_inner_macros)]
macro_rules! warn {
    (target: $target:expr, $($arg:tt)+) => (
        log!(target: $target, $crate::Level::Warn, $($arg)+);
    );
    ($($arg:tt)+) => (
        log!($crate::log::Level::Warn, $($arg)+);
    )
}

#[macro_export(local_inner_macros)]
macro_rules! info {
    (target: $target:expr, $($arg:tt)+) => (
        log!(target: $target, $crate::Level::Info, $($arg)+);
    );
    ($($arg:tt)+) => (
        log!($crate::log::Level::Info, $($arg)+);
    )
}

#[macro_export(local_inner_macros)]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => (
        log!(target: $target, $crate::Level::Debug, $($arg)+);
    );
    ($($arg:tt)+) => (
        log!($crate::log::Level::Debug, $($arg)+);
    )
}

#[macro_export(local_inner_macros)]
macro_rules! trace {
    (target: $target:expr, $($arg:tt)+) => (
        log!(target: $target, $crate::Level::Trace, $($arg)+);
    );
    ($($arg:tt)+) => (
        log!($crate::log::Level::Trace, $($arg)+);
    )
}

/// log-rs used `macro_export(local_inner_macros)` instead of $crate::log! to support older rustc version
/// but actually there is an other difference - $crate::log does not support macros reexport :(
/// so even miniquad is fine with 1.31+ rustc version, we need to use local_inner_macros as well
#[doc(hidden)]
#[macro_export]
macro_rules! __log_format_args {
    ($($args:tt)*) => {
        format!($($args)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_module_path {
    () => {
        module_path!()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_file {
    () => {
        file!()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_line {
    () => {
        line!()
    };
}

#[cfg(not(any(target_arch = "wasm32", target_os = "android", target_os = "ios")))]
pub fn __private_api_log_lit(
    message: &str,
    _level: Level,
    &(_target, _module_path, _file, _line): &(&str, &'static str, &'static str, u32),
) {
    eprintln!("{}", message);
}

#[cfg(target_arch = "wasm32")]
pub fn __private_api_log_lit(
    message: &str,
    level: Level,
    &(_target, _module_path, _file, _line): &(&str, &'static str, &'static str, u32),
) {
    use crate::native::wasm;
    use std::ffi::CString;

    let log_fn = match level {
        Level::Debug => wasm::console_debug,
        Level::Warn => wasm::console_warn,
        Level::Info => wasm::console_info,
        Level::Trace => wasm::console_debug,
        Level::Error => wasm::console_error,
    };
    let msg = CString::new(message).unwrap_or_else(|_| panic!());

    unsafe { log_fn(msg.as_ptr()) };
}

#[cfg(target_os = "android")]
pub fn __private_api_log_lit(
    message: &str,
    level: Level,
    &(_target, _module_path, _file, _line): &(&str, &'static str, &'static str, u32),
) {
    use std::ffi::CString;

    let log_fn = match level {
        Level::Debug => crate::native::android::console_debug,
        Level::Warn => crate::native::android::console_warn,
        Level::Info => crate::native::android::console_info,
        Level::Trace => crate::native::android::console_debug,
        Level::Error => crate::native::android::console_error,
    };
    let msg = CString::new(message).unwrap_or_else(|_| panic!());

    unsafe { log_fn(msg.as_ptr()) };
}

#[cfg(target_os = "ios")]
pub fn __private_api_log_lit(
    message: &str,
    _level: Level,
    &(_target, _module_path, _file, _line): &(&str, &'static str, &'static str, u32),
) {
    crate::native::ios::log(message);
}

#[test]
fn test_logs() {
    trace!("info");
    trace!("info: {}", 1);

    debug!("info");
    debug!("info: {}", 1);

    info!("info");
    info!("info: {}", 1);

    warn!("info");
    warn!("info: {}", 1);

    error!("info");
    error!("info: {}", 1);
}
