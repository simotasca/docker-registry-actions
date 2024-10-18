use std::collections::HashMap;
use crate::{config::Config, prelude::*, compose::ComposeCmd};
use http_tokio::{utils::print_request, Request, Response, StatusCode};
use serde::Deserialize;

pub async fn handle_connection(req: Request, mut res: Response) {
    print_request(&req);

    if !authenticate(&req) { 
        unauthorized(&mut res).await
    } else {
        res.send_empty().await;
        if let Err(err) = handle_registry_events(&req).await {
            err.print();
            server_error(&mut res).await;
        }
    }
    
}

async fn handle_registry_events(req: &Request) -> Result<()> {
    // TODO: una immagine può essere trovata in più compose e in più servizi del medesimo compose
    // fare quanto sotto per ogni compose configurato
    // semplicemente spostare questa funzione in un for
    let body: RegistryWebhookRequest = serde_json::from_str(&req.body).trace()?;
    // println!("{:#?}", body);


    let mut updated_compose = HashMap::<String, Vec<String>>::new();
    for event in body.events.iter().filter(|e| e.action.eq("push")) {
        let pushed_image = f!("{}/{}", event.request.host, event.target.repository);
        for listener in Config::global().listeners.iter() {
            let listener = listener.1;
            let service = match listener.itos.get(&pushed_image) { 
                Some(s) => s, 
                None => continue
            };
            updated_compose
                .entry((&listener.compose.path).into())
                .or_insert(Vec::new())
                .push(service.into());
        }
    }
    if updated_compose.is_empty() { return Ok(()) }

    for (compose_path, services) in updated_compose.iter() {
        let docker_compose = ComposeCmd::new(compose_path);
        println!("- detected services push: [{}] for compose {}", services.join(", "), compose_path);
        docker_compose.pull_services(&services).await.trace()?;
        println!("- services pulled");
        docker_compose.restart_services(&services).await.trace()?;
        println!("- services restarted");
    }
    
    Ok(())
}

fn authenticate(req: &Request) -> bool {
    let auth = match &Config::global().server.auth_token {
        None => return true,
        Some(token) => f!("Bearer {token}")
    };
    req.header("Authorization").map_or(false, |auth_header| auth_header.eq(&auth))
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