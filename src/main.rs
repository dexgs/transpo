use actix_files::{NamedFile, Files};
use actix_web::{get, web::{self, Bytes}, App, HttpServer, HttpRequest, HttpResponse, Responder, Result};
use actix_multipart::{Multipart, Field};
use actix_http::error::ErrorPreconditionFailed;
use futures::{TryStreamExt, StreamExt};

use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::path::PathBuf;
use std::str::FromStr;

mod store;
mod delete_worker;

// 500MB
const MAX_UPLOAD_SIZE: usize = 500000000;
// 5GB
const MAX_STORAGE_CAPACITY: usize = 5000000000;

const STORAGE_DIR: &'static str = "./storage";


async fn form_response(payload: Multipart) -> impl Responder {
    if let Ok(response) = parse_form(payload).await {
        return response;
    }
    HttpResponse::Ok().body("haha")
}

pub struct UploadConfig {
    pub days: u32,
    pub hours: u32,
    pub minutes: u32,
    pub download_limit: Option<u32>,
    pub password_hash: Option<u64>
}

async fn parse_form(mut payload: Multipart) -> Result<HttpResponse> {
    // expecting this order: days, hours, minutes, download limit, password, files
    let days: u32 = parse_next_field(&mut payload).await?;
    let hours: u32 = parse_next_field(&mut payload).await?;
    let minutes: u32 = parse_next_field(&mut payload).await?;
    
    if days == 0 && hours == 0 && minutes == 0 {
        return Err(ErrorPreconditionFailed("File must live at least 1 minute"));
    }
    
    let download_limit: u32 = parse_next_field(&mut payload).await?;
    let password: String = parse_next_field(&mut payload).await.unwrap_or(String::new());


    let upload_config = UploadConfig{
        days: days,
        hours: hours,
        minutes: minutes,
        download_limit: if download_limit != 0 {Some(download_limit)} else {None},
        password_hash: if password.len() != 0 {Some(hash_string(password))} else {None}
    };
    
    let path = PathBuf::from(STORAGE_DIR);
    store::store_files(MAX_UPLOAD_SIZE, path, upload_config, payload).await
}

fn hash_string(string: String) -> u64 {
    let mut hasher = DefaultHasher::new();
    hasher.write(string.as_bytes());
    hasher.finish()
}

async fn parse_next_field<T>(payload: &mut Multipart) -> Result<T>
where T: FromStr
{
    if let Some(field) = parse_field(payload.try_next().await?).await {
        return Ok(field)
    }
    Err(ErrorPreconditionFailed("Couldn't parse value from form entry"))
}

// take only first chunk since all the non-file form entries should not be large
async fn parse_field<T>(field: Option<Field>) -> Option<T> 
where T: FromStr
{
    String::from_utf8(field?.next().await?.ok()?.to_vec()).ok()?.parse::<T>().ok()
}

async fn index(_req: HttpRequest) -> Result<NamedFile> {
    let path = PathBuf::from("./www/index.html");
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::post().to(form_response))
            .route("/", web::get().to(index))
            .service(Files::new("www", "./www"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
