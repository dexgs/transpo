use chrono::offset::Utc;
use core::time::Duration;

use actix_web::{web, Result};

use std::path::PathBuf;
use std::fs;
use std::io::Read;

pub type DateTime = chrono::DateTime<Utc>;

pub fn expiry(days: u32, hours: u32, minutes: u32) -> DateTime {
    let offset = Duration::new((days * 86400 + hours * 3600 + minutes * 60) as u64, 0);
    now() + chrono::Duration::from_std(offset).unwrap()
}

pub fn now() -> DateTime {
    Utc::now()
}

pub async fn is_expired(path: &PathBuf) -> bool {
    let path = path.join("expiry_date");
    web::block(move || {
        is_expired_sync(path)
    }).await.unwrap_or(true)
}

pub fn is_expired_sync(path: PathBuf) -> Result<bool, ()> {
    let mut expiry_string = String::new();
    let mut f = fs::File::open(path).ok().ok_or(())?;
    f.read_to_string(&mut expiry_string).ok().ok_or(())?;
    let expiry_date = chrono::DateTime::parse_from_rfc2822(&expiry_string).ok().ok_or(())?;
    let result: Result<bool, ()> = Ok(expiry_date < now());
    result
}
