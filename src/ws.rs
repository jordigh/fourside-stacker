use crate::game::{play_piece, Direction};
use crate::Db;
use crate::{Client, Clients, Sockets};
use futures::{FutureExt, StreamExt};
use serde::Deserialize;
use serde_json::from_str;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};

#[derive(Deserialize, Debug)]
pub struct Play {
    pub row: usize,
    pub direction: Direction,
}

#[derive(Deserialize, Debug)]
pub struct ClientRequest {
    play: Option<Play>,
}

pub async fn remove_socket(uuid: &String, clients: Clients, sockets: Sockets) {
    let mut clients = clients.write().await;
    if let Some(client) = clients.get(uuid) {
        let user_id = client.user_id;
        clients.remove(uuid);

        let mut sockets = sockets.write().await;
        if let Some(uuids) = sockets.get_mut(&user_id) {
            uuids.remove(uuid);
        }
    }
}

pub async fn client_connection(
    ws: WebSocket,
    uuid: String,
    clients: Clients,
    sockets: Sockets,
    mut client: Client,
    db: Db,
) {
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();

    let client_rcv = UnboundedReceiverStream::new(client_rcv);
    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {}", e);
        }
    }));
    let username = client.username.clone();

    client.sender = Some(client_sender);
    clients.write().await.insert(uuid.clone(), client);

    println!("{} connected at {}", &username, uuid);

    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!(
                    "error receiving ws message for id: {}): {}",
                    uuid.clone(),
                    e
                );
                break;
            }
        };
        client_msg(uuid.clone(), msg, &clients, &sockets, &db).await;
    }

    remove_socket(&uuid, clients, sockets).await;
    println!("{} disconnected at {}", &username, uuid);
}

async fn client_msg(uuid: String, msg: Message, clients: &Clients, sockets: &Sockets, db: &Db) {
    println!("received message from {}: {:?}", uuid, msg);

    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };

    if message == "ping" || message == "ping\n" {
        return;
    }

    let client_req: ClientRequest = match from_str(message) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("error while parsing message to topics request: {}", e);
            return;
        }
    };

    let clients = clients.write().await;
    if let Some(client) = clients.get(&uuid) {
        play_piece(client, &clients, sockets, client_req.play, db).await
    } else {
        eprintln!("No player found with socket id {uuid}");
    }
}
