use crate::prelude::*;
use clap::Parser;
use core::panic;
use docker_compose_types::Compose;
use serde::Deserialize;
use serde_yaml::Deserializer;
use std::{collections::HashMap, fs::read_to_string, path::Path};
use tokio::sync::{OnceCell, SetError};

#[derive(Parser)]
#[command(about, long_about = None)]
/// Docker Registry Actions
struct Cli {
    /// The watched services
    #[arg(required = true)]
    services: Vec<String>,
    /// Sets a custom compose file
    #[arg(short,long="compose_file",value_name = "compose_file",default_value = "./compose.yml")]
    compose_path: String,
    /// http server host
    #[arg(long,value_name = "http_host",default_value="127.0.0.1")]
    host: String,
    /// http server port
    #[arg(short,long,value_name = "http_port",default_value="4483",value_parser=clap::value_parser!(u16).range(1024..))]
    port: u16,
    /// Custom authorization header.
    /// The regitry must be configured to add it in a Authentication header set to "Bearer {token}"
    #[arg(short,long,value_name = "auth_token")]
    token: Option<String>
}

static CONFIG: OnceCell<Config> = OnceCell::const_new();

pub struct Config {
    cli: Cli,
    /// image to service mappings
    itos: HashMap<String, String>,
    /// parsed compose spec
    pub compose: Compose,
    pub compose_path: String,
    pub compose_dir: String
}

impl Config {
    pub fn global() -> &'static Self {
        CONFIG.get().expect("CLI configuration is not inizialized")
    }

    pub fn init() {
        if let Err(e) = CONFIG.set(Cli::parse().into()) {
            match e {
                SetError::AlreadyInitializedError(_) => panic!("config is already initialized"),
                SetError::InitializingError(_) => panic!("concurrent inizialization of config occurred")
            }
        }
    }

    pub fn server_address(&self) -> String { f!("{}:{}", self.cli.host, self.cli.port) }

    pub fn access_token(&self) -> Option<String> { self.cli.token.to_owned() }

    pub fn image_to_service(&self, service: &str) -> Option<String> {
        self.itos.get(service).map(|s| s.to_string())
    }
}

impl From<Cli> for Config {
    fn from(cli: Cli) -> Self {
        let compose = parse_compose(&cli.compose_path);
        let compose_path = cli.compose_path.clone();
        let compose_dir = Path::new(&compose_path).parent().unwrap().to_string_lossy().into_owned();

        let mut itos: HashMap<String, String> = HashMap::new();
        for service_name in cli.services.iter() {
            match compose.services.0.get(service_name) {
                Some(Some(service)) => match &service.image {
                    Some(image) => itos.insert(image.into(), service_name.into()),
                    None => panic!("service '{service_name}' has no 'image' attribute"),
                },
                _ => panic!("service '{service_name}' not found in compose"),
            };
        }

        Self { cli, compose, compose_path, compose_dir, itos }
    }
}

fn parse_compose(s: &str) -> Compose {
    let path = Path::new(s);
    let str_path = path.to_str().unwrap();
    if !path.exists() {
        panic!("compose file not found at {str_path}");
    }
    let content = match read_to_string(&path) {
        Ok(content) => content,
        Err(_) => panic!("failed to read the compose file {str_path}"),
    };
    Compose::deserialize(Deserializer::from_str(&content))
        .expect(&f!("failed to parse the compose file {str_path}"))
}
