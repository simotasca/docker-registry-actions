mod compose;
mod config;
mod http;
mod prelude;

use crate::config::Config;
use anyhow::Context;
use http_tokio::utils::accept_connection;
pub use prelude::*;
use tokio::{net::TcpListener, task};

#[tokio::main]
async fn main() -> Result<()> {
    Config::init().await;

    if Config::global().test_mode {
        println!("the configuration is fine!");
        return Ok(());
    }

    let addr = Config::global().server.address();
    let server = TcpListener::bind(&addr)
        .await
        .context(f!("could not start server at {addr}"))?;
    println!("server listening on {addr}");

    loop {
        if let Ok((req, res)) = accept_connection(&server).await {
            task::spawn(async { http::handle_connection(req, res).await });
        }
    }
}
