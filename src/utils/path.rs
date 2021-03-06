use ::dunce;
use std::path::PathBuf;
use std::path::Path;
use std::fs;
use logger::get_logger;
use base_logging::Level;
use std::error::Error;
use errors::PupWorkerError;
use std::fs::File;

/// Treat platforms uniformly regardless of mix and matching formats
pub fn join<U: AsRef<Path>, V: AsRef<Path>>(a: U, b: V) -> PathBuf {
    let mut buffer = PathBuf::new();
    for component in a.as_ref().components() {
        buffer.push(component);
    }
    for component in b.as_ref().components() {
        buffer.push(component);
    }
    return buffer;
}

/// Check if a file exists
pub fn exists<P: AsRef<Path>>(path: P) -> bool {
    return fs::metadata(path.as_ref()).is_ok();
}

/// Return the canonical *display* path for a path.
pub fn display<P: AsRef<Path>>(path: P) -> String {
    return match dunce::canonicalize(path.as_ref()) {
        Ok(p) => {
            p.display().to_string()
        }
        Err(err) => {
            let rtn = path.as_ref().display().to_string();
            match get_logger() {
                Ok(mut logger) => { logger.log(Level::Debug, format!("Failed to normalize path: {:?}: {}", rtn, err.description())) }
                Err(_) => {}
            }
            rtn
        }
    };
}

// Write a 0-byte output file to the target path
pub fn blat<P: AsRef<Path>>(path: P) -> Result<(), PupWorkerError> {
    PupWorkerError::wrap(File::create(path.as_ref()))?;
    Ok(())
}