use result::*;
use tokio::net::TcpListener;

use crate::{request::Request, Response};
use std::time::SystemTime;

pub async fn accept_connection(server: &TcpListener) -> Result<(Request, Response)> {
    let (mut stream, addr) = server.accept().await
        .map_err(|err| { Error::from(format!("couldn't get client: {err}")) })
        .trace()?;
    let req = Request::parse((&mut stream, addr)).await.trace()?;
    let res = Response::new(stream);
    Ok((req, res))
}

/// [MDN: Access-Control-Allow-Origin](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Allow-Origin)
pub fn cors_allow_origin(req: &Request, res: &mut Response, allowed_origins: &[&str]) {
    if let Some(origin) = &req.header("Origin") {
        if allowed_origins.contains(&(origin as &str)) {
            res.set_header("Access-Control-Allow-Origin", origin);
            res.set_header("Vary", "Origin"); // for caching
        }
    }
}

pub fn format_request(req: &Request) -> String {
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let curr_date = format!(
        "{}-{:02}-{:02} {:02}:{:02}:{:02}",
        1970 + current_time / 31536000,
        (current_time % 31536000) / 2592000,
        (current_time % 2592000) / 86400,
        (current_time % 86400) / 3600,
        (current_time % 3600) / 60,
        current_time % 60
    );

    let method = format!("[\x1b[96;1m{}\x1b[0m]", req.method);
    format!("\x1b[2m{}\x1b[0m {: >20} {}", curr_date, method, req.path)
}

pub fn print_request(req: &Request) {
    println!("{}", format_request(req));
}
