use super::*;

pub struct TestServer {
    handle: CursiveHandle,
    data: Arc<DirtyCheckLock<MessageGroup>>,
}

impl TestServer {}

#[async_trait]
impl Server for TestServer {
    fn new(handle: CursiveHandle, data: Arc<DirtyCheckLock<MessageGroup>>) -> Self {
        Self {
            handle,
            data,
        }
    }

    async fn serve(self) {
        let mut i: u64 = 1;
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;

            let mut group = self.data.write();
            group.add_message(nadir_types::model::Message {
                id: format!("aaa{}", i % 24),
                counter: None,
                tags: vec![format!("foo{}", i % 24)],
                body: format!("{}", i),
                time: Some(chrono::Utc::now()),
            });
            i = i.wrapping_add(i << 17).wrapping_add(i >> 13);

            self.handle
                .send(Box::new(|c| c.on_event(cursive::event::Event::Refresh)))
                .unwrap();
        }
    }
}
