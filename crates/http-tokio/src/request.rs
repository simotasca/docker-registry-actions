use std::collections::HashMap;
use std::net::SocketAddr;
use std::result::Result as StdResult;
use thiserror::Error;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::net::TcpStream;

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("could not read from TcpStream")]
    Read(#[from] tokio::io::Error),
    #[error("invalid Content-Length header `{0}`")]
    ContentLength(String),
    #[error("request body is non utf-8")]
    Utf8(#[from] std::string::FromUtf8Error),
}

type Result<T> = StdResult<T, RequestError>;

// TODO: query params

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub body: String,
    pub address: SocketAddr,
}

impl Request {
    pub fn header(&self, key: &str) -> Option<String> {
        self.headers.get(key).map(|v| v.to_owned())
    }
    pub fn cookie(&self, name: &str) -> Option<&String> {
        self.cookies.get(name)
    }
    pub fn matcher(&self) -> (&str, &str) {
        (self.method.as_str(), self.path.as_str())
    }
}

impl Request {
    pub async fn parse(value: (&mut TcpStream, SocketAddr)) -> Result<Self> {
        let (stream, address) = value;
        let mut buf = String::new();
        let mut buf_reader = BufReader::new(stream);

        // parsing status line
        let (_, first_line) = read_line(&mut buf_reader, &mut buf).await?;
        let mut status = first_line.split(" ");
        let method = status.next().unwrap_or("GET").to_owned();
        let full_path = status.next().unwrap_or("/").to_owned();
        let mut full_path = full_path.split("?");

        let path = "/".to_owned() + full_path.next().unwrap_or("/").trim_end_matches("/").trim_start_matches("/");
        let _query_string = full_path.next().unwrap_or("");

        // parsing headers
        let mut headers: HashMap<String, String> = HashMap::new();
        loop {
            let (len, line) = read_line(&mut buf_reader, &mut buf).await?;
            if len <= 2 {
                break;
            }
            if let Some((k, v)) = line.split_once(": ") {
                headers.insert(k.to_owned(), v.to_owned());
            }
        }

        // parsing cookies
        let mut cookies = HashMap::<String, String>::new();
        if let Some(cookie) = headers.get("Cookie") {
            let split = cookie.split("; ");
            for cookie in split {
                let Some((k, v)) = cookie.split_once("=") else { continue };
                cookies.insert(k.to_owned(), v.to_owned());
            }
        }

        // parsing body
        let mut body: String = "".to_owned();
        if let Some(len) = headers.get("Content-Length") {
            let len = str::parse::<usize>(len).map_err(|_| RequestError::ContentLength(len.into()))?;

            let mut buf = vec![0 as u8; len];
            buf_reader.read_exact(&mut buf).await?;

            body = String::from_utf8(buf)?;
        }

        Ok(Self {
            path,
            method,
            headers,
            body,
            address,
            cookies,
        })
    }
}

async fn read_line(buf_reader: &mut BufReader<&mut TcpStream>, buf: &mut String) -> Result<(usize, String)> {
    let len = buf_reader.read_line(buf).await?;
    let parsed = String::from_utf8(buf.clone().into())?.replace("\r\n", ""); // remove line terminators
    buf.clear();
    return Ok((len, parsed));
}
