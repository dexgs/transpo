use chrono::offset::Utc;
use core::time::Duration;

pub type DateTime = chrono::DateTime<Utc>;

pub fn expiry(days: u32, hours: u32, minutes: u32) -> DateTime {
    let offset = Duration::new((days * 86400 + hours * 3600 + minutes * 60) as u64, 0);
    now() + chrono::Duration::from_std(offset).unwrap()
}

pub fn now() -> DateTime {
    Utc::now()
}
