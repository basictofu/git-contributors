use chrono::{FixedOffset, Local, LocalResult, NaiveDateTime};

use crate::cli;

fn parse_date_str_to_utc(s: &str, local_tz_offset: Option<i32>) -> Result<u32, &str> {
    if let Ok(naive) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%SZ") {
        // String is a utc date => ignore local timezone
        let ts = naive.and_utc().timestamp();
        if ts < 0 {
            return Err("Cannot handle dates before the epoch");
        }
        return Ok(ts as u32);
    } else {
        // String is a local date => include local timezone
        if let Some(offset) = local_tz_offset {
            let tz = FixedOffset::east_opt(offset).unwrap();
            if let Ok(naive) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
                if let LocalResult::Single(aware) = naive.and_local_timezone(tz) {
                    let ts = aware.timestamp();
                    if ts < 0 {
                        return Err("Cannot handle dates before the epoch");
                    }
                    return Ok(ts as u32);
                } else {
                    return Err("Could not determine unique datetime");
                }
            } else {
                return Err("Could not parse local date time");
            }
        } else {
            return Err("Cannot parse local datetime without timezone");
        }
    }
}

/// Resolve user-provided date range from CLI arguments
fn resolve_user_range(args: &cli::Args) -> (Option<u32>, Option<u32>) {
    let mut user_range: (Option<u32>, Option<u32>) = (None, None);
    let offset = Local::now().offset().local_minus_utc();
    if let Some(start) = &args.start {
        let timestamp = parse_date_str_to_utc(start, Some(offset)).unwrap();
        user_range.0 = Some(timestamp);
    }
    if let Some(end) = &args.end {
        let timestamp = parse_date_str_to_utc(end, Some(offset)).unwrap();
        user_range.1 = Some(timestamp);
    }
    user_range
}

pub fn resolve_date_range(data_range: (u32, u32), args: &cli::Args) -> (u32, u32) {
    // Default range is beginning of data to the current time
    let current_seconds = Local::now().timestamp() as u32;
    let mut resolved_range: (u32, u32) = (data_range.0, current_seconds);

    // Overwrite with user-provided values
    let user_range = resolve_user_range(args);
    if let Some(start) = user_range.0 {
        resolved_range.0 = start;
    }
    if let Some(end) = user_range.1 {
        resolved_range.1 = end;
    }

    resolved_range
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_utc() {
        let actual = parse_date_str_to_utc("2023-10-27 14:30:00Z", None).unwrap();
        let expected: u32 = 1698417000;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_local() {
        let actual = parse_date_str_to_utc("2023-10-27 14:30:00", Some(-5 * 3600)).unwrap();
        let expected: u32 = 1698435000;
        assert_eq!(actual, expected);
    }
}
