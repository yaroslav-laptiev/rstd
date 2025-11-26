use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};

pub fn str_to_local_dt(s: &str) -> Option<DateTime<Local>> {
    // Try full datetime first
    if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%d/%m/%Y %H:%M") {
        let res = Local.from_local_datetime(&dt);
        return match res {
            chrono::offset::LocalResult::Single(t) => Some(t),
            chrono::offset::LocalResult::Ambiguous(t1, _) => Some(t1),
            chrono::offset::LocalResult::None => None,
        };
    }

    // Try only date
    if let Ok(date) = NaiveDate::parse_from_str(s, "%d/%m/%Y") {
        let dt = date.and_time(NaiveTime::from_hms_opt(0, 0, 0)?);
        let res = Local.from_local_datetime(&dt);
        return match res {
            chrono::offset::LocalResult::Single(t) => Some(t),
            chrono::offset::LocalResult::Ambiguous(t1, _) => Some(t1),
            chrono::offset::LocalResult::None => None,
        };
    }

    None
}

pub fn db_timestamp_to_local_dt(dt_str: &str) -> DateTime<Local> {
    DateTime::parse_from_rfc3339(dt_str)
        .map(|dt| dt.with_timezone(&Local))
        .unwrap_or_else(|_| {
            let naive = NaiveDateTime::parse_from_str(dt_str, "%Y-%m-%d %H:%M:%S").unwrap();
            chrono::Local.from_local_datetime(&naive).unwrap()
        })
}
