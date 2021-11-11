//! Network interface and protocols
//!

use std::{net::SocketAddr, path::PathBuf};

use async_trait::async_trait;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub enum Endpoint {
    Websocket(String),
    #[cfg(unix)]
    DomainSocket(PathBuf),
    #[cfg(windows)]
    NamedPipe(PathBuf),
}

pub async fn listen_at<'de, S, R>(endpoint: Endpoint) -> anyhow::Result<Box<dyn Connection<S, R>>>
where
    S: Serialize + Send + 'static,
    R: Deserialize<'de>,
{
    match endpoint {
        Endpoint::Websocket(s) => connect_ws(s).await,
        #[cfg(unix)]
        Endpoint::DomainSocket(_) => todo!(),
        #[cfg(windows)]
        Endpoint::NamedPipe(_) => todo!(),
    }
}

async fn connect_ws<'de, S, R>(endpoint: String) -> anyhow::Result<Box<dyn Connection<S, R>>>
where
    S: Serialize + Send + 'static,
    R: Deserialize<'de>,
{
    let (stream, _) = tokio_tungstenite::connect_async(endpoint).await?;
    let conn = WebSocketConnection { stream };
    Ok(Box::new(conn))
}

#[async_trait]
pub trait Connection<S, R> {
    /// Try to receive a message from this connection.
    ///
    /// The implementor should:
    ///
    /// - Return `Ok(Some(_))` when a new message is available,
    /// - Return `Ok(None)` when the underlying connection is already closed,
    /// - Return `Err(_)` when the underlying connection encounters a problem and should be closed.
    async fn recv(&mut self) -> anyhow::Result<Option<R>>;

    /// Try to send a message into this connection.
    ///
    /// The implementor should:
    ///
    /// - Return `Ok(())` when the message is sent successfully,
    /// - Return `Err(_)` when the message cannot be sent.
    async fn send(&mut self, msg: S) -> anyhow::Result<()>;

    /// Close this connection, releasing resources.
    async fn close(&mut self);
}

pub struct WebSocketConnection {
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl WebSocketConnection {
    async fn recv_<S>(&mut self) -> anyhow::Result<Option<S>> {
        let next = match self.stream.try_next().await? {
            Some(v) => v,
            None => return Ok(None),
        };

        todo!()
    }
}

#[async_trait]
impl<'de, S, R> Connection<S, R> for WebSocketConnection
where
    S: Serialize + Send + 'static,
    R: Deserialize<'de>,
{
    async fn recv(&mut self) -> anyhow::Result<Option<R>> {
        self.recv_().await
    }

    async fn send(&mut self, msg: S) -> anyhow::Result<()> {
        todo!()
    }

    async fn close(&mut self) {
        self.stream.close(None).await;
    }
}
