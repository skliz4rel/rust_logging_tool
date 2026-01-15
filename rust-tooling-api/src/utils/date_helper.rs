use chrono::{TimeZone, Utc};
use mongodb::bson::DateTime;
use std::{alloc::System, time::SystemTime};

// let start_chrono = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
// let end_chrono   = Utc.with_ymd_and_hms(2025, 12, 31, 23, 59, 59).unwrap();

// let start_bson = DateTime::from_chrono(start_chrono);
// let end_bson   = DateTime::from_chrono(end_chrono);

pub struct Converter;

impl Converter {
    pub fn convert_str_datetime(created_at: &String) -> DateTime {
        let chono_datetime: SystemTime = chrono::DateTime::parse_from_rfc3339(created_at)
            .map_err(|err| format!("Format to parse start_time: {} ", err))
            .unwrap()
            .with_timezone(&Utc)
            .into();

        DateTime::from(chono_datetime)
    }
}
