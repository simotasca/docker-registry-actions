use crate::{compose::ComposeCmd, prelude::*};
use clap::Parser;
use docker_compose_types::Compose;
use serde_yaml::Deserializer as YamlDeserializer;
use core::panic;
use serde::{Deserialize, Deserializer};
use std::{collections::HashMap, io::Read, path::Path};
use tokio::{fs::File, io::AsyncReadExt, sync::{OnceCell, SetError}};

// TODO: for config path better use an env variable and set it in the .bashrc via installer 

/// Docker Registry Actions
#[derive(Parser)]
#[command(about, long_about = None)]
struct Cli {
    /// path to the yaml configuration file
    #[arg(short,long="config",value_name="config_path",default_value="/etc/docker-registry-actions/config.yml",required=false)]
    config_path: String,
    /// test the configuration
    #[arg(short,long,value_name="test")]
    test: bool,
}

static CONFIG: OnceCell<Config> = OnceCell::const_new();

/// needs to be initialized once with Config::init()
#[derive(Debug,Deserialize)]
pub struct Config {
    #[serde(default="Server::default")]
    pub server: Server,
    pub listeners: HashMap<String, Listener>,
    #[serde(default="bool::default")]
    pub remove_dangling: bool,
    #[serde(skip_deserializing,default="bool::default")]
    pub test_mode: bool
}

impl Config {
    pub fn global() -> &'static Self {
        CONFIG.get().expect("CLI configuration is not inizialized")
    }
    
    /// panics if there are errors in the configuration
    pub async fn init() {
        if let Err(e) = CONFIG.set(Self::from_cli(Cli::parse()).await) {
            match e {
                SetError::AlreadyInitializedError(_) => panic!("config is already initialized"),
                SetError::InitializingError(_) => panic!("concurrent inizialization of config occurred")
            }
        }
    }
}

#[derive(Debug,Deserialize)]
pub struct Server {
    #[serde(default="Server::default_host")]
    pub host: String,
    #[serde(default="Server::default_port")]
    pub port: u16,
    pub auth_token: Option<String>
}


impl Server {
    pub fn address(&self) -> String { f!("{}:{}", self.host, self.port) }
}

#[derive(Debug,Deserialize)]
pub struct Listener {
    #[serde(rename="compose_path",deserialize_with="deserialize_compose_with_path")]
    pub compose: ComposeWithPath,
    pub watch_services: Vec<String>,
    /// Image to service mappings
    #[serde(skip_deserializing,default="HashMap::default")]
    pub itos: HashMap<String, String>
}

#[derive(Debug)]
pub struct ComposeWithPath {
    pub path: String,
    pub content: Compose,
}

impl Config {
    async fn from_cli(cli: Cli) -> Self {
        // per esser un vero rustafariano dovrei ritonare un error enum e printare da fuori...
        let mut config_file = String::new();
        File::open(&cli.config_path).await.expect("configuration file not found")
            .read_to_string(&mut config_file).await.expect("failed to read configuration file");
        let mut config = serde_yaml::from_str::<Self>(&config_file)
            .unwrap_or_else(|err_msg| panic!("invalid configuration structure: {}", err_msg));
        // validation
        if config.server.port < 1024 { panic!("invalid configuration: invalid server port {}: cannot be less than 1024", config.server.port) }
        // if config.listeners.len() == 0 { panic!("invalid configuration: listeners must contain at least one element") }
        for (name, listener) in config.listeners.iter_mut() {
            if listener.watch_services.is_empty() {
                panic!("invalid configuration: listener '{}' should have at least one watch_services defined", name)
            }
            for service_name in listener.watch_services.iter() {
                match listener.compose.content.services.0.get(service_name) {
                    Some(Some(service)) => match &service.image {
                        Some(image) => listener.itos.insert(image.into(), service_name.into()),
                        None => panic!("invalid configuration: service '{service_name}' has no 'image' attribute in compose file '{}'", &listener.compose.path),
                    },
                    _ => panic!("invalid configuration: service '{service_name}' not found in compose file '{}'", &listener.compose.path),
                };
            }
        }
        config.test_mode = cli.test;
        config
    }
}

impl Server {
    fn default() -> Self { serde_yaml::from_str::<Self>("").unwrap() }
    fn default_host() -> String { String::from("0.0.0.0") }
    fn default_port() -> u16 { 4463_u16 }
}

fn deserialize_compose_with_path<'de, D>(deserializer: D) -> std::result::Result<ComposeWithPath, D::Error> where D: Deserializer<'de> {
    let path = String::deserialize(deserializer)?;
    let content = read_compose_file(&path);
    Ok(ComposeWithPath { path, content })
}

fn read_compose_file(compose_path: &str) -> Compose {
    if !Path::new(compose_path).exists() {
        panic!("invalid configuration: compose file not found at {compose_path}");
    }
    let content = ComposeCmd::new(compose_path).get_config().expect("invalid configuration: failed to load docker config");

    // let mut content = String::new();
    // std::fs::File::open(compose_path).expect(&f!("invalid configuration: compose file not found at {compose_path}"))
    //     .read_to_string(&mut content)
    //     .unwrap_or_else(|err_msg| panic!("failed to read the compose file {compose_path}: {}", err_msg));
    
    // TODO: deserialize only when needed (eg: OnceCell)
    Compose::deserialize(YamlDeserializer::from_str(&content))
        .unwrap_or_else(|err_msg| panic!("failed to parse the compose configuration {compose_path}: {}", err_msg))
}