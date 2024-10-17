use crate::{cli::Config, prelude::*};
use std::process::Stdio;
use tokio::process::Command;

pub async fn pull_services(services: &Vec<String>) -> Result<()> {
    compose_cmd()
        .args(&["pull", &services.join(" ")])
        .output()
        .await
        .map_err(|err| Error::new(&format!("failed to pull new docker images: {err}")))
        .trace()?;
    Ok(())
}

pub async fn restart_services(services: &Vec<String>) -> Result<()> {
    compose_cmd()
        .args(&["up", &services.join(" "), "-d"])
        .output()
        .await
        .map_err(|err| Error::new(&format!("failed to pull new docker images: {err}")))
        .trace()?;
    Ok(())
}

fn compose_cmd() -> Command {
    let mut cmd = Command::new("docker");
    cmd.current_dir(&Config::global().compose_dir)
        .stdout(Stdio::null())
        .args(&["compose", "-f", &Config::global().compose_path]);
    cmd
}
