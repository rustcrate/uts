use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the current Unix timestamp in seconds
///
/// # Examples
///
/// ```
/// let ts = timestamp::timestamp();
/// println!("Current timestamp: {}", ts);
/// ```
pub fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp() {
        let ts1 = timestamp();
        let ts2 = timestamp();
        assert!(ts2 >= ts1);
    }
}
