use std::sync::Arc;

use crate::{
    model::{ClientMessage, ServerMessage},
    msg::{GroupMsg, SenderId},
};
use flume::Sender;
use futures::{
    stream::{iter, select_all},
    Stream, StreamExt, TryStreamExt,
};
use nadir_net::{ConnPair, ConnStream, Endpoint};
use slotmap::{SecondaryMap, SlotMap};

use crate::Config;

enum ListenControlMessage {
    ClientMesage(Box<ClientMessage>),
    ConnectionClosed,
}

pub(crate) async fn listen_task(cfg: Arc<Config>, msg_sender: Sender<Box<GroupMsg>>) {
    let mut sender_map = SlotMap::<SenderId, _>::with_key();
    let mut endpoints = SecondaryMap::new();

    // A stream of all streams
    let mut combined_stream = select_all([]);

    // initialize endpoints
    for it in &cfg.connect_to {
        let endpoint = it.clone();
        let (send, recv) = connect_to_endpoint(endpoint.clone(), msg_sender.clone()).await;
        let key = sender_map.insert(send);
        let keyed_recv = wrap_recv_stream(recv, key);
        combined_stream.extend([keyed_recv]);
        endpoints.insert(key, endpoint);
    }

    loop {
        let event = combined_stream.next().await;
    }
}

fn wrap_recv_stream(
    recv: ConnStream<ClientMessage>,
    key: SenderId,
) -> impl Stream<Item = anyhow::Result<(SenderId, ListenControlMessage)>> {
    recv.map_ok(move |v| (key, ListenControlMessage::ClientMesage(Box::new(v))))
        .chain(iter(Some(Ok((
            key,
            ListenControlMessage::ConnectionClosed,
        )))))
}

async fn listen_event_loop() {}

async fn connect_to_endpoint(
    endpoint: Endpoint,
    msg_sender: Sender<Box<GroupMsg>>,
) -> ConnPair<ServerMessage, ClientMessage> {
    todo!()
}
