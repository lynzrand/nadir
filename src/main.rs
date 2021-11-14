use std::sync::{Arc, RwLock};

use model::{ClientMessage, ServerMessage};
use nadir_net::{ConnectionListener, Endpoint, ListenEndpoint};
use serde::{Deserialize, Serialize};
use slotmap::{DefaultKey, SecondaryMap, SlotMap};
use tokio_util::sync::CancellationToken;

mod component;
mod model;
mod net;

fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("Failed to build runtime");

    let cfg =
        serde_json::from_slice(&std::fs::read(".nadir-cfg.json").expect("Failed to read config"))
            .expect("Failed to parse config");

    rt.block_on(main_task(cfg))
}

async fn main_task(cfg: Config) {
    let cfg = Arc::new(cfg);
    let listen_task = tokio::spawn(listen_task(cfg));
    let render_task = tokio::spawn(render_task());
    let _ = tokio::join!(listen_task, render_task);
}

async fn listen_task(cfg: Arc<Config>) {
    let cancellation_token = tokio_util::sync::CancellationToken::new();
    let mut listens = SlotMap::new();
    let connections = Arc::new(RwLock::new(SecondaryMap::new()));
    for listen_point in &cfg.listen_on {
        listens.insert(listen_point.clone());
    }
    let listens = Arc::new(listens);
    for key in listens.keys() {
        tokio::spawn(manage_listen_task(
            listens.clone(),
            connections.clone(),
            key,
            cancellation_token.clone(),
        ));
    }
}

async fn manage_listen_task(
    listens: Arc<SlotMap<DefaultKey, ListenEndpoint>>,
    connections: Arc<
        RwLock<SecondaryMap<DefaultKey, ConnectionListener<ClientMessage, ServerMessage>>>,
    >,
    key: DefaultKey,
    cancel: CancellationToken,
) {
    let listen_endpoint = listens.get(key).unwrap();
    let listen_fut = nadir_net::listen_on(listen_endpoint, cancel.clone())
        .await
        .expect("Failed to listen");
    connections
        .write()
        .expect("Failed to obtain write lock")
        .insert(key, listen_fut);
}

async fn render_task() {
    loop {
        tokio::time::interval(std::time::Duration::from_millis(500))
            .tick()
            .await;
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    #[serde(default)]
    listen_on: Vec<ListenEndpoint>,

    #[serde(default)]
    connect_to: Vec<Endpoint>,
}
