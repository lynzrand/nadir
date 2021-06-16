use std::sync::Arc;
use crate::CursiveHandle;
use crate::util::DirtyCheckLock;
use crate::model::MessageGroup;
use async_trait::async_trait;

#[async_trait]
pub trait Server {
    fn new(handle: CursiveHandle, data: Arc<DirtyCheckLock<MessageGroup>>) -> Self;
    async fn serve(self);
}

mod test_server;

pub use test_server::TestServer;
