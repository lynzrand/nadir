use std::sync::{atomic::AtomicBool, Arc};

use flume::unbounded;

use nadir_net::{Endpoint, ListenEndpoint};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use tokio_util::sync::CancellationToken;

mod model;
mod msg;
mod net;
mod render;

static ALLOW_CTRL_C: AtomicBool = AtomicBool::new(true);
static ABORT_TOKEN: Lazy<CancellationToken> = Lazy::new(CancellationToken::new);

fn main() {
    ctrlc::set_handler(ctrl_c_handler).expect("Failed to set Ctrl+C handler");

    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("Failed to build runtime");

    let cfg =
        serde_yaml::from_slice(&std::fs::read(".nadir-cfg.yml").expect("Failed to read config"))
            .expect("Failed to parse config");

    rt.block_on(main_task(cfg))
}

fn ctrl_c_handler() {
    if ALLOW_CTRL_C.load(std::sync::atomic::Ordering::Relaxed) {
        ABORT_TOKEN.cancel();
    }
}

async fn main_task(cfg: Config) {
    let cfg = Arc::new(cfg);

    let (msg_tx, msg_rx) = unbounded();

    let listen_task = tokio::spawn(net::listen_task(cfg, msg_tx));
    let render_task = tokio::spawn(render::render_task(msg_rx));
    let _ = tokio::join!(listen_task, render_task);
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    #[serde(default)]
    listen_on: Vec<ListenEndpoint>,

    #[serde(default)]
    connect_to: Vec<Endpoint>,
}
