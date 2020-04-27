//! Global configuration.

use std::sync::RwLock;

lazy_static::lazy_static! {
    /// Verbose output.
    static ref IS_VERBOSE: RwLock<bool> = RwLock::new(false);
}

/// Sets the verbosity flag.
#[inline]
pub fn set_verb(b: bool) {
    *IS_VERBOSE
        .write()
        .expect("`IS_VERBOSE` flag is poisoned...") = b
}

/// True if the output should be verbose.
#[inline]
pub fn verb() -> bool {
    *IS_VERBOSE.read().expect("`IS_VERBOSE` flag is poisoned...")
}

/// Logs something on stdout if `crate::conf::verb()`.
macro_rules! log {
    (active => $stuff:expr) => (
        if $crate::conf::verb() {
            $stuff
        }
    );
    ($pref:expr => $($stuff:tt)*) => (
        log!("[{}] {}", $pref, format!($($stuff)*))
    );
    ($($stuff:tt)*) => (
        if $crate::conf::verb() {
            println!($($stuff)*)
        }
    )
}