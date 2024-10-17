use crate::{cli::Config, prelude::*, compose};
use http_tokio::{utils::print_request, Request, Response, StatusCode};
use serde::Deserialize;

pub async fn handle_connection(req: Request, mut res: Response) {
    print_request(&req);
    
    if !authenticate(&req) { 
        return unauthorized(&mut res).await
    }
    
    res.send_empty().await;

    if let Err(err) = handle_registry_events(&req).await {
        err.print();
        server_error(&mut res).await;
    }
}

async fn handle_registry_events(req: &Request) -> Result<()> {
    let body: RegistryWebhookRequest = serde_json::from_str(&req.body).trace()?;

    let mut updated_services: Vec<String> = vec![];
    for evt in body.events.iter().filter(|e| e.action.eq("push")) {
        let full_image_name = f!("{}/{}", evt.request.host, evt.target.repository);
        if let Some(service) = Config::global().image_to_service(&full_image_name) {
            updated_services.push(service);
        }
    }
    if updated_services.is_empty() { return Ok(()) }

    println!("- detected services push: {} ", updated_services.join(", "));
    compose::pull_services(&updated_services).await.trace()?;
    println!("- services pulled");
    compose::restart_services(&updated_services).await.trace()?;
    println!("- services restarted");
    
    Ok(())
}

fn authenticate(req: &Request) -> bool {
    let auth = match Config::global().access_token() {
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