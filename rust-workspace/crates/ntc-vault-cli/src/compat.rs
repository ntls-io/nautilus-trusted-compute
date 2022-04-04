//! Compatibility code.

use std::io;
use std::path::Path;

/// See: [`Path::try_exists`]
///
/// TODO: Drop this once `path_try_exists` is stable.
///
/// Tracking Issue: <https://github.com/rust-lang/rust/issues/83186>
pub fn try_exists(path: &Path) -> io::Result<bool> {
    match path.metadata() {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error),
    }
}
