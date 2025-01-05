use std::str::FromStr;

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, Offset, Utc};

#[derive(Debug, Clone)]
pub struct SerializableDateTime(pub DateTime<Utc>);

impl FromStr for SerializableDateTime {
    type Err = InvalidDateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(unix_time) = s.parse::<i64>() {
            if let Some(value) = DateTime::from_timestamp(unix_time, 0) {
                return Ok(SerializableDateTime(value));
            }
        }
        let split = s.split(' ');
        let (mut year, mut month, mut day, mut hour, mut minute, mut second) = (0, 0, 0, 0, 0, 0);
        let mut time_zone: SerializableTimeZone = SerializableTimeZone::default();
        for part in split {
            if let Ok(parsed_time_zone) = part.parse::<SerializableTimeZone>() {
                time_zone = parsed_time_zone;
                continue;
            }
            let Some(suffix_start) = part.rfind(|c: char| c.is_ascii_digit()) else {
                continue;
            };
            let (prefix, suffix) = (&part[..=suffix_start], &part[suffix_start + 1..]);
            let Ok(value) = prefix.parse::<u32>() else {
                continue;
            };
            match suffix {
                "Y" => year = value,
                "M" => month = value,
                "D" => day = value,
                "HR" => hour = value,
                "MIN" => minute = value,
                "SEC" => second = value,
                _ => continue,
            }
        }
        Ok(Self(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(year as i32, month, day).ok_or(InvalidDateError)?,
                NaiveTime::from_hms_opt(hour, minute, second).ok_or(InvalidDateError)?,
            )
            .and_local_timezone(time_zone.0)
            .single()
            .ok_or(InvalidDateError)?
            .into(),
        ))
    }
}

#[derive(Debug)]
pub struct InvalidDateError;
pub struct InvalidTimeZoneError;

pub struct SerializableTimeZone(FixedOffset);

impl Default for SerializableTimeZone {
    fn default() -> Self {
        Self(Utc.fix())
    }
}

impl FromStr for SerializableTimeZone {
    type Err = InvalidTimeZoneError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 3 || !s[0..3].eq_ignore_ascii_case("UTC") {
            return Err(InvalidTimeZoneError);
        }
        let time_zone_str = &s[3..];
        let negative = time_zone_str.starts_with('-');
        let valid = negative || time_zone_str.starts_with('+');
        if !valid {
            return Err(InvalidTimeZoneError);
        }
        let time_zone_str = &s[4..];
        if let Some(hr_min_separator) = time_zone_str.find(':') {
            let Ok(time_zone_hours) = time_zone_str[..hr_min_separator].parse::<i32>() else {
                return Err(InvalidTimeZoneError);
            };
            let Ok(time_zone_minutes) = time_zone_str[hr_min_separator + 1..].parse::<i32>() else {
                return Err(InvalidTimeZoneError);
            };
            Ok(Self(
                FixedOffset::east_opt(time_zone_hours * 3600 + time_zone_minutes * 60)
                    .ok_or(InvalidTimeZoneError)?,
            ))
        } else {
            let Ok(time_zone_hours) = time_zone_str.parse::<i32>() else {
                return Err(InvalidTimeZoneError);
            };
            Ok(Self(
                FixedOffset::east_opt(time_zone_hours * 3600).ok_or(InvalidTimeZoneError)?,
            ))
        }
    }
}
