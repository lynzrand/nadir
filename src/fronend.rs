use std::{net::SocketAddr, sync::Arc};

use futures::StreamExt;
use log::{info, warn};
use nadir_types::message::ApiMessage;
use tokio::{
    net::{TcpSocket, TcpStream},
    select,
    sync::mpsc::UnboundedReceiver,
};
use tokio_tungstenite::MaybeTlsStream;
use url::Url;

use crate::{model::group_list::GroupList, util::DirtyCheckLock};

pub async fn start_server(data: Arc<DirtyCheckLock<GroupList>>, listen: &str) {
    let port = TcpSocket::new_v6()
        .or_else(|_| TcpSocket::new_v4())
        .expect("Failed to listten on socket");
    let addr = listen.parse().expect("Malformed address");
    port.bind(addr).expect("Failed to listen");

    let (ch_send, ch_recv) = tokio::sync::mpsc::unbounded_channel();
    // tokio::spawn(batch_process_messages(ch_recv, handle, data));
    let listener = port.listen(1024).expect("failed to listen");

    info!("listening on {}", addr);

    loop {
        let (link, socket) = match listener.accept().await {
            Ok(x) => x,
            Err(e) => {
                log::error!("failed to accept link {}", e);
                continue;
            }
        };
        tokio::spawn(accept_connection(link, socket, ch_send.clone()));
    }
}

pub async fn connect_to_backends(
    backend: Url,
    stream: tokio::sync::mpsc::UnboundedSender<ApiMessage>,
) {
    let request = match hyper::Request::builder().uri(backend.as_str()).body(()) {
        Ok(req) => req,
        Err(e) => {
            log::error!("{}", e);
            return;
        }
    };
    let (conn, _) = match tokio_tungstenite::connect_async(request).await {
        Ok(conn) => conn,
        Err(e) => {
            log::error!("{}", e);
            return;
        }
    };
    match connection_loop(conn, stream).await {
        Ok(_) => {}
        Err(e) => {
            log::error!("{}", e)
        }
    };
}

async fn accept_connection(
    link: TcpStream,
    _socket: SocketAddr,
    stream: tokio::sync::mpsc::UnboundedSender<ApiMessage>,
) -> Result<(), tokio_tungstenite::tungstenite::Error> {
    info!("accepted connection to {}", _socket);
    let link = MaybeTlsStream::Plain(link);
    let conn = tokio_tungstenite::accept_async(link).await?;
    connection_loop(conn, stream).await
}

async fn connection_loop(
    mut conn: tokio_tungstenite::WebSocketStream<MaybeTlsStream<TcpStream>>,
    stream: tokio::sync::mpsc::UnboundedSender<ApiMessage>,
) -> Result<(), tokio_tungstenite::tungstenite::Error> {
    while let Some(Ok(x)) = conn.next().await {
        let t = match x.to_text() {
            Ok(text) => text,
            Err(_) => continue,
        };

        let value = match serde_json::from_str::<ApiMessage>(t) {
            Ok(v) => v,
            Err(e) => {
                log::warn!("Error receiving message: {}", e);
                continue;
            }
        };

        info!("recv message {:?}", value);

        let _ = stream.send(value);
    }
    Ok(())
}

const BATCH_TIME: std::time::Duration = std::time::Duration::from_millis(10);

async fn batch_process_messages(
    mut stream: UnboundedReceiver<ApiMessage>,
    // handle: CursiveHandle,
    data: Arc<DirtyCheckLock<GroupList>>,
) {
    let mut batch = vec![];
    loop {
        let mut paused = Box::pin(tokio::time::sleep(BATCH_TIME));
        while let Some(Some(val)) = select! {
            s = stream.recv() => Some(s),
            _ = &mut paused => None
        } {
            batch.push(val);
        }

        let mut data = data.write();
        for item in batch.drain(..) {
            match item {
                ApiMessage::Put(msg) => {
                    let group = data.get_group(&msg.group).map(|g| g.write());
                    if let Some(mut g) = group {
                        g.add_messages(msg.items.into_iter());
                    }
                }
                ApiMessage::Remove(msg) => {
                    let group = data.get_group(&msg.group).map(|g| g.write());
                    if let Some(mut g) = group {
                        g.remove_msg(msg.items.iter().map(|x| x.as_str()));
                    }
                }
                ApiMessage::PutGroup(msg) => {
                    data.add_group(Arc::new(DirtyCheckLock::new(
                        crate::model::MessageGroup::new(msg.group),
                    )));
                }
                ApiMessage::RemoveGroup(msg) => {
                    data.remove_group(msg.group);
                }
                ApiMessage::SetGroupCounter(msg) => {
                    let group = data.get_group(&msg.group).map(|g| g.write());
                    if let Some(mut g) = group {
                        g.set_counter(msg.counter);
                    }
                }
                ApiMessage::Config => {
                    warn!("Config message is not yet supported");
                }
            }
        }
        // let _ = handle.send(Box::new(|c| c.on_event(cursive::event::Event::Refresh)));
    }
}
