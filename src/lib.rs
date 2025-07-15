use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the current Unix timestamp in seconds
pub fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}
