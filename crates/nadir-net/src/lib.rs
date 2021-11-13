//! Network interface and protocols
pub mod websocket;

use std::path::PathBuf;

use futures::{Sink, Stream, TryStream};
use serde::{Deserialize, Serialize};
use tokio_util::sync::CancellationToken;

#[derive(Clone)]
pub enum Endpoint {
    #[cfg(feature = "websocket")]
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
        Endpoint::Websocket(s) => websocket::connect_ws(&s).await,
        #[cfg(unix)]
        Endpoint::UnixDomainSocket(_) => todo!(),
        #[cfg(windows)]
        Endpoint::WindowsNamedPipe(_) => todo!(),
    }
}

pub async fn listen_on<S, R>(
    endpoint: Endpoint,
    cancel: CancellationToken,
) -> anyhow::Result<ConnectionListener<S, R>>
where
    S: Serialize + Send + 'static,
    R: for<'de> Deserialize<'de> + 'static,
{
    match endpoint {
        Endpoint::Websocket(s) => websocket::spawn_listen_ws(&s, cancel).await,
        #[cfg(unix)]
        Endpoint::UnixDomainSocket(_) => todo!(),
        #[cfg(windows)]
        Endpoint::WindowsNamedPipe(_) => todo!(),
    }
}

/// The sending half of a connection, implemented as a [`Sink`].
///
/// The user should not treat an error in this sink as the signal of the connection being closed.
/// Instead, the user should rely on the corresponding [`ConnStream`].
pub type ConnSink<S> = Box<dyn Sink<S, Error = anyhow::Error> + Send>;

/// The receiving half of a connection, implemented as a [`Stream`].
///
/// The user must make sure this stream is read to the end, and drop both this value and the
/// corresponding [`ConnSink`] once it returns `Ok(None)` or `Err(_)`.
///
/// The implementor must handle non-fatal errors internally (including logging them). The
/// `Err(_)` value should only be used for fatal errors.
pub type ConnStream<R> =
    Box<dyn TryStream<Ok = R, Error = anyhow::Error, Item = Result<R, anyhow::Error>> + Send>;

/// A pair of sink and stream
pub type ConnPair<S, R> = (ConnSink<S>, ConnStream<R>);

/// The result of listening on a specific connection port. Returns a stream of connections
/// that advances every time this port accepts a connection.
pub type ConnectionListener<S, R> = Box<dyn Stream<Item = ConnPair<S, R>>>;
