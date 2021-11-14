use crate::{ConnPair, ConnectionListener};

use futures::{SinkExt, StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::TcpStream,
};
use tokio_stream::wrappers::ReceiverStream;
use tokio_tungstenite::{tungstenite, WebSocketStream};

pub async fn connect_ws<S, R>(endpoint: &str) -> anyhow::Result<ConnPair<S, R>>
where
    S: Serialize + Send + 'static,
    R: for<'de> Deserialize<'de> + Send + 'static,
{
    let (conn, _) = tokio_tungstenite::connect_async(endpoint).await?;

    from_ws_stream(conn)
}

pub async fn spawn_listen_ws<S, R>(
    endpoint: &str,
    _cancel: tokio_util::sync::CancellationToken,
) -> anyhow::Result<ConnectionListener<S, R>>
where
    S: Serialize + Send + 'static,
    R: for<'de> Deserialize<'de> + Send + 'static,
{
    let tcp = tokio::net::TcpListener::bind(endpoint).await?;
    let (send, recv) = tokio::sync::mpsc::channel(64);
    let _listen_task = tokio::spawn(async move {
        while let Ok((stream, _addr)) = tcp.accept().await {
            match accept_ws(stream).await {
                Ok(conn) => {
                    if let Err(_e) = send.send(conn).await {
                        log::error!("Failed to send connection into the stream");
                        return;
                    };
                }
                Err(e) => {
                    log::error!(
                        "Failed to initiate websocket connection with {}: {}",
                        _addr,
                        e
                    );
                }
            };
        }
    });
    Ok(Box::new(ReceiverStream::new(recv)))
}

async fn accept_ws<S, R>(stream: TcpStream) -> anyhow::Result<ConnPair<S, R>>
where
    S: Serialize + Send + 'static,
    R: for<'de> Deserialize<'de> + Send + 'static,
{
    let stream = tokio_tungstenite::accept_async(stream).await?;

    from_ws_stream(stream)
}

fn from_ws_stream<S, R, TStream>(conn: WebSocketStream<TStream>) -> anyhow::Result<ConnPair<S, R>>
where
    S: Serialize + Send + 'static,
    R: for<'de> Deserialize<'de> + Send + 'static,
    TStream: AsyncRead + AsyncWrite + Unpin + Send + Sync + 'static,
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
