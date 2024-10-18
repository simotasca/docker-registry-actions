use crate::prelude::*;
use std::process::Stdio;
use tokio::process::Command;

pub struct ComposeCmd {
    compose_path: String
}

impl ComposeCmd {
    pub fn new(compose_path: &str) -> Self {
        Self { compose_path: compose_path.into() }
    }

    pub async fn pull_services(&self, services: &Vec<String>) -> Result<()> {
        self.compose_cmd()
            .args(&["pull", &services.join(" ")])
            .output()
            .await
            .map_err(|err| Error::new(&format!("failed to pull new docker images: {err}")))
            .trace()?;
        Ok(())
    }
    
    pub async fn restart_services(&self, services: &Vec<String>) -> Result<()> {
        self.compose_cmd()
            .args(&["up", &services.join(" "), "-d"])
            .output()
            .await
            .map_err(|err| Error::new(&format!("failed to pull new docker images: {err}")))
            .trace()?;
        Ok(())
    }
    
    pub fn compose_cmd(&self) -> Command {
        let mut cmd = Command::new("docker");
        cmd.stdout(Stdio::null()).args(&["compose", "-f", &self.compose_path]);
        cmd
    }
}

