#![allow(dead_code)]

use chrono::prelude::DateTime;
use chrono::Local;
use std::time::{Duration, UNIX_EPOCH};

const MINUTE_1: u32 = 60 * 1000;
const MINUTE_5: u32 = 5 * MINUTE_1;
const MINUTE_15: u32 = 15 * MINUTE_1;
const MINUTE_30: u32 = 30 * MINUTE_1;
const HOURS_1: u32 = 60 * MINUTE_1;
const HOURS_6: u32 = 6 * HOURS_1;
const HOURS_12: u32 = 12 * HOURS_1;
const HOURS_24: u32 = 24 * HOURS_1;
const DAYS_1: u32 = 24 * HOURS_1;
const DAYS_7: u32 = 7 * DAYS_1;
const DAYS_30: u32 = 30 * DAYS_1;

pub fn extrahop_to_human_time(time_milliseconds: &u64) -> String {
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
            extrahop_to_human_time(&1647679315125),
            "2022-03-19 18:41:55 UTC +10:00".to_string(),
        );
    }
}
