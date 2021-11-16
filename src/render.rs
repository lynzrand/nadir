use std::time::Duration;

use crate::msg::GroupMsg;
use flume::Receiver;
use futures::future::{select, Either};

struct MessageState{

}

pub(crate) async fn render_task(msg: Receiver<Box<GroupMsg>>) {
    let mut rerender_tick = tokio::time::interval(Duration::from_secs(1));

    loop {
        let tick = rerender_tick.tick();
        tokio::pin!(tick);
        let next_msg = select(msg.recv_async(), tick).await;
        match next_msg {
            Either::Left((Ok(msg), _)) => match *msg {
                GroupMsg::PutGroup(_) => todo!(),
                GroupMsg::PutNotify(_, _) => todo!(),
                GroupMsg::RemoveGroup { group: _ } => todo!(),
                GroupMsg::RemoveNotify { group: _, key: _ } => todo!(),
            },
            Either::Left((Err(_), _)) => break,
            Either::Right(_) => {
                todo!("render")
            }
        }
    }
}
