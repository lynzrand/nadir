use std::{net::SocketAddr, path::PathBuf};

use clap::Clap;
use serde::{Deserialize, Serialize};
use url::Url;

/// Start options of this program.
#[derive(Debug, Clap)]
pub struct Opt {
    /// Config path. Defaults to './nadir.toml'
    #[clap(short, long)]
    pub config: Option<PathBuf>,
}

/// Config file for this instance
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Socket addresses we listen to.
    pub websocket_listen: Vec<SocketAddr>,

    /// Websocket addresses we automatically connect to at start.
    pub websocket_connect: Vec<Url>,

    /// A certificate file to use when using TLS for connection. Supplying a value
    /// here means you would use this certificate for starting a secure
    /// WebSocket server. It won't be used when connecting to other servers.
    pub tls_cert: Option<PathBuf>,

    /// A pre-shared secret to verify connections.
    pub secret: Option<String>,
}
