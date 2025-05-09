// src/utils/datetime_utils.rs

use chrono::{DateTime, Utc, NaiveDate, NaiveTime, NaiveDateTime, Datelike, Timelike};

/// Gets the current UTC date and time.
pub fn now_utc() -> DateTime<Utc> {
    Utc::now()
}

/// Formats a DateTime<Utc> object into a string.
/// Default format: YYYY-MM-DD HH:MM:SS UTC
pub fn format_datetime_utc(dt: &DateTime<Utc>, format_str: Option<&str>) -> String {
    dt.format(format_str.unwrap_or("%Y-%m-%d %H:%M:%S UTC")).to_string()
}

/// Parses a date string into a NaiveDate object.
/// Expects format: YYYY-MM-DD
pub fn parse_date(date_str: &str) -> Result<NaiveDate, chrono::ParseError> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
}

/// Parses a time string into a NaiveTime object.
/// Expects format: HH:MM:SS
pub fn parse_time(time_str: &str) -> Result<NaiveTime, chrono::ParseError> {
    NaiveTime::parse_from_str(time_str, "%H:%M:%S")
}

/// Parses a datetime string into a NaiveDateTime object.
/// Expects format: YYYY-MM-DD HH:MM:SS
pub fn parse_datetime(datetime_str: &str) -> Result<NaiveDateTime, chrono::ParseError> {
    NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S")
}

/// Gets the year from a NaiveDate or DateTime<Utc>.
pub fn year<T: Datelike>(date_like: &T) -> i32 {
    date_like.year()
}

/// Gets the month from a NaiveDate or DateTime<Utc>.
pub fn month<T: Datelike>(date_like: &T) -> u32 {
    date_like.month()
}

/// Gets the day from a NaiveDate or DateTime<Utc>.
pub fn day<T: Datelike>(date_like: &T) -> u32 {
    date_like.day()
}

/// Gets the hour from a NaiveTime or DateTime<Utc>.
pub fn hour<T: Timelike>(time_like: &T) -> u32 {
    time_like.hour()
}

/// Gets the minute from a NaiveTime or DateTime<Utc>.
pub fn minute<T: Timelike>(time_like: &T) -> u32 {
    time_like.minute()
}

/// Gets the second from a NaiveTime or DateTime<Utc>.
pub fn second<T: Timelike>(time_like: &T) -> u32 {
    time_like.second()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now_utc_and_format() {
        let now = now_utc();
        let formatted = format_datetime_utc(&now, None);
        // Basic check, format might vary slightly if second ticks over during test
        assert!(formatted.contains(&now.year().to_string()));
        assert!(formatted.ends_with("UTC"));

        let custom_formatted = format_datetime_utc(&now, Some("%Y/%m/%d %H:%M"));
        assert!(custom_formatted.contains(&format!("{}/{}", now.year(), format!("{:02}", now.month()))));
    }

    #[test]
    fn test_parse_date() {
        let date = parse_date("2023-10-26").unwrap();
        assert_eq!(year(&date), 2023);
        assert_eq!(month(&date), 10);
        assert_eq!(day(&date), 26);
        assert!(parse_date("invalid-date").is_err());
    }

    #[test]
    fn test_parse_time() {
        let time = parse_time("14:30:05").unwrap();
        assert_eq!(hour(&time), 14);
        assert_eq!(minute(&time), 30);
        assert_eq!(second(&time), 5);
        assert!(parse_time("invalid-time").is_err());
    }

    #[test]
    fn test_parse_datetime() {
        let dt = parse_datetime("2023-10-26 14:30:05").unwrap();
        assert_eq!(year(&dt), 2023);
        assert_eq!(month(&dt), 10);
        assert_eq!(day(&dt), 26);
        assert_eq!(hour(&dt), 14);
        assert_eq!(minute(&dt), 30);
        assert_eq!(second(&dt), 5);
        assert!(parse_datetime("invalid-datetime").is_err());
    }

    #[test]
    fn test_date_components() {
        let date = NaiveDate::from_ymd_opt(2024, 7, 15).unwrap();
        assert_eq!(year(&date), 2024);
        assert_eq!(month(&date), 7);
        assert_eq!(day(&date), 15);
    }

    #[test]
    fn test_time_components() {
        let time = NaiveTime::from_hms_opt(22, 5, 30).unwrap();
        assert_eq!(hour(&time), 22);
        assert_eq!(minute(&time), 5);
        assert_eq!(second(&time), 30);
    }
}
