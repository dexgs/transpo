use serde::Deserialize;
use actix::{Actor, StreamHandler};
use actix_files::{NamedFile, Files};
use actix_web::{web::{self, Form}, App, HttpServer, HttpRequest, HttpResponse, Responder, Result};
use actix_http::error::{ErrorPreconditionFailed, ErrorInsufficientStorage};
use actix_web_actors::ws;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::path::PathBuf;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use fs_extra;
mod store;
mod delete_worker;
mod time;
mod config;
mod load;
mod fs;
mod expiring_file;

// UPLOADING
#[derive(Deserialize)]
struct UploadForm {
    days: u32,
    hours: u32,
    minutes: u32,
    download_limit: u32,
    password: String,
    file_name: String,
}

async fn upload_form(_req: HttpRequest, form: Form<UploadForm>) -> Result<HttpResponse> {
    if fs_extra::dir::get_size(PathBuf::from(config::STORAGE_PATH)).ok().ok_or(())? as usize + config::MAX_UPLOAD_SIZE > config::MAX_STORAGE_CAPACITY {
        return Err(ErrorInsufficientStorage("The server has reached its storage limit."));
    }
    if form.days == 0 && form.hours == 0 && form.minutes == 0 {
        return Err(ErrorPreconditionFailed("File must live at least 1 minute."));
    }
    if form.minutes > config::MAX_MINUTES || form.hours > config::MAX_HOURS || form.days > config::MAX_DAYS {
        return Err(ErrorPreconditionFailed(
                format!("Maximum allowed time is {} days, {} hours and {} minutes.",
                        config::MAX_DAYS, config::MAX_HOURS, config::MAX_MINUTES)));
    }
    if form.download_limit > config::MAX_DOWNLOAD_LIMIT {
        return Err(ErrorPreconditionFailed(
                format!("Download limit can be at most {}.", config::MAX_DOWNLOAD_LIMIT)));
    }

    let download_limit = if form.download_limit != 0 {Some(form.download_limit)} else {None};
    let password_hash = if form.password.len() != 0 {Some(hash_string(&form.password))} else {None};

    store::write_metadata(form.days, form.hours, form.minutes, download_limit, password_hash, form.file_name.clone())
}

// DOWNLOADING
#[derive(Deserialize)]
struct DownloadForm {
    password: String
}

async fn download_form(req: HttpRequest, form: Form<DownloadForm>) -> Result<impl Responder> {
    let name = req.path().split("/").last().ok_or(())?.to_owned();
    let password = if form.password.len() == 0 {
        None
    } else {
        Some(hash_string(&form.password))
    };

    Ok(load::load_file(name, password).await?)
}

// UPLOAD STREAM
async fn upload_stream(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse> {
    let name = req.path().split("/").last().ok_or(())?.to_owned();
    let dir = PathBuf::from(config::STORAGE_PATH).join(&name);
    let file_path = dir.join("upload");
    if dir.exists() && !file_path.exists() {
        let f = web::block(move || {
            OpenOptions::new()
                .append(true)
                .create_new(true)
                .open(file_path)
        }).await?;
        return ws::start(UploadWs{f: f, num_bytes: 0}, &req, stream);
    }
    Err(ErrorPreconditionFailed("Tried to stream to invalid dir"))
}

struct UploadWs {
    num_bytes: usize,
    f: File
}

impl Actor for UploadWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for UploadWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Binary(bin)) => {
                if let Ok(b) = self.f.write(&bin) {
                    self.num_bytes += b;
                    if self.num_bytes > config::MAX_UPLOAD_SIZE {
                        ctx.close(None);
                    }
                } else {
                    ctx.close(None);
                }
            },
            _ => {},
        };
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    delete_worker::spawn();

    HttpServer::new(|| {
        App::new()
            .route("/{name}", web::get().to(download))
            .route("/{name}", web::post().to(download_form))
            .route("/", web::post().to(upload_form))
            .route("/", web::get().to(index))
            .route("/ws/{name}", web::get().to(upload_stream))
            .service(Files::new("www", "./www"))
    })
    .bind(format!("127.0.0.1:{}", config::PORT))?
    .run()
    .await
}


fn hash_string(string: &String) -> u64 {
    let mut hasher = DefaultHasher::new();
    hasher.write(string.as_bytes());
    hasher.finish()
}

async fn index(_req: HttpRequest) -> Result<NamedFile> {
    let path = PathBuf::from("./www/index.html");
    Ok(NamedFile::open(path)?)
}

async fn download(req: HttpRequest) -> Result<impl Responder> {
    let name = req.path().split("/").last().ok_or(())?.to_owned();
    let file_dir = PathBuf::from(config::STORAGE_PATH).join(&name);
    let path = if file_dir.exists() {
        if file_dir.join("password_hash").exists() {
            PathBuf::from("./www/unlock.html")
        } else {
            PathBuf::from("./www/download.html")
        }
    } else {
        PathBuf::from("./www/not-found.html")
    };
    Ok(NamedFile::open(path)?.with_header("Cache-Control", "no-store"))
}
