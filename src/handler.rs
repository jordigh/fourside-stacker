use std::fs;
use crate::{ws, Client, Clients, Result, Db};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{http::StatusCode, http::Response, reply::json, ws::Message, Reply};

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    username: String,
}

#[derive(Serialize, Debug)]
pub struct RegisterResponse {
    url: String,
}

#[derive(Deserialize, Debug)]
pub struct Event {
    topic: String,
    user_id: Option<i32>,
    message: String,
}

pub async fn publish_handler(body: Event, clients: Clients) -> Result<impl Reply> {
    clients
        .read()
        .await
        .iter()
        .filter(|(_, client)| match body.user_id {
            Some(v) => client.user_id == v,
            None => true,
        })
        .filter(|(_, client)| client.topics.contains(&body.topic))
        .for_each(|(_, client)| {
            if let Some(sender) = &client.sender {
                let _ = sender.send(Ok(Message::text(body.message.clone())));
            }
        });

    Ok(StatusCode::OK)
}

pub async fn register_handler(body: RegisterRequest, clients: Clients, db: Db) -> Result<impl Reply> {
    dbg!(&body);
    let username = body.username;
    let player = db.read().await.get_player(username).await;
    let uuid = Uuid::new_v4().as_simple().to_string();
    register_client(uuid.clone(), player.id, clients).await;
    Ok(json(&RegisterResponse {
        url: format!("ws://127.0.0.1:8000/ws/{}", uuid),
    }))
}

async fn register_client(id: String, user_id: i32, clients: Clients) {
    clients.write().await.insert(
        id,
        Client {
            user_id,
            topics: vec![String::from("cats")],
            sender: None,
        },
    );
}

pub async fn unregister_handler(id: String, clients: Clients) -> Result<impl Reply> {
    clients.write().await.remove(&id);
    Ok(StatusCode::OK)
}

pub async fn ws_handler(ws: warp::ws::Ws, id: String, clients: Clients) -> Result<impl Reply> {
    let client = clients.read().await.get(&id).cloned();
    match client {
        Some(c) => Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, id, clients, c))),
        None => Err(warp::reject::not_found()),
    }
}

pub async fn health_handler() -> Result<impl Reply> {
    Ok(Response::builder().status(StatusCode::OK).body("All's good here!"))
}

pub async fn index_handler() -> Result<impl Reply> {
    let index_html = fs::read_to_string("frontend/dist/index.html").expect("should find index.html");
    Ok(Response::builder().status(StatusCode::OK).body(index_html))
}
