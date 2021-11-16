use messages::prelude::Context;

use nadir_net::{Endpoint, ListenEndpoint};
use net::NetworkManager;
use once_cell::sync::Lazy;
use render::Renderer;
use serde::{Deserialize, Serialize};

use tokio_util::sync::CancellationToken;

mod model;
mod net;
mod render;

static ABORT: Lazy<CancellationToken> = Lazy::new(CancellationToken::new);

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let net_ctx = Context::new();
    let render_ctx = Context::new();
    let net_addr = net_ctx.address();
    let render_addr = render_ctx.address();

    let net = NetworkManager::new(net_addr.clone(), render_addr);
    let render = Renderer::new(net_addr);

    tokio::spawn(net_ctx.run(net));
    tokio::spawn(render_ctx.run(render));

    ABORT.cancelled().await;
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    #[serde(default)]
    listen_on: Vec<ListenEndpoint>,

    #[serde(default)]
    connect_to: Vec<Endpoint>,
}
