use super::*;

pub struct GroupServer {
    pub handle: CursiveHandle,
    pub data: Arc<DirtyCheckLock<MessageGroup>>,
}

impl GroupServer {
    pub fn new(handle: CursiveHandle, data: Arc<DirtyCheckLock<MessageGroup>>) -> Self {
        Self {
            handle,
            data,
        }
    }
}

#[async_trait]
impl Server for GroupServer {
    async fn serve(self) {
    }
}
