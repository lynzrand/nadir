use std::sync::Arc;

use crate::msg::GroupMsg;
use flume::Sender;

use crate::Config;

pub(crate) async fn listen_task(_cfg: Arc<Config>, _msg: Sender<Box<GroupMsg>>) {}
