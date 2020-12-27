use super::UploadConfig;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use actix_web::{web, HttpResponse, Result};
use actix_multipart::Multipart;
use actix_http::error::ErrorPayloadTooLarge;

use futures::{TryStreamExt, StreamExt};

use std::path::PathBuf;
use std::fs;
use std::io::Write;
use std::process::{Command, ExitStatus};

use super::time;
use super::config;

// Based on this example:
// https://github.com/actix/examples/blob/master/multipart/src/main.rs

pub async fn store_files(upload_config: UploadConfig, mut payload: Multipart) -> Result<HttpResponse> {
    let storage_dir = PathBuf::from(config::STORAGE_PATH);
    let s = storage_dir.clone();
    let name = web::block(|| gen_name(s)).await.unwrap();
    let dir = storage_dir.join(&name).join("tmp");
    let d = dir.clone();
    web::block(move || fs::create_dir_all(d)).await?;
    
    let mut num_files: usize = 0;
    let mut total_size: usize = 0;

    while let Ok(Some(mut file)) = payload.try_next().await {
        let file_name = sanitize_filename::sanitize(
            file.content_disposition()
            .ok_or(())?
            .get_filename()
            .ok_or(())?);
        num_files += 1;
        
        let d = dir.clone();
        let mut f = web::block(move || fs::File::create(d.join(file_name))).await?;

        while let Some(Ok(data)) = file.next().await {
            total_size += data.len();
            if total_size > config::MAX_UPLOAD_SIZE {
                drop(web::block(move || fs::remove_dir_all(dir.parent().unwrap())).await);
                return Err(ErrorPayloadTooLarge(config::MAX_UPLOAD_SIZE));
            }
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }
    
    let d = dir.clone();
    if num_files > 1 || total_size >= config::COMPRESSION_THRESHOLD {
        let n = name.clone();
        web::block(move || {
            zip(&d, n)
            //fs::remove_dir_all(d)
        }).await?;
    } else {
        web::block(move || fs::rename(&d, d.parent().unwrap().join("file")))
            .await?;
    }

    let dir = PathBuf::from(dir.parent().unwrap());

    let d = dir.clone();
    web::block(move || store_metadata(d, upload_config)).await?;

    Ok(HttpResponse::Ok().body(name))
}

fn store_metadata(dir: PathBuf, c: UploadConfig) -> std::io::Result<()> {
    if let Some(password_hash) = c.password_hash {
        let mut password_hash_file = fs::File::create(dir.join("password_hash"))?;
        password_hash_file.write_all(format!("{}", password_hash).as_bytes())?;
    }
    if let Some(download_limit) = c.download_limit {
        let mut remaining_downloads = fs::File::create(dir.join("remaining_downloads"))?;
        remaining_downloads.write_all(format!("{}", download_limit).as_bytes())?;
    }
    let mut expiry_date_file = fs::File::create(dir.join("expiry_date"))?;
    expiry_date_file.write_all(time::expiry(c.days, c.hours, c.minutes).to_rfc2822().as_bytes())?;
    Ok(())
}

fn gen_name(storage_dir: PathBuf) -> std::io::Result<String> {
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

fn zip(tmp_dir: &PathBuf, name: String) -> std::io::Result<ExitStatus> {
    let file_dir = tmp_dir.parent().unwrap().join("file");
    fs::create_dir_all(&file_dir)?;
    // *nix dependent. may add windows support in future
    Command::new("zip")
        .arg("-rj")
        .arg(file_dir.join(format!("{}_{}.zip", config::ZIP_PREFIX, name)))
        .arg(tmp_dir)
        .spawn()?
        .wait()
}
