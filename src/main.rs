#[macro_use]
extern crate log;

use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use warp::{ws::Message, Filter, Rejection};

mod db;
mod game;
mod handler;
mod ws;

type Result<T> = std::result::Result<T, Rejection>;
type Clients = Arc<RwLock<HashMap<String, Client>>>;
type Sockets = Arc<RwLock<HashMap<i32, HashSet<String>>>>;
type Db = Arc<RwLock<db::Db>>;

#[derive(Debug, Clone)]
pub struct Client {
    pub username: String,
    pub user_id: i32,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

pub const GAME_SIZE: usize = 7;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("starting up");

    let db = match db::Db::db_init().await {
        Err(err) => panic!("{}", err),
        Ok(db) => Arc::new(RwLock::new(db)),
    };

    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));
    let sockets: Sockets = Arc::new(RwLock::new(HashMap::new()));

    let index_route = warp::path::end().and_then(handler::index_handler);
    let static_route = warp::path("static").and(warp::fs::dir("frontend/dist"));
    let health_route = warp::path!("health").and_then(handler::health_handler);

    let register = warp::path("register");
    let register_routes = register
        .and(warp::post())
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and(with_sockets(sockets.clone()))
        .and(with_db(db.clone()))
        .and_then(handler::register_handler)
        .or(register
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_clients(clients.clone()))
            .and(with_sockets(sockets.clone()))
            .and_then(handler::unregister_handler));

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param())
        .and(with_clients(clients.clone()))
        .and(with_db(db.clone()))
        .and_then(handler::ws_handler);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["POST", "GET"])
        .allow_headers(vec![
            "User-Agent",
            "Sec-Fetch-Mode",
            "Referer",
            "Origin",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
            "Content-Type",
        ]);

    let routes = index_route
        .or(static_route)
        .or(health_route)
        .or(register_routes)
        .or(ws_route)
        .with(cors);

    println!("Listening at http://127.0.0.1:8000");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

fn with_sockets(sockets: Sockets) -> impl Filter<Extract = (Sockets,), Error = Infallible> + Clone {
    warp::any().map(move || sockets.clone())
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
