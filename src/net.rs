//! Network interface and protocols
//!

use std::net::SocketAddr;

pub enum Endpoint {
    Tcp(SocketAddr),
    Websocket(),
    #[cfg(unix)]
    DomainSocket(),
    #[cfg(windows)]
    NamedPipe(),
}

pub trait NadirEndpoint {}
