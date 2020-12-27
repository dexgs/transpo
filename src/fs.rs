use std::path::PathBuf;
use std::str::FromStr;
use std::string::ToString;
use actix_web::web;
use std::fs;
use std::io::{Read, Write};

pub async fn read_file<T>(path: PathBuf) -> Option<T> 
where T: FromStr + std::marker::Send + 'static
{
    web::block(move || {
        fs::File::open(path).ok().and_then(|mut f| {
            let mut string = String::new();
            f.read_to_string(&mut string).ok()?;
            string.parse().ok()
        }).ok_or(())
    }).await.ok()
}

pub async fn write_file<T>(path: PathBuf, t: T) -> bool 
where T: ToString + std::marker::Send + 'static
{
    web::block(move || {
        let mut f = fs::File::create(path)?;
        f.write_all(t.to_string().as_bytes())
    }).await.is_ok()
}

pub async fn delete_file(path: PathBuf) -> bool {
    web::block(move || {
        fs::remove_dir_all(path)
    }).await.is_ok()
}
