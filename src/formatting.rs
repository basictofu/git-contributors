use chrono::{DateTime, FixedOffset, Local};

pub fn format_date_with_offset(date: u32, tz_offset_seconds: Option<i32>) -> String {
    let local_tz_offset: i32 = match tz_offset_seconds {
        Some(offset) => offset,
        None => Local::now().offset().local_minus_utc(),
    };

    let dt = DateTime::from_timestamp(date as i64, 0).unwrap();
    let tz = FixedOffset::east_opt(local_tz_offset).unwrap();
    let d = dt.with_timezone(&tz);
    d.to_rfc3339()[..10].to_string()
}

pub fn generate_axis(width: u64, range: (u32, u32), tz_offset_seconds: Option<i32>) -> String {
    let start = format_date_with_offset(range.0, tz_offset_seconds);
    let end = format_date_with_offset(range.1, tz_offset_seconds);

    let gap_size = width - (start.len() as u64 + end.len() as u64);
    let gap: String = " ".repeat(gap_size as usize);

    start + &gap + &end
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_axis() {
        let width = 23;
        let start: u32 = 1698435000; // 2:30 pm in CST
        let end: u32 = start + 10 * 3600; // 10 hr later, next morning
        let range = (start, end);
        let axis = generate_axis(width, range, Some(-5 * 3600));
        assert_eq!(axis, "2023-10-27   2023-10-28");
    }

    #[test]
    fn test_format_date() {
        let date: u32 = 1698435000;
        let formatted = format_date_with_offset(date, -5 * 3600);
        assert_eq!(formatted, "2023-10-27");
    }
}
