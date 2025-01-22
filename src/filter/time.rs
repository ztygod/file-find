use chrono::{DateTime, Local, NaiveDate, NaiveDateTime};
use std::time::SystemTime;

#[derive(Debug, PartialEq, Eq)]
pub enum TimeFilter {
    Before(SystemTime),
    After(SystemTime),
}

impl TimeFilter {
    /*
    这个函数的目的是提供多种方式将字符串解析为 SystemTime 类型。它支持解析：
    1.相对时间（如 "1h"）
    2.RFC 3339 格式（如 "2025-01-21T12:34:56Z"）
    3.简单日期格式（如 "2025-01-21"）
    4.带时间的日期时间（如 "2025-01-21 12:34:56"）
    5.Unix 时间戳（如 "@1609459200"）
     */
    fn from_str(ref_time: &SystemTime, s: &str) -> Option<SystemTime> {
        humantime::parse_duration(s)
            .map(|duration| *ref_time - duration)
            .ok()
            .or_else(|| {
                DateTime::parse_from_rfc3339(s)
                    .map(|dt| dt.into())
                    .ok()
                    .or_else(|| {
                        NaiveDate::parse_from_str(s, "%F")
                            .ok()?
                            .and_hms_opt(0, 0, 0)?
                            .and_local_timezone(Local)
                            .latest()
                    })
                    .or_else(|| {
                        NaiveDateTime::parse_from_str(s, "%F %T")
                            .ok()?
                            .and_local_timezone(Local)
                            .latest()
                    })
                    .or_else(|| {
                        let timestamp_secs = s.strip_prefix('@')?.parse().ok()?;
                        DateTime::from_timestamp(timestamp_secs, 0).map(Into::into)
                    })
                    .map(|dt| dt.into())
            })
    }

    pub fn before(ref_time: &SystemTime, s: &str) -> Option<TimeFilter> {
        TimeFilter::from_str(ref_time, s).map(TimeFilter::Before)
    }

    pub fn after(ref_time: &SystemTime, s: &str) -> Option<TimeFilter> {
        TimeFilter::from_str(ref_time, s).map(TimeFilter::After)
    }

    pub fn applies_to(&self, t: &SystemTime) -> bool {
        match self {
            TimeFilter::Before(limit) => t < limit,
            TimeFilter::After(limit) => t > limit,
        }
    }
}
