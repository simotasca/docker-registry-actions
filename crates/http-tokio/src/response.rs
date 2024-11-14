use crate::{content_type::ContentType, status_code::StatusCode};
use std::result::Result as StdResult;
use std::{collections::HashMap, ffi::OsStr, future::Future, path::Path};
use thiserror::Error;
use tokio::io::AsyncBufReadExt;
use tokio::{
    fs::File,
    io::{AsyncWriteExt, BufReader},
    net::TcpStream,
};

#[derive(Error, Debug)]
pub enum ResponseError {
    #[error("could not write to TcpStream")]
    Write(#[from] tokio::io::Error),
    #[error("could not flush to TcpStream")]
    Flush(tokio::io::Error),
    #[error("could not send response: {0}")]
    Sendable(String),
}

impl ResponseError {
    pub fn sendable<E>(error: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        ResponseError::Sendable(format!("{error}"))
    }
}

type Result<T> = StdResult<T, ResponseError>;

pub struct Response {
    status: StatusCode,
    headers: HashMap<String, String>,
    stream: TcpStream,
    sent: bool,
}

// constructor
impl Response {
    pub fn new(stream: TcpStream) -> Response {
        Self {
            status: StatusCode::Ok,
            headers: HashMap::new(),
            stream,
            sent: false,
        }
    }
}

// public methods
impl Response {
    pub fn set_header(&mut self, k: &str, v: &str) {
        self.headers.insert(k.into(), v.into());
    }

    /// Sends without body
    pub async fn try_send_empty(&mut self) -> Result<()> {
        self.headers.insert("Content-Length".to_owned(), 0.to_string());
        self.write(self.fmt_head()).await?;
        self.flush().await?;
        Ok(())
    }

    /// Sends without the body with error fallback to status code 500
    pub async fn send_empty(&mut self) {
        self.try_send_empty().await.unwrap_or_else(|_| {
            let _ = self.status(StatusCode::InternalServerError).try_send("Internal server error");
        });
    }

    /// Sends the body
    pub async fn try_send<T: Sendable>(&mut self, body: T) -> Result<()> {
        body.prepare(self);
        if !self.headers.contains_key("Content-Type") {
            self.content_type(ContentType::TextPlain);
        }
        if !self.headers.contains_key("Content-Length") {
            self.headers.insert("Content-Length".to_owned(), body.content_length());
        }
        self.write(self.fmt_head()).await?;
        body.write(&mut self.stream).await?;
        self.flush().await?;
        Ok(())
    }

    /// Sends the body and in case of failure computes the closure
    pub async fn send_or<T: Sendable, F: FnOnce(&mut Self)>(&mut self, body: T, op: F) {
        self.try_send(body).await.unwrap_or_else(|_| op(self));
    }

    /// Sends the body with error fallback to status code 500
    pub async fn send<T: Sendable>(&mut self, body: T) {
        self.send_or(body, |res| {
            let _ = res.status(StatusCode::InternalServerError).try_send("Internal server error");
        })
        .await;
    }

    /// Sets the content type
    pub fn content_type(&mut self, c_type: ContentType) -> &mut Self {
        self.raw_content_type(c_type.to_string().as_str());
        self
    }

    /// Sets the content type without enum
    pub fn raw_content_type(&mut self, c_type: &str) -> &mut Self {
        self.headers.insert("Content-Type".to_owned(), c_type.to_owned());
        self
    }

    /// Sets the status code
    pub fn status(&mut self, status_code: StatusCode) -> &mut Self {
        self.status = status_code;
        self
    }

    /// Returns whether the stream has been flushed
    pub fn sent(&self) -> bool {
        self.sent
    }
}

// private methods
impl Response {
    async fn write(&mut self, res: String) -> Result<()> {
        self.stream.write(res.as_bytes()).await?;
        Ok(())
    }

    async fn flush(&mut self) -> Result<()> {
        self.stream.flush().await.map_err(|e| ResponseError::Flush(e))?;
        self.sent = true;
        Ok(())
    }

    fn fmt_head(&self) -> String {
        let (status_code, status) = self.status.as_tuple();

        let mut string_headers = "".to_string();
        for (key, val) in &self.headers {
            string_headers.push_str(format!("\r\n{key}: {val}").as_str());
        }

        format!("HTTP/1.1 {status_code} {status}{string_headers}\r\n\r\n")
    }
}

pub trait Sendable {
    /// method executed before writing the headers
    fn prepare(&self, res: &mut Response);
    /// method executed after writing the headers
    fn write(&self, stream: &mut TcpStream) -> impl Future<Output = Result<()>>;
    /// used to determine the content length header
    fn content_length(&self) -> String;
}

impl Sendable for String {
    fn prepare(&self, _: &mut Response) {}
    async fn write(&self, stream: &mut TcpStream) -> Result<()> {
        let mut bytes = self.as_bytes();
        while !bytes.is_empty() {
            let written = stream.write(bytes).await.map_err(|err| ResponseError::Sendable(format!("{err}")))?;
            bytes = &bytes[written..];
        }
        Ok(())
    }
    fn content_length(&self) -> String {
        self.len().to_string()
    }
}

impl Sendable for &str {
    fn prepare(&self, _: &mut Response) {}
    async fn write(&self, stream: &mut TcpStream) -> Result<()> {
        let mut bytes = self.as_bytes();
        while !bytes.is_empty() {
            let written = stream.write(bytes).await.map_err(|err| ResponseError::sendable(err))?;
            bytes = &bytes[written..];
        }
        Ok(())
    }
    fn content_length(&self) -> String {
        self.len().to_string()
    }
}

impl Sendable for &Path {
    fn prepare(&self, res: &mut Response) {
        let ext = self.extension().unwrap_or(OsStr::new("")).to_str().unwrap_or("");
        res.content_type(ContentType::from_ext(ext));
    }

    async fn write(&self, stream: &mut TcpStream) -> Result<()> {
        let file = File::open(self)
            .await
            .map_err(|err| ResponseError::Sendable(format!("could not open file {}: {}", self.display(), err)))?;
        let mut reader = BufReader::new(file);
        let mut buf = Vec::<u8>::new();
        while let Ok(read) = reader.read_until(b"\n"[0], &mut buf).await {
            if read == 0 {
                break;
            }
            stream.write_all(&buf).await.map_err(|err| ResponseError::sendable(err))?;
            // buf = Vec::<u8>::new();
            buf.clear();
        }
        Ok(())
    }

    fn content_length(&self) -> String {
        match std::fs::metadata(self) {
            Ok(metadata) => metadata.len().to_string(),
            Err(_) => "0".into(),
        }
    }
}
