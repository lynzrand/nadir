use url::Url;

/// Start options of this program.
pub struct Opt {
    /// Paths and ports to listen on. May contain websockets/HTTP server ports or
    /// unix domain sockets.
    listen: Vec<String>,
}
