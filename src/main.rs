mod cli;
mod compose;
mod http;
mod prelude;

use crate::prelude::*;
use cli::Config;
use http::handle_connection;
use http_tokio::utils::accept_connection;
use tokio::{net::TcpListener, task};

#[tokio::main]
async fn main() -> ! {
    Config::init();
    let addr = Config::global().server_address();
    let server = TcpListener::bind(&addr).await.expect(&f!("could not start server at {addr}"));
    println!("server listening on {addr}");
    loop {
        if let Ok((req, res)) = accept_connection(&server).await {
            task::spawn(async { handle_connection(req, res).await });
        }
    }
}
