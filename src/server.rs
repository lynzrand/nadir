use std::sync::Arc;
use crate::CursiveHandle;
use crate::util::DirtyCheckLock;
use crate::model::MessageGroup;
use async_trait::async_trait;

#[async_trait]
pub trait Server {
    async fn serve(self);
}

mod group_server;
pub use group_server::GroupServer;

mod test_server;
pub use test_server::TestServer;

mod maildir_local_server;
pub use maildir_local_server::MaildirLocalServer;
