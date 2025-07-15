use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the current Unix timestamp in seconds
pub fn time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Invalid time")
        .as_secs()
}
