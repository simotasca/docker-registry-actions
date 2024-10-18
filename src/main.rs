mod config;
mod http;
mod prelude;
mod compose;

use crate::config::Config;
use http_tokio::utils::accept_connection;
use tokio::{net::TcpListener, task};

#[tokio::main]
async fn main() {
    Config::init().await;
    
    if Config::global().test_mode {
        return println!("the configuration is fine!");
    }

    let addr = Config::global().server.address();
    let server = TcpListener::bind(&addr).await
        .unwrap_or_else(|err_msg| panic!("could not start server at {addr}: {}", err_msg));
    println!("server listening on {addr}");
    loop {
        if let Ok((req, res)) = accept_connection(&server).await {
            task::spawn(async { http::handle_connection(req, res).await });
        }
    }
}
