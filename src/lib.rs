use std::time::{SystemTime, UNIX_EPOCH};

/// Current Unix timestamp in seconds
pub fn time() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}
