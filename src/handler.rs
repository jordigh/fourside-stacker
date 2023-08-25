use std::collections::HashSet;

use crate::{ws, Client, Clients, Db, Result, Sockets};
use serde::{Deserialize, Serialize};
use std::{env, fs};
use uuid::Uuid;
use warp::{http::Response, http::StatusCode, reply::json, Reply};

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    username: String,
}

#[derive(Serialize, Debug)]
pub struct RegisterResponse {
    url: String,
}

pub async fn register_handler(
    body: RegisterRequest,
    clients: Clients,
    sockets: Sockets,
    db: Db,
) -> Result<impl Reply> {
    let username = body.username;
    let player = db.write().await.get_player(&username).await;
    let uuid = Uuid::new_v4().as_simple().to_string();
    register_client(username.clone(), player.id, uuid.clone(), clients, sockets).await;
    let protocol;
    let base_url = match env::var("STACKED_FOURSIDE_HOST") {
        Ok(val) => {
            protocol = "wss";
            val
        }
        _ => {
            protocol = "ws";
            String::from("127.0.0.1:4321")
        }
    };
    Ok(json(&RegisterResponse {
        url: format!("{protocol}://{base_url}/ws/{uuid}"),
    }))
}

async fn register_client(
    username: String,
    user_id: i32,
    uuid: String,
    clients: Clients,
    sockets: Sockets,
) {
    clients.write().await.insert(
        uuid.clone(),
        Client {
            username,
            user_id,
            sender: None,
        },
    );
    let mut sockets = sockets.write().await;
    let uuids = sockets.entry(user_id).or_insert(HashSet::new());
    (*uuids).insert(uuid);
}

pub async fn unregister_handler(
    uuid: String,
    clients: Clients,
    sockets: Sockets,
) -> Result<impl Reply> {
    ws::remove_socket(&uuid, clients, sockets).await;
    Ok(StatusCode::OK)
}

pub async fn ws_handler(
    ws: warp::ws::Ws,
    uuid: String,
    clients: Clients,
    sockets: Sockets,
    db: Db,
) -> Result<impl Reply> {
    let client = clients.read().await.get(&uuid).cloned();
    match client {
        Some(client) => Ok(ws.on_upgrade(move |socket| {
            ws::client_connection(socket, uuid, clients, sockets, client, db)
        })),
        None => Err(warp::reject::not_found()),
    }
}

pub async fn health_handler() -> Result<impl Reply> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body("All's good here!"))
}

pub async fn index_handler() -> Result<impl Reply> {
    let index_html =
        fs::read_to_string("frontend/dist/index.html").expect("should find index.html");
    Ok(Response::builder().status(StatusCode::OK).body(index_html))
}
