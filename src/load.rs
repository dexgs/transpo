use actix_files::NamedFile;
use actix_web::{HttpRequest, Result, Responder, Error};
use actix_web::http::header::ContentDisposition;
use actix_web::http::HeaderValue;
use actix_http::error::{ErrorUnauthorized, ErrorNotFound};

use super::config;
use super::time;
use super::fs;
use super::expiring_file::ExpiringFile;

use std::path::PathBuf;

pub enum FileResponse {
    Named(NamedFile),
    Expiring(ExpiringFile)
}

impl Responder for FileResponse {
    type Error = Error;
    type Future = <NamedFile as Responder>::Future;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        match self {
            FileResponse::Named(file) => file.respond_to(req),
            FileResponse::Expiring(file) => file.respond_to(req)
        }
    }
}

pub async fn load_file(name: String, password_hash: Option<u64>) -> Result<FileResponse> {
    let dir = PathBuf::from(config::STORAGE_PATH).join(name);

    if time::is_expired(&dir).await {
        fs::delete_file(dir).await;
        return not_found();
    }

    let file_password: Option<u64> = fs::read_file(dir.join("password_hash")).await;

    let remaining_downloads: Option<u32> = fs::read_file(dir.join("remaining_downloads")).await;
    if let Some(remaining_downloads) = remaining_downloads {
        // downloads may be exhausted, but file can still be up if it's currently being downloaded
        if remaining_downloads == 0 {
            return not_found();
        } else if file_password == password_hash {
            fs::write_file(dir.join("remaining_downloads"), remaining_downloads - 1).await;
        } else {
            return password_protected();
        }
    } else if file_password != password_hash {
        return password_protected();
    }
    // everything succeeded

    let file = NamedFile::open(dir.join("upload"))?;
    let file_name = if let Some(name) = fs::read_file(dir.join("file_name")).await {
        name
    } else {
        String::new()
    };
    // must be set, since default is inline for text, image and video
    let file = file.set_content_disposition(ContentDisposition::from_raw(
        &HeaderValue::from_str(&format!("attachment; filename=\"{}\"", file_name))?)?);
    if remaining_downloads == Some(1) {
        Ok(FileResponse::Expiring(ExpiringFile::new(file)))
    } else {
        Ok(FileResponse::Named(file))
    }
}

fn not_found() -> Result<FileResponse> {
    Err(ErrorNotFound("Not found"))
}

fn password_protected() -> Result<FileResponse> {
    Err(ErrorUnauthorized("Incorrect Password"))
}
