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

const NAME_LENGTH: usize = 20;

// Based on this example:
// https://github.com/actix/examples/blob/master/multipart/src/main.rs

pub async fn store_files(max_upload_size: usize, storage_dir: PathBuf, upload_config: UploadConfig, mut payload: Multipart) -> Result<HttpResponse> {
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
            if total_size > max_upload_size {
                drop(web::block(|| fs::remove_dir_all(dir)).await);
                return Err(ErrorPayloadTooLarge(max_upload_size));
            }
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }
    Ok(HttpResponse::Ok().body("haha"))
}

fn gen_name(storage_dir: PathBuf) -> std::io::Result<String> {
    let mut name = String::new();
    let mut rng = thread_rng();
    while storage_dir.join(&name).exists() {
        name = (&mut rng).sample_iter(Alphanumeric)
            .take(NAME_LENGTH)
            .map(char::from)
            .collect();
    }
    Ok(name)
}
