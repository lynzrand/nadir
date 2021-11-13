//! Network interface and protocols
//!

use std::{net::SocketAddr, path::PathBuf};

use async_trait::async_trait;
use futures::{Sink, SinkExt, Stream, StreamExt, TryStream, TryStreamExt};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_stream::wrappers::ReceiverStream;
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
    R: for<'de> Deserialize<'de> + 'static,
{
    match endpoint {
        Endpoint::Websocket(s) => connect_ws(&s).await,
        #[cfg(unix)]
        Endpoint::UnixDomainSocket(_) => todo!(),
        #[cfg(windows)]
        Endpoint::WindowsNamedPipe(_) => todo!(),
    }
}

type ConnSink<S> = Box<dyn Sink<S, Error = anyhow::Error> + Send>;

type ConnStream<R> =
    Box<dyn TryStream<Ok = R, Error = anyhow::Error, Item = Result<R, anyhow::Error>> + Send>;

type ConnPair<S, R> = (ConnSink<S>, ConnStream<R>);

async fn connect_ws<S, R>(endpoint: &str) -> anyhow::Result<ConnPair<S, R>>
where
    S: Serialize + Send + 'static,
    R: for<'de> Deserialize<'de> + 'static,
{
    let (conn, _) = tokio_tungstenite::connect_async(endpoint).await?;

    from_ws_stream(conn)
}

async fn spawn_listen_ws<S, R>(
    endpoint: &str,
    cancel: tokio_util::sync::CancellationToken,
) -> anyhow::Result<Box<dyn Stream<Item = ConnPair<S, R>>>>
where
    S: Serialize + Send + 'static,
    R: for<'de> Deserialize<'de> + 'static,
{
    let tcp = tokio::net::TcpListener::bind(endpoint).await?;
    let (send, recv) = tokio::sync::mpsc::channel(64);
    let _listen_task = tokio::spawn(async move {
        while let Ok((stream, _addr)) = tcp.accept().await {
            match accept_ws(stream).await {
                Ok(conn) => {
                    send.send(conn).await;
                }
                Err(e) => {
                    // todo
                }
            };
        }
    });
    Ok(Box::new(ReceiverStream::new(recv)))
}

async fn accept_ws<S, R>(stream: TcpStream) -> anyhow::Result<ConnPair<S, R>>
where
    S: Serialize + Send + 'static,
    R: for<'de> Deserialize<'de>,
{
    let stream = tokio_tungstenite::accept_async(stream).await?;

    Ok(todo!())
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
