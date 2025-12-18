use std::io;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

pub struct RawModeGuard;

impl RawModeGuard {
    pub fn new() -> Result<Self, io::Error> {
        enable_raw_mode()?;
        Ok(Self)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        // Never panic in Drop
        let _ = disable_raw_mode();
    }
}
