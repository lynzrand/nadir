mod component;
mod model;
mod net;

fn main() {
    let mut rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("Failed to build runtime");
    rt.block_on(main_task())
}

async fn main_task() {}

async fn notify_task() {}

async fn render_task() {
    loop {
        tokio::time::interval(std::time::Duration::from_millis(500))
            .tick()
            .await;
    }
}
