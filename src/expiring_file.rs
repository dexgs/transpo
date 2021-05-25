use actix_files::NamedFile;
use actix_web::{web::Bytes, Responder, HttpRequest, HttpResponse, body::{BodySize, Body}, dev::MessageBody, Error};
use futures::future::ready;
use futures::future::Ready;

use std::pin::Pin;
use std::task::{Context, Poll};
use std::borrow::BorrowMut;
use std::path::PathBuf;


// Deletes directory containing file after it
// has been completely sent to a client.
pub struct ExpiringFile {
    file: NamedFile,
}

impl ExpiringFile {
    pub fn new(file: NamedFile) -> Self {
        Self {
            file: file
        }
    }

    pub fn into_response(self, req: &HttpRequest) -> Result<HttpResponse, Error> {
        // file is at `storage/URL/file` and we want to remove `storage/URL/`
        let path = PathBuf::from(self.file.path().parent().unwrap());
        let mut resp = self.file.into_response(req)?;
        let stream = resp.take_body();
        let wrapper = StreamWrapper::new(path, stream);
        let body = Body::from_message(wrapper);
        Ok(resp.set_body(body))
    }
}

impl Responder for ExpiringFile {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        ready(self.into_response(req))
    }
}

struct StreamWrapper<S> 
where S: MessageBody
{
    path: PathBuf,
    stream: S,
}

impl<S> StreamWrapper<S>
where S: MessageBody
{
    pub fn new(path: PathBuf, stream: S) -> Self {
        Self {
            path: path,
            stream: stream
        }
    }
}

impl<S> MessageBody for StreamWrapper<S>
where S: MessageBody + Unpin
{
    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Option<Result<Bytes, Error>>> {
        let stream = Pin::new((self.stream).borrow_mut());
        let progress = stream.poll_next(cx);
        match &progress {
            Poll::Ready(chunk) => {
                if chunk.is_none() || chunk.as_ref().unwrap().is_err() {
                    // so this method doesn't block
                    let p = PathBuf::from(&self.path);
                    std::thread::spawn(|| std::fs::remove_dir_all(p));
                }
            }
            _ => {}
        }
        progress
    }

    fn size(&self) -> BodySize {
        self.stream.size()
    }
}

impl<S: MessageBody> Drop for StreamWrapper<S> {
    fn drop(&mut self) {
        let p = PathBuf::from(&self.path);
        std::thread::spawn(|| std::fs::remove_dir_all(p));
    }
}
