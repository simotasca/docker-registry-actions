use crate::{request::Request, Response};
use std::time::SystemTime;
use tokio::net::TcpListener;

pub async fn accept_connection(server: &TcpListener) -> Result<(Request, Response), String> {
    let (mut stream, addr) = server.accept().await.map_err(|err| format!("couldn't get client: {err}"))?;
    let req = Request::parse((&mut stream, addr)).await.map_err(|err| format!("{err}"))?;
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

pub fn print_request(req: &Request) {
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
    println!("\x1b[2m{}\x1b[0m {: <19} {}", curr_date, method, req.path);
}
