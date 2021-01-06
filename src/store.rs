use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use actix_web::{HttpResponse, Result};

use std::path::PathBuf;
use std::fs;
use std::io::Write;

use super::time;
use super::config;

pub fn write_metadata(days: u32, hours: u32, minutes: u32, download_limit: Option<u32>, password_hash: Option<u64>, file_name: String) -> Result<HttpResponse> {
    let storage_dir = PathBuf::from(config::STORAGE_PATH);
    let name = gen_name(&storage_dir)?;
    let dir = storage_dir.join(&name);
    std::fs::create_dir_all(&dir)?;

    if let Some(password_hash) = password_hash {
        let mut password_hash_file = fs::File::create(dir.join("password_hash"))?;
        password_hash_file.write_all(format!("{}", password_hash).as_bytes())?;
    }
    if let Some(download_limit) = download_limit {
        let mut remaining_downloads = fs::File::create(dir.join("remaining_downloads"))?;
        remaining_downloads.write_all(format!("{}", download_limit).as_bytes())?;
    }
    let mut expiry_date_file = fs::File::create(dir.join("expiry_date"))?;
    expiry_date_file.write_all(time::expiry(days, hours, minutes).to_rfc2822().as_bytes())?;
    
    if file_name.len() > 0 {
        let mut file_name_file = fs::File::create(dir.join("file_name"))?;
        file_name_file.write_all(file_name.as_bytes())?;
    }

    Ok(HttpResponse::Ok().body(name))
}

fn gen_name(storage_dir: &PathBuf) -> std::io::Result<String> {
    let mut name = String::new();
    let mut rng = thread_rng();
    while storage_dir.join(&name).exists() {
        name = (&mut rng).sample_iter(Alphanumeric)
            .take(config::NAME_LENGTH)
            .map(char::from)
            .collect();
    }
    Ok(name)
}
