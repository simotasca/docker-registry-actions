use crate::{compose::ComposeCmd, config::Config, prelude::*};
use anyhow::Context;
use http_tokio::{utils::print_request, Request, Response, StatusCode};
use serde::Deserialize;
use std::collections::HashMap;

pub async fn handle_connection(req: Request, mut res: Response) {
    print_request(&req);

    if !authenticate(&req) {
        unauthorized(&mut res).await
    } else {
        res.send_empty().await;
        if let Err(err) = handle_registry_events(&req).await {
            eprintln!("{:?}", err);
            server_error(&mut res).await;
        }
    }
}

type ComposePath = String;
type ServiceName = String;

async fn handle_registry_events(req: &Request) -> Result<()> {
    let body: RegistryWebhookRequest =
        serde_json::from_str(&req.body).context("failed to parse registry request")?;

    eprintln!("REQUEST {:?}", body);

    let mut updated_compose = HashMap::<ComposePath, Vec<ServiceName>>::new();
    let mut possibly_dangling_images: Vec<String> = vec![];
    for event in body.events.iter().filter(|e| e.action.eq("push")) {
        let pushed_image = f!("{}/{}", event.request.host, event.target.repository);
        eprintln!("PUSHED IMAGE {}", pushed_image);
        if Config::global().remove_dangling {
            possibly_dangling_images.push(pushed_image.clone());
        }
        for listener in Config::global().listeners.iter() {
            let listener = listener.1;
            eprintln!("checking listener for compose: {}", listener.compose.path);
            eprintln!("services listening: {:?}", listener.itos);
            let service = match listener.itos.get(&pushed_image) {
                Some(s) => s,
                None => continue,
            };
            eprintln!("FOUND SERVICE: {:?}", listener.watch_services);
            updated_compose
                .entry((&listener.compose.path).into())
                .or_insert(Vec::new())
                .push(service.into());
        }
    }
    if updated_compose.is_empty() {
        return Ok(());
    }

    for (compose_path, services) in updated_compose.iter() {
        let docker_compose = ComposeCmd::new(compose_path);
        println!(
            "- detected services push: [{}] for compose '{}'",
            services.join(", "),
            compose_path
        );
        docker_compose.pull_services(&services).await?;
        println!("- services pulled");
        docker_compose.restart_services(&services).await?;
        println!("- services restarted");
    }

    if Config::global().remove_dangling {
        for image in possibly_dangling_images {
            ComposeCmd::clean_dangling(&image).await?;
        }
    }

    Ok(())
}

fn authenticate(req: &Request) -> bool {
    let auth = match &Config::global().server.auth_token {
        None => return true,
        Some(token) => f!("Bearer {token}"),
    };
    req.header("Authorization")
        .map_or(false, |auth_header| auth_header.eq(&auth))
}

// default responses
async fn server_error(res: &mut Response) {
    res.status(StatusCode::InternalServerError)
        .send("500 Internal server error")
        .await;
}

async fn unauthorized(res: &mut Response) {
    res.status(StatusCode::Unauthorized)
        .send("401 Unauthorized")
        .await;
}

// body
#[derive(Deserialize, Debug)]
#[allow(unused)]
struct RegistryWebhookRequest {
    events: Vec<RegistryEvent>,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct RegistryEvent {
    id: String,
    timestamp: String,
    action: String,
    target: RegistryEventTarget,
    request: RegistryEventRequest,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct RegistryEventTarget {
    repository: String,
    tag: String,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct RegistryEventRequest {
    id: String,
    addr: String,
    host: String,
    method: String,
    useragent: String,
}
