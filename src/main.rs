use serde::Deserialize;

use actix_files::{NamedFile, Files};
use actix_web::{web::{self, Form}, App, HttpServer, HttpRequest, HttpResponse, Responder, Result};
use actix_multipart::{Multipart, Field};
use actix_http::error::{ErrorPreconditionFailed, ErrorInsufficientStorage};
use futures::{TryStreamExt, StreamExt};

use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::path::PathBuf;
use std::str::FromStr;

use fs_extra;

mod store;
mod delete_worker;
mod time;
mod config;
mod load;
mod fs;
mod expiring_file;

async fn form_response(payload: Multipart) -> impl Responder {
    match parse_form(payload).await {
        Ok(response) => response,
        Err(e) => e.into()
    }
}

pub struct UploadConfig {
    pub days: u32,
    pub hours: u32,
    pub minutes: u32,
    pub download_limit: Option<u32>,
    pub password_hash: Option<u64>
}

async fn parse_form(mut payload: Multipart) -> Result<HttpResponse> {
    if fs_extra::dir::get_size(PathBuf::from(config::STORAGE_PATH)).ok().ok_or(())? as usize + config::MAX_UPLOAD_SIZE > config::MAX_STORAGE_CAPACITY {
        return Err(ErrorInsufficientStorage("The server has reached capacity"));
    }
    // expecting this order: days, hours, minutes, download limit, password, files
    let days: u32 = parse_next_field(&mut payload).await?;
    let hours: u32 = parse_next_field(&mut payload).await?;
    let minutes: u32 = parse_next_field(&mut payload).await?;
    
    if days == 0 && hours == 0 && minutes == 0 {
        return Err(ErrorPreconditionFailed("File must live at least 1 minute"));
    }

    if minutes > config::MAX_MINUTES || hours > config::MAX_HOURS || days > config::MAX_DAYS {
        return Err(ErrorPreconditionFailed(
                format!("Maximum allowed time is {} days, {} hours and {} minutes.",
                        config::MAX_DAYS, config::MAX_HOURS, config::MAX_MINUTES)));
    }
    
    let download_limit: u32 = parse_next_field(&mut payload).await?;

    if download_limit > config::MAX_DOWNLOAD_LIMIT {
        return Err(ErrorPreconditionFailed(
                format!("Download limit can be at most {}.", config::MAX_DOWNLOAD_LIMIT)));
    }

    let password: String = parse_next_field(&mut payload).await.unwrap_or(String::new());

    let upload_config = UploadConfig{
        days: days,
        hours: hours,
        minutes: minutes,
        download_limit: if download_limit != 0 {Some(download_limit)} else {None},
        password_hash: if password.len() != 0 {Some(hash_string(password))} else {None}
    };

    store::store_files(upload_config, payload).await
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

async fn download(req: HttpRequest) -> Result<impl Responder> {
    let name = String::from(&req.match_info()[0]);
    Ok(load::load_file(name, None).await?)
}

#[derive(Deserialize)]
struct DownloadForm {
    name: String,
    password: String
}

async fn download_form(form: Form<DownloadForm>) -> Result<impl Responder> {
    let form = form.into_inner();
    let password = if form.password.len() == 0 {
        None
    } else {
        Some(hash_string(form.password))
    };

    Ok(load::load_file(form.name, password).await?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    delete_worker::spawn();

    HttpServer::new(|| {
        App::new()
            .route("/{name}", web::get().to(download))
            .route("/{name}", web::post().to(download_form))
            .route("/", web::post().to(form_response))
            .route("/", web::get().to(index))
            .service(Files::new("www", "./www"))
    })
    .bind(format!("127.0.0.1:{}", config::PORT))?
    .run()
    .await
}
