use crate::prelude::*;
use std::process::Command as SyncCommand;
use std::process::Stdio;
use tokio::process::Command;

pub struct ComposeCmd {
    compose_path: String,
}

impl ComposeCmd {
    pub fn new(compose_path: &str) -> Self {
        Self {
            compose_path: compose_path.into(),
        }
    }

    pub fn get_config(&self) -> Result<String> {
        let config = SyncCommand::new("docker")
            .args(self.compose_args())
            .args(&["config"])
            .output()
            .map_err(|err| Error::new(&format!("failed to load docker configuration: {err}")))
            .trace()?
            .stdout;
        let config_str = String::from_utf8_lossy(&config);
        Ok(config_str.into_owned())
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

    pub async fn clean_dangling(image_name: &str) -> Result<()> {
        let mut image_ls_cmd = Command::new("docker");
        image_ls_cmd.args(&[
            "images",
            &f!("-f 'reference={image_name}'"),
            "-f 'dangling=true'",
            "-q",
        ]);
        let out = image_ls_cmd
            .output()
            .await
            .map_err(|err| Error::new(&format!("failed to list dangling docker images: {err}")))
            .trace()?;
        let image_ids = String::from_utf8_lossy(&out.stdout);
        if !image_ids.is_empty() {
            let mut delete_image_cmd = Command::new("docker");
            delete_image_cmd.args(&["rmi", image_ids.trim()]);
            delete_image_cmd
                .output()
                .await
                .map_err(|err| Error::new(&format!("failed to remove dangling images: {err}")))
                .trace()?;
        }
        Ok(())
    }

    pub fn compose_cmd(&self) -> Command {
        let mut cmd = Command::new("docker");
        cmd.stdout(Stdio::null()).args(self.compose_args());
        cmd
    }

    pub fn compose_args(&self) -> [&str; 3] {
        ["compose", "-f", &self.compose_path]
    }
}
