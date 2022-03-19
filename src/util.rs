use chrono::prelude::DateTime;
use chrono::Local;
use std::time::{Duration, UNIX_EPOCH};

#[allow(dead_code)]
pub fn extrahop_to_human_time(time_milliseconds: u64) -> String {
    let time_seconds = UNIX_EPOCH + Duration::from_secs(time_milliseconds / 1000);
    let datetime = DateTime::<Local>::from(time_seconds);
    datetime.format("%Y-%m-%d %H:%M:%S UTC %Z").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extrahop_to_human_time() {
        assert_eq!(
            extrahop_to_human_time(1647679315125),
            String::from("2022-03-19 18:41:55 UTC +10:00"),
        );
    }
}
