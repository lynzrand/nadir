//! Network interface and protocols
//!

use std::{net::SocketAddr, path::PathBuf};

use async_trait::async_trait;
use futures::{Sink, SinkExt, Stream, StreamExt, TryStream, TryStreamExt};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite, MaybeTlsStream, WebSocketStream};

pub enum Endpoint {
    Websocket(String),
    #[cfg(unix)]
    UnixDomainSocket(PathBuf),
    #[cfg(windows)]
    WindowsNamedPipe(PathBuf),
}

pub async fn connect_to<S, R>(endpoint: Endpoint) -> anyhow::Result<ConnPair<S, R>>
where
    S: Serialize + Send + 'static,
    R: for<'de> Deserialize<'de>,
{
    match endpoint {
        Endpoint::Websocket(s) => connect_ws(&s).await,
        #[cfg(unix)]
        Endpoint::UnixDomainSocket(_) => todo!(),
        #[cfg(windows)]
        Endpoint::WindowsNamedPipe(_) => todo!(),
    }
}

type ConnSink<S> = Box<dyn Sink<S, Error = anyhow::Error>>;

type ConnStream<R> =
    Box<dyn TryStream<Ok = R, Error = anyhow::Error, Item = Result<R, anyhow::Error>>>;

type ConnPair<S, R> = (ConnSink<S>, ConnStream<R>);

async fn connect_ws<S, R>(endpoint: &str) -> anyhow::Result<ConnPair<S, R>>
where
    S: Serialize + Send + 'static,
    R: for<'de> Deserialize<'de>,
{
    let (conn, _) = tokio_tungstenite::connect_async(endpoint).await?;

    from_ws_stream(conn)
}

async fn spawn_listen_ws<S, R>(
    endpoint: &str,
    cancel: tokio_util::sync::CancellationToken,
) -> anyhow::Result<Box<dyn Stream<Item = ConnPair<S, R>>>> {
    let listen_task = tokio::spawn(async move { todo!() });
    todo!();
}

fn from_ws_stream<S, R>(
    conn: WebSocketStream<MaybeTlsStream<TcpStream>>,
) -> anyhow::Result<ConnPair<S, R>>
where
    S: Serialize + Send + 'static,
    R: for<'de> Deserialize<'de>,
{
    let (send_half, recv_half) = conn.split();
    let send_half = send_half.with(|el| async move {
        let msg = tungstenite::Message::Text(serde_json::to_string(&el)?);
        Ok::<_, anyhow::Error>(msg)
    });
    let recv_half = recv_half.err_into().try_filter_map(|el| async move {
        match el {
            tungstenite::Message::Text(s) => {
                let deserialize_result = serde_json::from_str(&s)?;
                Ok(Some(deserialize_result))
            }
            _ => Ok(None),
        }
    });
    Ok((Box::new(send_half), Box::new(recv_half)))
}
