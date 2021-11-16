use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

use flume::{unbounded, Receiver, Sender};
use futures::{
    channel::mpsc::UnboundedSender,
    future::{select, Either},
};
use model::{ClientMessage, ServerMessage};
use msg::{ControlMessage, GroupMsg};
use nadir_net::{ConnectionListener, Endpoint, ListenEndpoint};
use serde::{Deserialize, Serialize};
use slotmap::{DefaultKey, SecondaryMap, SlotMap};
use tokio_util::sync::CancellationToken;

mod component;
mod model;
mod msg;
mod net;

fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("Failed to build runtime");

    let cfg =
        serde_yaml::from_slice(&std::fs::read(".nadir-cfg.yml").expect("Failed to read config"))
            .expect("Failed to parse config");

    rt.block_on(main_task(cfg))
}

async fn main_task(cfg: Config) {
    let cfg = Arc::new(cfg);

    let (msg_tx, msg_rx) = unbounded();

    let listen_task = tokio::spawn(listen_task(cfg, msg_tx));
    let render_task = tokio::spawn(render_task(msg_rx));
    let _ = tokio::join!(listen_task, render_task);
}

async fn listen_task(cfg: Arc<Config>, msg: Sender<Box<GroupMsg>>) {
    let cancellation_token = tokio_util::sync::CancellationToken::new();
}

async fn render_task(msg: Receiver<Box<GroupMsg>>) {
    let mut rerender_tick = tokio::time::interval(Duration::from_secs(1));

    loop {
        let tick = rerender_tick.tick();
        tokio::pin!(tick);
        let next_msg = select(msg.recv_async(), tick).await;
        match next_msg {
            Either::Left((Ok(msg), _)) => {
                todo!("update inner state")
            }
            Either::Left((Err(_), _)) => break,
            Either::Right(_) => {
                todo!("render")
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    #[serde(default)]
    listen_on: Vec<ListenEndpoint>,

    #[serde(default)]
    connect_to: Vec<Endpoint>,
}
